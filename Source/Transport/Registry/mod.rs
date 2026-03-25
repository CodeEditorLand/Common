//! # Transport Registry
//!
//! The Transport Registry enables dynamic transport selection and management,
//! allowing components to register, select, and switch between transports at
//! runtime.
//!
//! This module provides:
//!
//! - [`TransportRegistry`] - Central registry for managing multiple transports
//! - [`TransportSelector`] - Auto-selects best transport based on context
//! - [`TransportContext`] - Context information for transport selection
//! - [`EnvironmentDetector`] - Runtime environment detection
//!
//! ## Architecture
//!
//! The registry follows a strategy pattern where multiple transport strategies
//! can be registered and selected based on environment, requirements, and
//! constraints.
//!
//! ```rust,ignore
//! use common_common::transport::{TransportRegistry, TransportStrategy, UnifiedRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut registry = TransportRegistry::new();
//!
//!     // Register available transports
//!     registry.register("grpc", GrpcTransport::new("localhost:50051")?);
//!     registry.register("ipc", IpcTransport::new("/tmp/socket")?);
//!
//!     // Auto-select based on environment
//!     let context = TransportContext::detect();
//!     let selected = registry.auto_select(&context)?;
//!     println!("Selected transport: {}", selected);
//!
//!     // Use the active transport
//!     if let Some(transport) = registry.get_active() {
//!         let response = transport.send_request(UnifiedRequest::new("ping")).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Selection Strategy
//!
//! Transports are selected based on:
//!
//! - Environment (platform, is_web, is_desktop)
//! - Requirements (streaming, cross-process, cross-network, performance)
//! - Constraints (allowed/forbidden transports, latency limits)
//!
//! The selection uses a priority-based fallback chain with automatic
//! connection testing.

use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::{
    Common::{
        EnvironmentDetector, DefaultTransportTypeDetector, TransportType, TransportTypeDetector,
    },
    TransportConfig::TransportConfig,
    TransportError::TransportError,
    TransportStrategy::{self, TransportStrategy as CommonTransportStrategy, TransportCapabilities},
    TransportMetrics,
    UnifiedRequest::UnifiedRequest,
    UnifiedResponse::UnifiedResponse,
};

/// Selection strategy for automatic transport selection.
#[derive(Debug, Clone)]
pub struct TransportSelector {
    /// Environment detector for auto-selection
    environment_detector: Box<dyn TransportTypeDetector + Send + Sync>,
    /// Priority order for fallback chain
    priority_order: Vec<TransportType>,
}

impl TransportSelector {
    /// Creates a new `TransportSelector` with default settings.
    pub fn new() -> Self {
        Self {
            environment_detector: Box::new(DefaultTransportTypeDetector),
            priority_order: Self::default_priority_order(),
        }
    }

    /// Creates a new `TransportSelector` with custom environment detector.
    pub fn with_detector(detector: Box<dyn TransportTypeDetector + Send + Sync>) -> Self {
        Self {
            environment_detector: detector,
            priority_order: Self::default_priority_order(),
        }
    }

    /// Gets the default priority order based on environment.
    fn default_priority_order() -> Vec<TransportType> {
        let mut order = Vec::new();

        // In WASM environment, prioritize WASM
        #[cfg(target_arch = "wasm32")]
        {
            order.push(TransportType::Wasm);
            order.push(TransportType::Grpc);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Desktop/server environment
            order.push(TransportType::Ipc); // Highest priority for same-machine
            order.push(TransportType::Grpc);
        }

        order
    }

