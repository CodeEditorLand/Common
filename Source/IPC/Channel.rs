//! # Channel Registry - single source of truth for Wind ↔ Mountain IPC
//!
//! Every Tauri `invoke` request from Wind is a string-typed RPC. Historically
//! that string was hand-kept on both sides (Rust `match command.as_str()` in
//! `Mountain/Source/IPC/WindServiceHandlers.rs` and string literals in Wind's
//! `Effect/*/Live.ts` files). The result: drift - a channel could be declared
//! in Wind, referenced in Mountain's match, and still have no implementation
//! (the `extensions:install` no-op stub shipped for months).
//!
//! `Channel` is the enumerated registry. Rust callers dispatch on the variant;
//! the wire string is produced by `AsStr()` and parsed by `FromStr`. The
//! matching TypeScript const object lives at
//! `Element/Wind/Source/IPC/Channel.ts` - kept in sync by convention (a grep
//! diff is cheap; a full codegen would be overkill for 147 strings).
//!
//! ## Why a declarative macro?
//!
//! The variant → wire-string mapping is pure data. `DefineChannels!` expands
//! it into the enum body, `AsStr`, `All`, and `FromStr` in one pass so adding
//! a channel is a single-line change that compilers can't forget.
//!
//! ## Channel priority classes (Atom O3)
//!
//! `Priority` returns the Echo scheduler lane a given channel should dispatch
//! on. Used by the O1 wrap in `mountain_ipc_invoke` so user-facing latency
//! never queues behind background work. Three classes:
//!
//!   - `High`: direct user action (commands, file read, terminal input,
//!     notifications, VSIX install).
//!   - `Low`: background / deferrable (search, logging, update checks,
//!     offline-gallery stubs).
//!   - `Normal`: everything else.

#![allow(non_snake_case, non_camel_case_types)]

/// Lane selector for Echo scheduler dispatch.
///
/// Deliberately isolated from `Echo::Task::Priority` so Common stays
/// dependency-free on Echo. Mountain's `mountain_ipc_invoke` wrapper maps
/// `ChannelPriority` → `Echo::Task::Priority` at the single submit site.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChannelPriority {
	High,
	Normal,
	Low,
}

macro_rules! DefineChannels {
	($($Variant:ident => $Wire:literal,)* $(,)?) => {
		/// Enumerated IPC channel identifiers for Wind ↔ Mountain calls.
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub enum Channel {
			$($Variant,)*
		}

		impl Channel {
			/// Wire string produced on the Tauri transport.
			pub fn AsStr(&self) -> &'static str {
				match self {
					$(Self::$Variant => $Wire,)*
				}
			}

			/// Full set of channels, in declaration order.
			pub fn All() -> &'static [Self] {
				&[$(Self::$Variant,)*]
			}
		}

		impl ::std::fmt::Display for Channel {
			fn fmt(&self, Formatter:&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				Formatter.write_str(self.AsStr())
			}
		}

		impl ::std::str::FromStr for Channel {
			type Err = ::std::string::String;

			fn from_str(Wire:&str) -> ::std::result::Result<Self, Self::Err> {
				match Wire {
					$($Wire => Ok(Self::$Variant),)*
					_ => Err(format!("unknown IPC channel: {}", Wire)),
				}
			}
		}
	};
}

