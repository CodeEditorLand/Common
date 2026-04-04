#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # Transport Registry
//!
//! The Transport Registry enables dynamic transport selection and management,
//! allowing components to register, select, and switch between transports at
//! runtime.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use super::{
	Common::{DefaultTransportTypeDetector, TransportType, TransportTypeDetector},
	TransportError::TransportError,
	TransportStrategy::{TransportMetrics, TransportStrategy as CommonTransportStrategy},
};

/// Selection strategy for automatic transport selection.
pub struct TransportSelector {
	/// Environment detector for auto-selection
	#[allow(dead_code)]
	EnvironmentDetector: Box<dyn TransportTypeDetector + Send + Sync>,
	/// Priority order for fallback chain
	PriorityOrder: Vec<TransportType>,
}

impl TransportSelector {
	/// Creates a new `TransportSelector` with default settings.
	pub fn New() -> Self {
		Self {
			EnvironmentDetector: Box::new(DefaultTransportTypeDetector),
			PriorityOrder: Self::DefaultPriorityOrder(),
		}
	}

	/// Creates a new `TransportSelector` with custom environment detector.
	pub fn WithDetector(Detector: Box<dyn TransportTypeDetector + Send + Sync>) -> Self {
		Self {
			EnvironmentDetector: Detector,
			PriorityOrder: Self::DefaultPriorityOrder(),
		}
	}

	/// Gets the default priority order based on environment.
	fn DefaultPriorityOrder() -> Vec<TransportType> {
		let mut Order = Vec::new();

		#[cfg(target_arch = "wasm32")]
		{
			Order.push(TransportType::Wasm);
			Order.push(TransportType::Grpc);
		}

		#[cfg(not(target_arch = "wasm32"))]
		{
			Order.push(TransportType::Ipc);
			Order.push(TransportType::Grpc);
		}

		Order
	}

	/// Selects the best transport based on context and capabilities.
	pub fn SelectBest(&self, Context: &TransportContext) -> Result<String, TransportError> {
		let mut Candidates = Vec::new();

		for TransportKind in self.PriorityOrder.iter() {
			if !Context.TransportAvailable(*TransportKind) {
				continue;
			}

			if !Context.IsAllowed(*TransportKind) {
				continue;
			}

			let Score = self.CalculateScore(*TransportKind, Context);
			Candidates.push((*TransportKind, Score));
		}

		Candidates.sort_by(|Left, Right| Right.1.total_cmp(&Left.1));

		Candidates
			.first()
			.map(|(TransportKind, _)| TransportKind.AsString().to_string())
			.ok_or_else(|| {
				TransportError::NotFound("No suitable transport available for current context")
			})
	}

	/// Calculates a suitability score for a transport given the context.
	fn CalculateScore(
		&self,
		TransportKind: TransportType,
		Context: &TransportContext,
	) -> f64 {
		let mut Score = 0.0;

		if let Some(Position) = self
			.PriorityOrder
			.iter()
			.position(|Kind| *Kind == TransportKind)
		{
			Score += (self.PriorityOrder.len() - Position) as f64 * 10.0;
		}

		let Environment = Context.Environment();
		match (Environment.IsWeb, TransportKind) {
			(true, TransportType::Wasm) => Score += 50.0,
			(false, TransportType::Ipc) => Score += 40.0,
			_ => {}
		}

		let Requirements = Context.Requirements();
		if Requirements.StreamingRequired {
			match TransportKind {
				TransportType::Grpc => Score += 30.0,
				TransportType::Wasm => Score += 20.0,
				TransportType::Ipc => Score -= 20.0,
				TransportType::Unknown => {}
			}
		}

		if Requirements.CrossNetwork {
			match TransportKind {
				TransportType::Grpc => Score += 50.0,
				TransportType::Ipc => Score -= 50.0,
				TransportType::Wasm => Score += 10.0,
				TransportType::Unknown => {}
			}
		}

		Score += match Requirements.Performance {
			PerformanceLevel::Critical => match TransportKind {
				TransportType::Ipc => 40.0,
				TransportType::Grpc => 20.0,
				TransportType::Wasm => 0.0,
				TransportType::Unknown => 0.0,
			},
			PerformanceLevel::High => match TransportKind {
				TransportType::Ipc => 30.0,
				TransportType::Grpc => 20.0,
				TransportType::Wasm => 10.0,
				TransportType::Unknown => 0.0,
			},
			PerformanceLevel::Medium => 10.0,
			PerformanceLevel::Low => 0.0,
		};

		if let Some(MaximumLatency) = Requirements.MaximumLatencyMilliseconds {
			let EstimatedLatency = self.EstimateLatencyMilliseconds(TransportKind);
			if EstimatedLatency <= MaximumLatency {
				Score += 20.0;
			} else {
				Score -= 30.0;
			}
		}

		Score
	}