    /// Selects the best transport based on context and capabilities.
    ///
    /// This method evaluates all registered transports and returns the name of
    /// the most suitable one according to the selection criteria.
    ///
    /// # Parameters
    ///
    /// * `context` - The selection context containing environment, requirements, and constraints
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Name of the selected transport
    /// * `Err(TransportError)` - If no suitable transport is found
    pub fn select_best(&self, context: &TransportContext) -> Result<String, TransportError> {
        let mut candidates = Vec::new();

        // Check all available transports
        for transport_type in self.priority_order.iter() {
            if !context.transport_available(*transport_type) {
                continue;
            }

            if !context.is_allowed(*transport_type) {
                continue;
            }

            // Calculate priority score (higher = better)
            let score = self.calculate_score(*transport_type, context);
            candidates.push((transport_type.clone(), score));
        }

        // Sort by score descending
        candidates.sort_by(|a, b| b.1.total_cmp(&a.1));

        candidates
            .first()
            .map(|(name, _)| name.as_str().to_string())
            .ok_or_else(|| {
                TransportError::not_found("No suitable transport available for current context")
            })
    }

    /// Calculates a suitability score for a transport given the context.
    ///
    /// Higher scores indicate better suitability.
    fn calculate_score(&self, transport_type: TransportType, context: &TransportContext) -> f64 {
        let mut score = 0.0;

        // Base score from priority order (higher priority = higher base score)
        if let Some(pos) = self
            .priority_order
            .iter()
            .position(|t| *t == transport_type)
        {
            score += (self.priority_order.len() - pos) as f64 * 10.0;
        }

        // Environment match bonus
        let env = context.environment();
        match (env.is_web, transport_type) {
            (true, TransportType::Wasm) => score += 50.0, // WASM is best for web
            (false, TransportType::Ipc) => score += 40.0, // IPC great for desktop
            _ => {}
        }

        // Requirements-based scoring
        let req = context.requirements();
        if req.streaming_required {
            match transport_type {
                TransportType::Grpc => score += 30.0, // gRPC excellent for streaming
                TransportType::Wasm => score += 20.0,
                TransportType::Ipc => score -= 20.0, // IPC doesn't support streaming well
            }
        }

        if req.cross_network {
            match transport_type {
                TransportType::Grpc => score += 50.0, // gRPC is network-focused
                TransportType::Ipc => score -= 50.0, // IPC only local
                TransportType::Wasm => score += 10.0,
            }
        }

        // Performance requirements
        score += match req.performance {
            PerformanceLevel::Critical => match transport_type {
                TransportType::Ipc => 40.0,
                TransportType::Grpc => 20.0,
                TransportType::Wasm => 0.0,
            },
            PerformanceLevel::High => match transport_type {
                TransportType::Ipc => 30.0,
                TransportType::Grpc => 20.0,
                TransportType::Wasm => 10.0,
            },
            PerformanceLevel::Medium => 10.0,
            PerformanceLevel::Low => 0.0,
        };

        // Latency constraint
        if let Some(max_latency) = req.max_latency_ms {
            let estimated_latency = self.estimate_latency_ms(transport_type);
            if estimated_latency <= max_latency {
                score += 20.0; // Meets latency requirement
            } else {
                score -= 30.0; // Exceeds latency budget
            }
        }

        score
    }

    /// Estimates typical latency for a transport type in milliseconds.
    fn estimate_latency_ms(&self, transport_type: TransportType) -> u64 {
        match transport_type {
            TransportType::Ipc => 1,    // < 0.1ms typically, round to 1
            TransportType::Grpc => 5,   // 1-10ms typical
            TransportType::Wasm => 20,  // 5-50ms typical
            TransportType::Unknown => u64::MAX,
        }
    }
}

impl Default for TransportSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// Context information for transport selection.
///
/// This struct encapsulates all the information needed to make an intelligent
/// transport selection decision.
#[derive(Debug, Clone)]
pub struct TransportContext {
    environment: EnvironmentInfo,
    requirements: TransportRequirements,
    constraints: TransportConstraints,
    available_transports: HashSet<TransportType>,
}

impl TransportContext {
    /// Creates a new transport selection context.
    pub fn new(
        environment: EnvironmentInfo,
        requirements: TransportRequirements,
        constraints: TransportConstraints,
    ) -> Self {
        let available_transports = DefaultTransportTypeDetector::list_available_transports()
            .into_iter()
            .collect();

        Self {
            environment,
            requirements,
            constraints,
            available_transports,
        }
    }