DefineChannels! {
	// --- Cocoon bridge ---
	CocoonExtensionHostMessage                    => "cocoon:extensionHostMessage",

	// --- Commands ---
	CommandsExecute                               => "commands:execute",
	CommandsGetAll                                => "commands:getAll",

	// --- Configuration ---
	ConfigurationGet                              => "configuration:get",
	ConfigurationUpdate                           => "configuration:update",

	// --- Decorations ---
	DecorationsClear                              => "decorations:clear",
	DecorationsGet                                => "decorations:get",
	DecorationsGetMany                            => "decorations:getMany",
	DecorationsSet                                => "decorations:set",

	// --- Diagnostics ---
	DiagnosticLog                                 => "diagnostic:log",

	// --- Encryption ---
	EncryptionDecrypt                             => "encryption:decrypt",
	EncryptionEncrypt                             => "encryption:encrypt",

	// --- Environment ---
	EnvironmentGet                                => "environment:get",

	// --- Extension host debug service ---
	ExtensionHostDebugServiceAttachSession        => "extensionhostdebugservice:attachSession",
	ExtensionHostDebugServiceClose                => "extensionhostdebugservice:close",
	ExtensionHostDebugServiceReload               => "extensionhostdebugservice:reload",
	ExtensionHostDebugServiceTerminateSession     => "extensionhostdebugservice:terminateSession",

	// --- Extensions ---
	ExtensionsGet                                 => "extensions:get",
	ExtensionsGetAll                              => "extensions:getAll",
	ExtensionsGetExtensions                       => "extensions:getExtensions",
	ExtensionsGetExtensionsControlManifest        => "extensions:getExtensionsControlManifest",
	ExtensionsGetInstalled                        => "extensions:getInstalled",
	ExtensionsGetRecommendations                  => "extensions:getRecommendations",
	ExtensionsGetUninstalled                      => "extensions:getUninstalled",
	ExtensionsInstall                             => "extensions:install",
	ExtensionsIsActive                            => "extensions:isActive",
	ExtensionsQuery                               => "extensions:query",
	ExtensionsReinstall                           => "extensions:reinstall",
	ExtensionsScanSystemExtensions                => "extensions:scanSystemExtensions",
	ExtensionsScanUserExtensions                  => "extensions:scanUserExtensions",
	ExtensionsUninstall                           => "extensions:uninstall",
	ExtensionsUpdateMetadata                      => "extensions:updateMetadata",

	// --- File system ---
	FileCloneFile                                 => "file:cloneFile",
	FileClose                                     => "file:close",
	FileCopy                                      => "file:copy",
	FileDelete                                    => "file:delete",
	FileExists                                    => "file:exists",
	FileMkdir                                     => "file:mkdir",
	FileMove                                      => "file:move",
	FileOpen                                      => "file:open",
	FileRead                                      => "file:read",
	FileReadBinary                                => "file:readBinary",
	FileReaddir                                   => "file:readdir",
	FileReadFile                                  => "file:readFile",
	FileRealpath                                  => "file:realpath",
	FileRename                                    => "file:rename",
	FileStat                                      => "file:stat",
	FileUnwatch                                   => "file:unwatch",
	FileWatch                                     => "file:watch",
	FileWrite                                     => "file:write",
	FileWriteBinary                               => "file:writeBinary",
	FileWriteFile                                 => "file:writeFile",

	// --- Git (renderer `localGit` channel; stock VS Code names it
	//          `ILocalGitService`, shared-process wire "localGit"). Land
	//          routes each method to a Mountain subprocess handler that
	//          spawns native `git`.
	GitCancel                                     => "git:cancel",
	GitCheckout                                   => "git:checkout",
	GitClone                                      => "git:clone",
	GitExec                                       => "git:exec",
	GitFetch                                      => "git:fetch",
	GitIsAvailable                                => "git:isAvailable",
	GitPull                                       => "git:pull",
	GitRevListCount                               => "git:revListCount",
	GitRevParse                                   => "git:revParse",

	// --- History ---
	HistoryCanGoBack                              => "history:canGoBack",
	HistoryCanGoForward                           => "history:canGoForward",
	HistoryClear                                  => "history:clear",
	HistoryGetStack                               => "history:getStack",
	HistoryGoBack                                 => "history:goBack",
	HistoryGoForward                              => "history:goForward",
	HistoryPush                                   => "history:push",

	// --- Keybindings ---
	KeybindingAdd                                 => "keybinding:add",
	KeybindingGetAll                              => "keybinding:getAll",
	KeybindingLookup                              => "keybinding:lookup",
	KeybindingRemove                              => "keybinding:remove",

	// --- Labels ---
	LabelGetBase                                  => "label:getBase",
	LabelGetURI                                   => "label:getUri",
	LabelGetWorkspace                             => "label:getWorkspace",

	// --- Lifecycle ---
	LifecycleAdvancePhase                         => "lifecycle:advancePhase",
	LifecycleGetPhase                             => "lifecycle:getPhase",
	LifecycleRequestShutdown                      => "lifecycle:requestShutdown",
	LifecycleSetPhase                             => "lifecycle:setPhase",
	LifecycleWhenPhase                            => "lifecycle:whenPhase",

	// --- Log (legacy / short) ---
	LogCreateLogger                               => "log:createLogger",
	LogRegisterLogger                             => "log:registerLogger",

	// --- Logger (current) ---
	LoggerCreateLogger                            => "logger:createLogger",
	LoggerCritical                                => "logger:critical",
	LoggerDebug                                   => "logger:debug",
	LoggerDeregisterLogger                        => "logger:deregisterLogger",
	LoggerError                                   => "logger:error",
	LoggerFlush                                   => "logger:flush",
	LoggerGetLevel                                => "logger:getLevel",
	LoggerGetRegisteredLoggers                    => "logger:getRegisteredLoggers",
	LoggerInfo                                    => "logger:info",
	LoggerLog                                     => "logger:log",
	LoggerRegisterLogger                          => "logger:registerLogger",
	LoggerSetLevel                                => "logger:setLevel",
	LoggerSetVisibility                           => "logger:setVisibility",
	LoggerTrace                                   => "logger:trace",
	LoggerWarn                                    => "logger:warn",

	// --- Menubar ---
	MenubarUpdateMenubar                          => "menubar:updateMenubar",

	// --- Model ---
	ModelClose                                    => "model:close",
	ModelGet                                      => "model:get",
	ModelGetAll                                   => "model:getAll",
	ModelOpen                                     => "model:open",
	ModelUpdateContent                            => "model:updateContent",

	// --- Native host ---
	NativeOpenExternal                            => "native:openExternal",
	NativeShowItemInFolder                        => "native:showItemInFolder",

	// --- Notifications ---
	NotificationEndProgress                       => "notification:endProgress",
	NotificationShow                              => "notification:show",
	NotificationShowProgress                      => "notification:showProgress",
	NotificationUpdateProgress                    => "notification:updateProgress",

	// --- Output channel ---
	OutputAppend                                  => "output:append",
	OutputAppendLine                              => "output:appendLine",
	OutputClear                                   => "output:clear",
	OutputCreate                                  => "output:create",
	OutputShow                                    => "output:show",

	// --- Progress ---
	ProgressBegin                                 => "progress:begin",
	ProgressEnd                                   => "progress:end",
	ProgressReport                                => "progress:report",

	// --- Search ---
	SearchFindFiles                               => "search:findFiles",
	SearchFindInFiles                             => "search:findInFiles",

	// --- Storage ---
	StorageClose                                  => "storage:close",
	StorageDelete                                 => "storage:delete",
	StorageGet                                    => "storage:get",
	StorageGetItems                               => "storage:getItems",
	StorageIsUsed                                 => "storage:isUsed",
	StorageKeys                                   => "storage:keys",
	StorageOptimize                               => "storage:optimize",
	StorageSet                                    => "storage:set",
	StorageUpdateItems                            => "storage:updateItems",

	// --- QuickInput (vscode.window.showQuickPick / showInputBox) ---
	QuickInputShowInputBox                        => "quickInput:showInputBox",
	QuickInputShowQuickPick                       => "quickInput:showQuickPick",

	// --- TextFile (editor working-copy surface) ---
	TextFileRead                                  => "textFile:read",
	TextFileWrite                                 => "textFile:write",
	TextFileSave                                  => "textFile:save",

	// --- WorkingCopy (dirty-state tracking) ---
	WorkingCopyGetAllDirty                        => "workingCopy:getAllDirty",
	WorkingCopyGetDirtyCount                      => "workingCopy:getDirtyCount",
	WorkingCopyIsDirty                            => "workingCopy:isDirty",
	WorkingCopySetDirty                           => "workingCopy:setDirty",

	// --- Terminal ---
	TerminalCreate                                => "terminal:create",
	TerminalDispose                               => "terminal:dispose",
	TerminalHide                                  => "terminal:hide",
	TerminalSendText                              => "terminal:sendText",
	TerminalShow                                  => "terminal:show",

	// --- Themes ---
	ThemesGetActive                               => "themes:getActive",
	ThemesList                                    => "themes:list",
	ThemesSet                                    => "themes:set",

	// --- Update ---
	UpdateApplyUpdate                             => "update:applyUpdate",
	UpdateCheckForUpdates                         => "update:checkForUpdates",
	UpdateDownloadUpdate                          => "update:downloadUpdate",
	UpdateIsLatestVersion                         => "update:isLatestVersion",
	UpdateQuitAndInstall                          => "update:quitAndInstall",

	// --- URL handlers ---
	URLRegisterExternalURIOpener                  => "url:registerExternalUriOpener",

	// --- Workbench ---
	WorkbenchGetConfiguration                     => "workbench:getConfiguration",

	// --- Workspaces ---
	WorkspacesAddFolder                           => "workspaces:addFolder",
	WorkspacesAddRecentlyOpened                   => "workspaces:addRecentlyOpened",
	WorkspacesClearRecentlyOpened                 => "workspaces:clearRecentlyOpened",
	WorkspacesCreateUntitledWorkspace             => "workspaces:createUntitledWorkspace",
	WorkspacesDeleteUntitledWorkspace             => "workspaces:deleteUntitledWorkspace",
	WorkspacesEnterWorkspace                      => "workspaces:enterWorkspace",
	WorkspacesGetDirtyWorkspaces                  => "workspaces:getDirtyWorkspaces",
	WorkspacesGetFolders                          => "workspaces:getFolders",
	WorkspacesGetName                             => "workspaces:getName",
	WorkspacesGetRecentlyOpened                   => "workspaces:getRecentlyOpened",
	WorkspacesGetWorkspaceIdentifier              => "workspaces:getWorkspaceIdentifier",
	WorkspacesRemoveFolder                        => "workspaces:removeFolder",
	WorkspacesRemoveRecentlyOpened                => "workspaces:removeRecentlyOpened",

	// --- Legacy wire-shape channels (non prefix:method) ---
	// Two historical groups predate the `prefix:method` convention:
	//   1. `UserInterface.Show*Dialog` - dotted names mirrored from
	//      Cocoon→Mountain gRPC; the Wind-side Files/Live.ts routes them
	//      through Tauri IPC today. Rename target: `dialog:showOpen` /
	//      `dialog:showSave`.
	//   2. `mountain_get_status` - snake_case Tauri command.
	// Grouped at the tail so the eventual rename is a single block move.
	MountainGetStatus                             => "mountain_get_status",
	UserInterfaceShowOpenDialog                   => "UserInterface.ShowOpenDialog",
	UserInterfaceShowSaveDialog                   => "UserInterface.ShowSaveDialog",
}