	/// Estimates typical latency for a transport type in milliseconds.
	fn EstimateLatencyMilliseconds(&self, TransportKind: TransportType) -> u64 {
		match TransportKind {
			TransportType::Ipc => 1,
			TransportType::Grpc => 5,
			TransportType::Wasm => 20,
			TransportType::Unknown => u64::MAX,
		}
	}
}

impl Default for TransportSelector {
	fn default() -> Self {
		Self::New()
	}
}

/// Context information for transport selection.
#[derive(Debug, Clone)]
pub struct TransportContext {
	EnvironmentInfo: EnvironmentInfo,
	RequirementsInfo: TransportRequirements,
	ConstraintsInfo: TransportConstraints,
	AvailableTransports: HashSet<TransportType>,
}

impl TransportContext {
	/// Creates a new transport selection context.
	pub fn New(
		EnvironmentInfo: EnvironmentInfo,
		RequirementsInfo: TransportRequirements,
		ConstraintsInfo: TransportConstraints,
	) -> Self {
		let AvailableTransports =
			DefaultTransportTypeDetector::list_available_transports()
				.into_iter()
				.collect();

		Self {
			EnvironmentInfo,
			RequirementsInfo,
			ConstraintsInfo,
			AvailableTransports,
		}
	}

	/// Detects the current environment and creates a context with default requirements.
	pub fn Detect() -> Self {
		let EnvironmentInfo = DefaultTransportTypeDetector::DetectEnvironment();
		let RequirementsInfo = TransportRequirements::default();
		let ConstraintsInfo = TransportConstraints::default();

		Self::New(EnvironmentInfo, RequirementsInfo, ConstraintsInfo)
	}

	/// Gets the environment information.
	pub fn Environment(&self) -> &EnvironmentInfo {
		&self.EnvironmentInfo
	}

	/// Gets the transport requirements.
	pub fn Requirements(&self) -> &TransportRequirements {
		&self.RequirementsInfo
	}

	/// Gets the transport constraints.
	pub fn Constraints(&self) -> &TransportConstraints {
		&self.ConstraintsInfo
	}

	/// Checks if a transport type is available in this environment.
	pub fn TransportAvailable(&self, TransportKind: TransportType) -> bool {
		self.AvailableTransports.contains(&TransportKind)
	}

	/// Checks if a transport type is allowed by constraints.
	pub fn IsAllowed(&self, TransportKind: TransportType) -> bool {
		if self.ConstraintsInfo.ForbiddenTransports.contains(&TransportKind) {
			return false;
		}

		if self.ConstraintsInfo.AllowedTransports.is_empty() {
			true
		} else {
			self.ConstraintsInfo.AllowedTransports.contains(&TransportKind)
		}
	}

	/// Sets custom available transports (for testing or override).
	pub fn WithAvailableTransports(mut self, Transports: Vec<TransportType>) -> Self {
		self.AvailableTransports = Transports.into_iter().collect();
		self
	}
}

/// Environment information for transport selection.
#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
	/// Operating system platform
	pub Platform: Platform,
	/// Whether running in a web browser
	pub IsWeb: bool,
	/// Whether running as a desktop application
	pub IsDesktop: bool,
	/// Browser capability information (if in browser)
	pub BrowserCapabilities: Option<BrowserCapabilities>,
}