    /// Detects the current environment and creates a context with default requirements.
    ///
    /// This is the primary method for creating a context when you want automatic
    /// detection of the environment.
    pub fn detect() -> Self {
        let detector = DefaultTransportTypeDetector;
        let environment = detector.detect_environment();
        let requirements = TransportRequirements::default();
        let constraints = TransportConstraints::default();

        Self::new(environment, requirements, constraints)
    }

    /// Gets the environment information.
    pub fn environment(&self) -> &EnvironmentInfo {
        &self.environment
    }

    /// Gets the transport requirements.
    pub fn requirements(&self) -> &TransportRequirements {
        &self.requirements
    }

    /// Gets the transport constraints.
    pub fn constraints(&self) -> &TransportConstraints {
        &self.constraints
    }

    /// Checks if a transport type is available in this environment.
    pub fn transport_available(&self, transport_type: TransportType) -> bool {
        self.available_transports.contains(&transport_type)
    }

    /// Checks if a transport type is allowed by constraints.
    pub fn is_allowed(&self, transport_type: TransportType) -> bool {
        // Check forbidden list
        if self.constraints.forbidden_transports.contains(&transport_type) {
            return false;
        }

        // Check allowed list (empty means all allowed)
        if self.constraints.allowed_transports.is_empty() {
            true
        } else {
            self.constraints.allowed_transports.contains(&transport_type)
        }
    }

    /// Sets custom available transports (for testing or override).
    pub fn with_available_transports(mut self, transports: Vec<TransportType>) -> Self {
        self.available_transports = transports.into_iter().collect();
        self
    }
}

/// Environment information for transport selection.
#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    /// Operating system platform
    pub platform: Platform,
    /// Whether running in a web browser
    pub is_web: bool,
    /// Whether running as a desktop application
    pub is_desktop: bool,
    /// Browser capability information (if in browser)
    pub browser_capabilities: Option<BrowserCapabilities>,
}

impl EnvironmentInfo {
    /// Creates a new environment info.
    pub fn new(
        platform: Platform,
        is_web: bool,
        is_desktop: bool,
        browser_capabilities: Option<BrowserCapabilities>,
    ) -> Self {
        Self {
            platform,
            is_web,
            is_desktop,
            browser_capabilities,
        }
    }
}

/// Platform enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Browser,
    Mobile,
    Unknown,
}

impl Platform {
    /// Gets the current platform.
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "macos")]
        return Self::MacOS;
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos"), not(target_os = "linux")))]
        return Self::Unknown;
    }
}

/// Browser capabilities detection.
#[derive(Debug, Clone)]
pub struct BrowserCapabilities {
    pub wasm_supported: bool,
    pub web_worker_supported: bool,
    pub web_socket_supported: bool,
    pub shared_array_buffer_supported: bool,
}

impl Default for BrowserCapabilities {
    fn default() -> Self {
        Self {
            wasm_supported: cfg!(target_arch = "wasm32"),
            web_worker_supported: false, // TODO: Detect properly
            web_socket_supported: false,
            shared_array_buffer_supported: false,
        }
    }
}

/// Transport requirements for selection.
#[derive(Debug, Clone)]
pub struct TransportRequirements {
    /// Whether bidirectional streaming is required
    pub streaming_required: bool,
    /// Whether cross-process communication is needed
    pub cross_process: bool,
    /// Whether cross-network communication is needed
    pub cross_network: bool,
    /// Performance requirement level
    pub performance: PerformanceLevel,
    /// Reliability requirement level
    pub reliability: ReliabilityLevel,
    /// Maximum acceptable latency in milliseconds (optional)
    pub max_latency_ms: Option<u64>,
}

impl Default for TransportRequirements {
    fn default() -> Self {
        Self {
            streaming_required: false,
            cross_process: false,
            cross_network: false,
            performance: PerformanceLevel::Medium,
            reliability: ReliabilityLevel::Medium,
            max_latency_ms: None,
        }
    }
}