impl Channel {
	/// Echo scheduler lane for this channel. See module-level docs for the
	/// classification rationale.
	pub fn Priority(&self) -> ChannelPriority {
		use Channel::*;

		match self {
			// --- Direct user action → High ---
			CommandsExecute
			| CocoonExtensionHostMessage
			| ExtensionsInstall
			| ExtensionsUninstall
			| ExtensionsReinstall
			| FileRead
			| FileReadBinary
			| FileReadFile
			| FileStat
			| FileExists
			| FileOpen
			| FileWrite
			| FileWriteBinary
			| FileWriteFile
			| FileDelete
			| FileCopy
			| FileMove
			| FileRename
			| FileMkdir
			| KeybindingLookup
			| MenubarUpdateMenubar
			| ModelUpdateContent
			| NativeOpenExternal
			| NativeShowItemInFolder
			| NotificationShow
			| NotificationShowProgress
			| NotificationUpdateProgress
			| NotificationEndProgress
			| TerminalCreate
			| TerminalSendText
			| TerminalShow
			| TerminalHide
			| TerminalDispose
			| WorkspacesEnterWorkspace
			| WorkspacesAddFolder
			| WorkspacesRemoveFolder
			| WorkspacesCreateUntitledWorkspace
			| WorkspacesDeleteUntitledWorkspace => ChannelPriority::High,

			// --- Background / deferrable → Low ---
			GitClone
			| GitFetch
			| GitPull
			| GitRevListCount
			| SearchFindFiles
			| SearchFindInFiles
			| LogCreateLogger
			| LogRegisterLogger
			| LoggerCreateLogger
			| LoggerCritical
			| LoggerDebug
			| LoggerDeregisterLogger
			| LoggerError
			| LoggerFlush
			| LoggerGetLevel
			| LoggerGetRegisteredLoggers
			| LoggerInfo
			| LoggerLog
			| LoggerRegisterLogger
			| LoggerSetLevel
			| LoggerSetVisibility
			| LoggerTrace
			| LoggerWarn
			| StorageOptimize
			| UpdateCheckForUpdates
			| UpdateDownloadUpdate
			| UpdateApplyUpdate
			| UpdateIsLatestVersion
			| UpdateQuitAndInstall
			| ExtensionsQuery
			| ExtensionsGetRecommendations
			| ExtensionsGetExtensions
			| ExtensionsGetExtensionsControlManifest
			| ExtensionsGetUninstalled
			| ExtensionsUpdateMetadata
			| DiagnosticLog => ChannelPriority::Low,

			// --- Everything else → Normal ---
			_ => ChannelPriority::Normal,
		}
	}
}