impl EnvironmentInfo {
	/// Creates a new environment info.
	pub fn New(
		Platform: Platform,
		IsWeb: bool,
		IsDesktop: bool,
		BrowserCapabilities: Option<BrowserCapabilities>,
	) -> Self {
		Self {
			Platform,
			IsWeb,
			IsDesktop,
			BrowserCapabilities,
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
	pub fn Current() -> Self {
		#[cfg(target_os = "windows")]
		return Self::Windows;
		#[cfg(target_os = "macos")]
		return Self::MacOS;
		#[cfg(target_os = "linux")]
		return Self::Linux;
		#[cfg(all(
			not(target_os = "windows"),
			not(target_os = "macos"),
			not(target_os = "linux")
		))]
		return Self::Unknown;
	}
}

/// Browser capabilities detection.
#[derive(Debug, Clone)]
pub struct BrowserCapabilities {
	pub WasmSupported: bool,
	pub WebWorkerSupported: bool,
	pub WebSocketSupported: bool,
	pub SharedArrayBufferSupported: bool,
}

impl Default for BrowserCapabilities {
	fn default() -> Self {
		Self {
			WasmSupported: cfg!(target_arch = "wasm32"),
			WebWorkerSupported: false,
			WebSocketSupported: false,
			SharedArrayBufferSupported: false,
		}
	}
}

/// Transport requirements for selection.
#[derive(Debug, Clone)]
pub struct TransportRequirements {
	/// Whether bidirectional streaming is required
	pub StreamingRequired: bool,
	/// Whether cross-process communication is needed
	pub CrossProcess: bool,
	/// Whether cross-network communication is needed
	pub CrossNetwork: bool,
	/// Performance requirement level
	pub Performance: PerformanceLevel,
	/// Reliability requirement level
	pub Reliability: ReliabilityLevel,
	/// Maximum acceptable latency in milliseconds (optional)
	pub MaximumLatencyMilliseconds: Option<u64>,
}

