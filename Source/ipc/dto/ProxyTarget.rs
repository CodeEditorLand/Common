/// @module ProxyTarget
/// @description Defines the `ProxyTarget` enum for identifying the target
/// service or context for an RPC message, mirroring the
/// MainContext/ExtHostContext pattern from VS Code.

/// An enum that provides strongly-typed identifiers for all services that can
/// be communicated with across the IPC boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyTarget {
	// --- For ExtHost -> MainThread calls (Cocoon -> Mountain) ---
	MainThreadCommands,
	MainThreadConfiguration,
	MainThreadDiagnostics,
	MainThreadDocuments,
	MainThreadExtensionEnablement,
	MainThreadFileSystem,
	MainThreadLanguageFeatures,
	MainThreadLanguages,
	MainThreadOutputService,
	MainThreadSecrets,
	MainThreadStorage,
	MainThreadTerminalService,
	MainThreadWindow,
	MainThreadWebviews,
	MainThreadTelemetry,
	MainThreadWorkspace,
	MainThreadStatusBar,
	MainThreadScm,
	MainThreadTesting,
	MainThreadDebugService,
	MainThreadTaskService,
	MainThreadCustomEditors,
	MainThreadTreeView,

	// --- For MainThread -> ExtHost calls (Mountain -> Cocoon) ---
	ExtHostCommands,
	ExtHostConfiguration,
	ExtHostDiagnostics,
	ExtHostDocuments,
	ExtHostExtensionService,
	ExtHostFileSystemInfo,
	ExtHostLanguageFeatures,
	ExtHostLanguages,
	ExtHostOutputService,
	ExtHostStorage,
	ExtHostTerminalService,
	ExtHostEnv,
	ExtHostWebviews,
	ExtHostTelemetry,
	ExtHostChatProvider,
	ExtHostExtensionEnablement,
	ExtHostCustomEditors,
	ExtHostQuickInput,
	ExtHostMessageService,
	ExtHostDialogs,
	ExtHostAuthentication,
	ExtHostDebugService,
	ExtHostTaskService,
	ExtHostManagedSockets,
	ExtHostTreeView,
	ExtHostScm,
	ExtHostTesting,
}

impl ProxyTarget {
	/// Returns a string prefix representing the target, often used in
	/// constructing fully qualified RPC method names (e.g.,
	/// `MainThreadCommands$executeCommand`).
	pub fn GetTargetPrefix(&self) -> String { format!("{:?}", self) }
}