/// Performance requirement level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PerformanceLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Reliability requirement level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReliabilityLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Transport selection constraints.
#[derive(Debug, Clone)]
pub struct TransportConstraints {
    /// Allowed transport types (empty means all allowed)
    pub allowed_transports: Vec<TransportType>,
    /// Forbidden transport types
    pub forbidden_transports: Vec<TransportType>,
    /// Maximum allowed latency in milliseconds
    pub max_latency_ms: Option<u64>,
    /// Maximum allowed bandwidth in bytes per second
    pub max_bandwidth_bps: Option<u64>,
}

impl Default for TransportConstraints {
    fn default() -> Self {
        Self {
            allowed_transports: Vec::new(),
            forbidden_transports: Vec::new(),
            max_latency_ms: None,
            max_bandwidth_bps: None,
        }
    }
}

/// Central registry for managing transport strategies.
///
/// The registry allows registration, selection, and management of multiple
/// transport implementations. It supports both explicit selection and
/// automatic selection based on context.
#[derive(Debug, Clone)]
pub struct TransportRegistry {
    /// Registered transports (name -> TransportStrategy)
    transports: HashMap<String, Arc<RwLock<dyn CommonTransportStrategy>>>,
    /// Currently active transport name
    active: Option<String>,
    /// Transport selector for auto-selection
    selector: TransportSelector,
}

impl TransportRegistry {
    /// Creates a new, empty transport registry.
    pub fn new() -> Self {
        Self {
            transports: HashMap::new(),
            active: None,
            selector: TransportSelector::new(),
        }
    }

    /// Creates a new registry with a custom selector.
    pub fn with_selector(selector: TransportSelector) -> Self {
        Self {
            transports: HashMap::new(),
            active: None,
            selector,
        }
    }

    /// Registers a new transport with the registry.
    ///
    /// # Parameters
    ///
    /// * `name` - Unique name for this transport
    /// * `transport` - The transport strategy instance (thread-safe)
    ///
    /// # Notes
    ///
    /// - If a transport with the same name already exists, it will be replaced
    /// - The transport is wrapped in `Arc<RwLock<...>>` for thread-safe access
    pub fn register(&mut self, name: String, transport: Arc<dyn CommonTransportStrategy>) {
        log::info!("Registering transport: {}", name);
        self.transports.insert(name, transport);
    }

    /// Unregisters a transport from the registry.
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the transport to unregister
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Transport was successfully unregistered
    /// * `Err(TransportError)` - Transport not found or error during disconnection
    pub fn unregister(&mut self, name: &str) -> Result<(), TransportError> {
        let transport_opt = self.transports.remove(name);

        if let Some(transport) = transport_opt {
            // Disconnect the transport if it's connected
            let is_connected = {
                let transport_ref = transport.read().map_err(|_| {
                    TransportError::internal("Failed to acquire read lock on transport")
                })?;
                transport_ref.is_connected()
            };

            if is_connected {
                let mut transport_mut = transport.write().map_err(|_| {
                    TransportError::internal("Failed to acquire write lock on transport")
                })?;
                transport_mut.disconnect().await?;
            }

            log::info!("Unregistered transport: {}", name);
            Ok(())
        } else {
            Err(TransportError::not_found(format!("Transport '{}' not found", name)))
        }
    }

    /// Selects a transport by name as the active transport.
    ///
    /// # Parameters
    ///
    /// * `name` - Name of the transport to activate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Transport successfully selected
    /// * `Err(TransportError)` - Transport not found or connection failed
    pub fn select(&mut self, name: &str) -> Result<(), TransportError> {
        let transport = self
            .transports
            .get(name)
            .ok_or_else(|| TransportError::not_found(format!("Transport '{}' not found", name)))?;

        log::info!("Selecting transport: {}", name);

        // Connect to the transport if not already connected
        {
            let mut transport_mut = transport.write().map_err(|_| {
                TransportError::internal("Failed to acquire write lock on transport")
            })?;

            if !transport_mut.is_connected() {
                transport_mut.connect().await?;
            }
        }

        self.active = Some(name.to_string());
        Ok(())
    }