impl Default for TransportRequirements {
	fn default() -> Self {
		Self {
			StreamingRequired: false,
			CrossProcess: false,
			CrossNetwork: false,
			Performance: PerformanceLevel::Medium,
			Reliability: ReliabilityLevel::Medium,
			MaximumLatencyMilliseconds: None,
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
	pub AllowedTransports: Vec<TransportType>,
	/// Forbidden transport types
	pub ForbiddenTransports: Vec<TransportType>,
	/// Maximum allowed latency in milliseconds
	pub MaximumLatencyMilliseconds: Option<u64>,
	/// Maximum allowed bandwidth in bytes per second
	pub MaximumBandwidthBytesPerSecond: Option<u64>,
}

impl Default for TransportConstraints {
	fn default() -> Self {
		Self {
			AllowedTransports: Vec::new(),
			ForbiddenTransports: Vec::new(),
			MaximumLatencyMilliseconds: None,
			MaximumBandwidthBytesPerSecond: None,
		}
	}
}

/// Central registry for managing transport strategies.
pub struct TransportRegistry {
	/// Registered transports (name -> Arc<dyn CommonTransportStrategy>)
	Transports: HashMap<String, Arc<dyn CommonTransportStrategy>>,
	/// Currently active transport name
	Active: Option<String>,
	/// Transport selector for auto-selection
	Selector: TransportSelector,
}

impl TransportRegistry {
	/// Creates a new, empty transport registry.
	pub fn New() -> Self {
		Self {
			Transports: HashMap::new(),
			Active: None,
			Selector: TransportSelector::New(),
		}
	}

	/// Creates a new registry with a custom selector.
	pub fn WithSelector(Selector: TransportSelector) -> Self {
		Self {
			Transports: HashMap::new(),
			Active: None,
			Selector,
		}
	}

	/// Registers a new transport with the registry.
	pub fn Register(&mut self, Name: String, Transport: Arc<dyn CommonTransportStrategy>) {
		log::info!("Registering transport: {}", Name);
		self.Transports.insert(Name, Transport);
	}

	/// Unregisters a transport from the registry.
	pub async fn Unregister(&mut self, Name: &str) -> Result<(), TransportError> {
		let TransportOption = self.Transports.remove(Name);

		if let Some(_Transport) = TransportOption {
			// Arc<dyn CommonTransportStrategy> does not expose mutable disconnect here;
			// callers should disconnect before unregistering if needed.
			log::info!("Unregistered transport: {}", Name);
			Ok(())
		} else {
			Err(TransportError::NotFound(format!(
				"Transport '{}' not found",
				Name
			)))
		}
	}

	/// Selects a transport by name as the active transport.
	pub async fn Select(&mut self, Name: &str) -> Result<(), TransportError> {
		if !self.Transports.contains_key(Name) {
			return Err(TransportError::NotFound(format!(
				"Transport '{}' not found",
				Name
			)));
		}

		log::info!("Selecting transport: {}", Name);
		self.Active = Some(Name.to_string());
		Ok(())
	}

	/// Automatically selects the best transport based on the provided context.
	pub async fn AutoSelect(
		&mut self,
		Context: &TransportContext,
	) -> Result<String, TransportError> {
		let SelectedName = self.Selector.SelectBest(Context)?;
		self.Select(&SelectedName).await?;
		Ok(SelectedName)
	}

	/// Gets the currently active transport, if any.
	pub fn GetActive(&self) -> Option<Arc<dyn CommonTransportStrategy>> {
		self.Active
			.as_ref()
			.and_then(|Name| self.Transports.get(Name))
			.cloned()
	}

	/// Gets a specific transport by name.
	pub fn Get(&self, Name: &str) -> Option<Arc<dyn CommonTransportStrategy>> {
		self.Transports.get(Name).cloned()
	}

	/// Lists all registered transport names.
	pub fn List(&self) -> Vec<String> {
		self.Transports.keys().cloned().collect()
	}

	/// Checks if a transport with the given name is registered.
	pub fn Has(&self, Name: &str) -> bool {
		self.Transports.contains_key(Name)
	}

	/// Gets metrics for all registered transports.
	pub fn GetAllMetrics(&self) -> HashMap<String, TransportMetrics> {
		let mut Metrics = HashMap::new();
		for (Name, Transport) in &self.Transports {
			Metrics.insert(Name.clone(), Transport.Metrics());
		}
		Metrics
	}

	/// Gets health status (connected/not connected) for all transports.
	pub fn GetHealthStatus(&self) -> HashMap<String, bool> {
		let mut Status = HashMap::new();
		for (Name, Transport) in &self.Transports {
			Status.insert(Name.clone(), Transport.IsConnected());
		}
		Status
	}

	/// Gets the name of the currently active transport.
	pub fn ActiveName(&self) -> Option<&str> {
		self.Active.as_deref()
	}

	/// Sets the selector to use for auto-selection.
	pub fn SetSelector(&mut self, Selector: TransportSelector) {
		self.Selector = Selector;
	}

	/// Waits for a transport to be ready (connected) with timeout.
	pub async fn WaitForReady(
		&self,
		Name: &str,
		Timeout: Duration,
	) -> Result<(), TransportError> {
		use tokio::time::Instant;

		let Start = Instant::now();
		let Transport = self
			.Get(Name)
			.ok_or_else(|| TransportError::NotFound(format!("Transport '{}' not found", Name)))?;

		loop {
			if Transport.IsConnected() {
				return Ok(());
			}

			if Start.elapsed() >= Timeout {
				return Err(TransportError::Timeout(
					"Transport did not become ready within timeout",
				));
			}

			tokio::time::sleep(Duration::from_millis(50)).await;
		}
	}
}

impl Default for TransportRegistry {
	fn default() -> Self {
		Self::New()
	}
}

/// Provides environment detection for DefaultTransportTypeDetector.
impl DefaultTransportTypeDetector {
	/// Detects the current environment information.
	pub fn DetectEnvironment() -> EnvironmentInfo {
		let CurrentPlatform = Platform::Current();
		let IsDesktop = !cfg!(target_arch = "wasm32");

		let Environment = EnvironmentInfo {
			Platform: CurrentPlatform,
			IsWeb: false,
			IsDesktop,
			BrowserCapabilities: None,
		};

		#[cfg(target_arch = "wasm32")]
		{
			EnvironmentInfo {
				IsWeb: true,
				IsDesktop: false,
				BrowserCapabilities: Some(BrowserCapabilities::default()),
				..Environment
			}
		}

		#[cfg(not(target_arch = "wasm32"))]
		{
			Environment
		}
	}
}

#[cfg(test)]
mod tests {
	use async_trait::async_trait;

	use super::{
		super::{
			TransportConfig::TransportConfig,
			TransportError::TransportError,
			TransportStrategy::{TransportCapabilities, TransportMetrics},
			UnifiedRequest::UnifiedRequest,
			UnifiedResponse::UnifiedResponse,
		},
		*,
	};

	#[test]
	fn TestTransportSelectorCreation() {
		let Selector = TransportSelector::New();
		assert!(!Selector.PriorityOrder.is_empty());
	}

	#[test]
	fn TestTransportContextCreation() {
		let Environment = EnvironmentInfo::New(Platform::Linux, false, true, None);
		let Requirements = TransportRequirements::default();
		let Constraints = TransportConstraints::default();

		let Context = TransportContext::New(Environment, Requirements, Constraints);
		// Without real detector injection, available transports come from DefaultTransportTypeDetector
		assert!(Context.TransportAvailable(TransportType::Grpc));
	}

	#[test]
	fn TestTransportRegistryCreation() {
		let Registry = TransportRegistry::New();
		assert!(Registry.List().is_empty());
		assert!(Registry.ActiveName().is_none());
	}

	#[tokio::test]
	async fn TestRegistryRegisterUnregister() {
		let mut Registry = TransportRegistry::New();

		let MockTransportInstance = Arc::new(MockTransport::New());
		Registry.Register("mock".to_string(), MockTransportInstance);

		assert!(Registry.Has("mock"));
		assert_eq!(Registry.List().len(), 1);

		Registry.Unregister("mock").await.unwrap();
		assert!(!Registry.Has("mock"));
	}

	/// Mock transport for testing
	#[derive(Debug, Clone)]
	struct MockTransport;

	impl MockTransport {
		fn New() -> Self {
			Self
		}
	}

	#[async_trait]
	impl CommonTransportStrategy for MockTransport {
		async fn Connect(&mut self) -> Result<(), TransportError> {
			Ok(())
		}

		async fn Disconnect(&mut self) -> Result<(), TransportError> {
			Ok(())
		}

		async fn SendRequest(
			&mut self,
			Request: UnifiedRequest,
		) -> Result<UnifiedResponse, TransportError> {
			Ok(UnifiedResponse::Success(
				Request.CorrelationIdentifier.clone().unwrap_or_default(),
				Vec::new(),
			))
		}

		async fn SendNotification(
			&mut self,
			_Notification: UnifiedRequest,
		) -> Result<(), TransportError> {
			Ok(())
		}

		fn StreamEvents(
			&self,
		) -> std::result::Result<
			futures::stream::BoxStream<'static, UnifiedResponse>,
			TransportError,
		> {
			Err(TransportError::NotSupported("Streaming not supported"))
		}

		fn IsConnected(&self) -> bool {
			true
		}

		fn LatencyMilliseconds(&self) -> u64 {
			0
		}

		fn TransportKind(&self) -> TransportType {
			TransportType::Grpc
		}

		fn Configuration(&self) -> &TransportConfig {
			static CONFIG: std::sync::OnceLock<TransportConfig> = std::sync::OnceLock::new();
			CONFIG.get_or_init(TransportConfig::default)
		}

		fn Capabilities(&self) -> TransportCapabilities {
			TransportCapabilities::default()
		}

		fn Metrics(&self) -> TransportMetrics {
			TransportMetrics::New()
		}

		fn SupportsStreaming(&self) -> bool {
			false
		}
	}
}