#[cfg(test)]
mod Tests {
	use super::{Channel, ChannelPriority};
	use std::str::FromStr;

	#[test]
	fn RoundTrip() {
		for Variant in Channel::All() {
			let Wire = Variant.AsStr();
			let Parsed = Channel::from_str(Wire).expect("round-trip");
			assert_eq!(*Variant, Parsed, "{} failed round-trip", Wire);
		}
	}

	#[test]
	fn PriorityIsTotal() {
		// Every variant must match one of three classes; the `_` fallback
		// returning Normal guarantees totality, so this test just runs the
		// mapping on every variant to catch any future match panic.
		for Variant in Channel::All() {
			let _Class = Variant.Priority();
		}
	}

	#[test]
	fn UserActionIsHigh() {
		assert_eq!(Channel::CommandsExecute.Priority(), ChannelPriority::High);
		assert_eq!(Channel::ExtensionsInstall.Priority(), ChannelPriority::High);
		assert_eq!(Channel::TerminalSendText.Priority(), ChannelPriority::High);
	}

	#[test]
	fn BackgroundIsLow() {
		assert_eq!(Channel::SearchFindInFiles.Priority(), ChannelPriority::Low);
		assert_eq!(Channel::LoggerInfo.Priority(), ChannelPriority::Low);
	}

	#[test]
	fn RejectsUnknown() {
		assert!(Channel::from_str("nope:nope").is_err());
		assert!(Channel::from_str("").is_err());
	}

	#[test]
	fn UniqueWireStrings() {
		let mut Seen = std::collections::HashSet::new();
		for Variant in Channel::All() {
			assert!(Seen.insert(Variant.AsStr()), "duplicate wire: {}", Variant.AsStr());
		}
	}
}