    /// Automatically selects the best transport based on the provided context.
    ///
    /// This method uses the registered selector to choose the most appropriate
    /// transport, then connects to it and makes it active.
    ///
    /// # Parameters
    ///
    /// * `context` - Selection context (environment, requirements, constraints)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Name of the selected transport
    /// * `Err(TransportError)` - No suitable transport found or connection failed
    pub fn auto_select(&mut self, context: &TransportContext) -> Result<String, TransportError> {
        // Find the best transport
        let selected_name = self.selector.select_best(context)?;

        // Connect to it
        self.select(&selected_name)?;

        Ok(selected_name)
    }

    /// Gets the currently active transport, if any.
    ///
    /// # Returns
    ///
    /// * `Some(Arc<RwLock<...>>)` - Active transport with read/write lock
    /// * `None` - No active transport selected
    pub fn get_active(&self) -> Option<Arc<RwLock<dyn CommonTransportStrategy>>> {
        self.active
            .as_ref()
            .and_then(|name| self.transports.get(name))
            .cloned()
    }

    /// Gets a specific transport by name.
    ///
    /// # Parameters
    ///
    /// * `name` - Transport name
    ///
    /// # Returns
    ///
    /// * `Some(Arc<RwLock<...>>)` - Transport if found
    /// * `None` - Transport not registered
    pub fn get(&self, name: &str) -> Option<Arc<RwLock<dyn CommonTransportStrategy>>> {
        self.transports.get(name).cloned()
    }

    /// Lists all registered transport names.
    pub fn list(&self) -> Vec<String> {
        self.transports.keys().cloned().collect()
    }

    /// Checks if a transport with the given name is registered.
    pub fn has(&self, name: &str) -> bool {
        self.transports.contains_key(name)
    }

    /// Gets metrics for all registered transports.
    ///
    /// # Returns
    ///
    /// A map from transport name to its current metrics.
    pub fn get_all_metrics(&self) -> Result<HashMap<String, TransportMetrics>, TransportError> {
        let mut metrics = HashMap::new();

        for (name, transport) in &self.transports {
            let transport_ref = transport.read().map_err(|_| {
                TransportError::internal(format!("Failed to read lock transport '{}'", name))
            })?;
            metrics.insert(name.clone(), transport_ref.metrics());
        }

        Ok(metrics)
    }

    /// Gets health status (connected/not connected) for all transports.
    ///
    /// # Returns
    ///
    /// A map from transport name to its connected status.
    pub fn get_health_status(&self) -> Result<HashMap<String, bool>, TransportError> {
        let mut status = HashMap::new();

        for (name, transport) in &self.transports {
            let transport_ref = transport.read().map_err(|_| {
                TransportError::internal(format!("Failed to read lock transport '{}'", name))
            })?;
            status.insert(name.clone(), transport_ref.is_connected());
        }

        Ok(status)
    }

    /// Gets the name of the currently active transport.
    pub fn active_name(&self) -> Option<&str> {
        self.active.as_deref()
    }

    /// Sets the selector to use for auto-selection.
    pub fn set_selector(&mut self, selector: TransportSelector) {
        self.selector = selector;
    }

    /// Waits for a transport to be ready (connected) with timeout.
    ///
    /// # Parameters
    ///
    /// * `name` - Transport name
    /// * `timeout` - Maximum time to wait
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Transport is connected
    /// * `Err(TransportError)` - Timeout or connection failure
    pub async fn wait_for_ready(
        &self,
        name: &str,
        timeout: Duration,
    ) -> Result<(), TransportError> {
        use tokio::time::{timeout as tokio_timeout, Instant};

        let start = Instant::now();
        let transport = self
            .get(name)
            .ok_or_else(|| TransportError::not_found(format!("Transport '{}' not found", name)))?;

        loop {
            let connected = {
                let transport_ref = transport.read().map_err(|_| {
                    TransportError::internal("Failed to acquire read lock on transport")
                })?;
                transport_ref.is_connected()
            };

            if connected {
                return Ok(());
            }

            if start.elapsed() >= timeout {
                return Err(TransportError::timeout("Transport did not become ready within timeout"));
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Implement TransportTypeDetector for DefaultTransportTypeDetector to provide environment
impl DefaultTransportTypeDetector {
    /// Detects the current environment information.
    pub fn detect_environment() -> EnvironmentInfo {
        let platform = Platform::current();
        let is_desktop = !cfg!(target_arch = "wasm32");

        // Basic environment
        let env = EnvironmentInfo {
            platform,
            is_web: false,
            is_desktop,
            browser_capabilities: None,
        };

        // In WASM, it's a web environment
        #[cfg(target_arch = "wasm32")]
        {
            EnvironmentInfo {
                is_web: true,
                is_desktop: false,
                browser_capabilities: Some(BrowserCapabilities::default()),
                ..env
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            env
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_selector_creation() {
        let selector = TransportSelector::new();
        assert!(!selector.priority_order.is_empty());
    }

    #[test]
    fn test_transport_context_creation() {
        let env = EnvironmentInfo::new(Platform::Linux, false, true, None);
        let req = TransportRequirements::default();
        let constraints = TransportConstraints::default();

        let context = TransportContext::new(env, req, constraints);
        assert!(!context.transport_available(TransportType::Grpc)); // Not available without real detector
    }

    #[test]
    fn test_transport_registry_creation() {
        let registry = TransportRegistry::new();
        assert!(registry.list().is_empty());
        assert!(registry.active_name().is_none());
    }

    #[tokio::test]
    async fn test_registry_register_unregister() {
        let mut registry = TransportRegistry::new();

        // Create a mock transport for testing
        let mock_transport = Arc::new(RwLock::new(MockTransport::new()));
        registry.register("mock".to_string(), mock_transport);

        assert!(registry.has("mock"));
        assert_eq!(registry.list().len(), 1);

        // Unregister
        registry.unregister("mock").unwrap();
        assert!(!registry.has("mock"));
    }

    /// Mock transport for testing
    #[derive(Debug, Clone)]
    struct MockTransport;

    impl MockTransport {
        fn new() -> Self {
            Self
        }
    }

    #[async_trait]
    impl CommonTransportStrategy for MockTransport {
        async fn connect(&mut self) -> Result<(), TransportError> {
            Ok(())
        }

        async fn disconnect(&mut self) -> Result<(), TransportError> {
            Ok(())
        }

        async fn send_request(
            &mut self,
            request: UnifiedRequest,
        ) -> Result<UnifiedResponse, TransportError> {
            Ok(UnifiedResponse::success(
                request.correlation_id.clone(),
                Vec::new(),
            ))
        }

        async fn send_notification(
            &mut self,
            notification: UnifiedRequest,
        ) -> Result<(), TransportError> {
            Ok(())
        }

        fn stream_events(
            &self,
        ) -> std::result::Result<
            futures::stream::BoxStream<'static, UnifiedResponse>,
            TransportError,
        > {
            Err(TransportError::not_supported("Streaming not supported"))
        }

        fn is_connected(&self) -> bool {
            true
        }

        fn latency_ms(&self) -> u64 {
            0
        }

        fn transport_type(&self) -> TransportType {
            TransportType::Grpc
        }

        fn config(&self) -> &TransportConfig {
            static CONFIG: TransportConfig = TransportConfig::default();
            &CONFIG
        }

        fn capabilities(&self) -> TransportCapabilities {
            TransportCapabilities::default()
        }

        fn metrics(&self) -> TransportMetrics {
            TransportMetrics::new()
        }

        fn supports_streaming(&self) -> bool {
            false
        }
    }
}
