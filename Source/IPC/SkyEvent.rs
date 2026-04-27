//! # Sky Event Registry - single source of truth for Mountain → Sky/Wind events
//!
//! Mountain emits Tauri events on `sky://…` URIs to notify the webview of
//! state changes that don't originate from a Wind-initiated `invoke` call.
//! Historically each emit site used a free-text string literal and each Wind
//! listener matched against its own free-text string - drift was invisible
//! until runtime (the listener simply never fired).
//!
//! `SkyEvent` is the enumerated registry. Mountain callers dispatch on the
//! variant; the wire string is produced by `AsStr()` and parsed by `FromStr`.
//! The matching TypeScript const object lives at
//! `Element/Wind/Source/IPC/SkyEvent.ts` - kept in sync by convention, same
//! protocol as the `Channel` registry.
//!
//! ## Adding a new event
//!
//! 1. Add the variant here AND in `Element/Wind/Source/IPC/SkyEvent.ts`.
//! 2. Emit from Mountain: `ApplicationHandle.emit(SkyEvent::TerminalData.AsStr(), Payload)`.
//! 3. Subscribe from Wind: `IPCService.events(SkyEvent.TerminalData)`.
//!
//! ## Why a declarative macro?
//!
//! Same rationale as `Channel`: the variant → wire-string mapping is pure
//! data. `DefineSkyEvents!` expands it into enum body + `AsStr` + `All` +
//! `FromStr` in one pass so adding an event is a single-line change that
//! compilers can't forget.

#![allow(non_snake_case, non_camel_case_types)]

macro_rules! DefineSkyEvents {
	($($Variant:ident => $Wire:literal,)* $(,)?) => {
		/// Enumerated Mountain → Sky/Wind event identifiers.
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub enum SkyEvent {
			$($Variant,)*
		}

		impl SkyEvent {
			/// Wire string produced on the Tauri event transport.
			pub fn AsStr(&self) -> &'static str {
				match self {
					$(Self::$Variant => $Wire,)*
				}
			}

			/// Full set of events, in declaration order.
			pub fn All() -> &'static [Self] {
				&[$(Self::$Variant,)*]
			}
		}

		impl ::std::fmt::Display for SkyEvent {
			fn fmt(&self, Formatter:&mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				Formatter.write_str(self.AsStr())
			}
		}

		impl ::std::str::FromStr for SkyEvent {
			type Err = ::std::string::String;

			fn from_str(Wire:&str) -> ::std::result::Result<Self, Self::Err> {
				match Wire {
					$($Wire => Ok(Self::$Variant),)*
					_ => Err(format!("unknown Sky event: {}", Wire)),
				}
			}
		}
	};
}

DefineSkyEvents! {
	// --- Configuration ---
	ConfigurationChanged                          => "sky://configuration/changed",

	// --- Debug ---
	DebugDapMessage                               => "sky://debug/dap-message",
	DebugRegister                                 => "sky://debug/register",
	DebugStart                                    => "sky://debug/start",
	DebugStop                                     => "sky://debug/stop",

	// --- Diagnostics ---
	DiagnosticsChanged                            => "sky://diagnostics/changed",

	// --- CustomEditor ---
	CustomEditorSaved                             => "sky://customEditor/saved",

	// --- Dialog ---
	DialogOpen                                    => "sky://dialog/open",
	DialogSave                                    => "sky://dialog/save",

	// --- Documents ---
	DocumentsOpen                                 => "sky://documents/open",
	DocumentsRenamed                              => "sky://documents/renamed",
	DocumentsSaved                                => "sky://documents/saved",

	// --- Editor ---
	EditorApplyEdits                              => "sky://editor/applyEdits",
	EditorOpenDocument                            => "sky://editor/openDocument",
	EditorSaveAll                                 => "sky://editor/saveAll",

	// --- Extensions ---
	ExtensionsInstalled                           => "sky://extensions/installed",
	ExtensionsUninstalled                         => "sky://extensions/uninstalled",

	// --- ExtHost ---
	ExtHostDebugClose                             => "sky://exthost/debug-close",
	ExtHostDebugReload                            => "sky://exthost/debug-reload",

	// --- Input ---
	InputBoxShow                                  => "sky://input-box/show",

	// --- Language ---
	LanguageConfigure                             => "sky://language/configure",
	LanguagesSetDocumentLanguage                  => "sky://languages/setDocumentLanguage",

	// --- Lifecycle ---
	LifecyclePhaseChanged                         => "sky://lifecycle/phaseChanged",
	LifecycleWillShutdown                         => "sky://lifecycle/willShutdown",

	// --- Native ---
	NativeOpenExternal                            => "sky://native/openExternal",

	// --- Notifications ---
	NotificationProgressBegin                     => "sky://notification/progress-begin",
	NotificationProgressEnd                       => "sky://notification/progress-end",
	NotificationProgressUpdate                    => "sky://notification/progress-update",
	NotificationShow                              => "sky://notification/show",

	// --- Output ---
	OutputAppend                                  => "sky://output/append",
	OutputClear                                   => "sky://output/clear",
	OutputCreate                                  => "sky://output/create",
	OutputDispose                                 => "sky://output/dispose",
	OutputReplace                                 => "sky://output/replace",
	OutputReveal                                  => "sky://output/reveal",
	OutputShow                                    => "sky://output/show",

	// --- Progress ---
	ProgressBegin                                 => "sky://progress/begin",
	ProgressComplete                              => "sky://progress/complete",
	ProgressEnd                                   => "sky://progress/end",
	ProgressReport                                => "sky://progress/report",
	ProgressStart                                 => "sky://progress/start",
	ProgressUpdate                                => "sky://progress/update",

	// --- QuickPick ---
	QuickPickShow                                 => "sky://quickpick/show",

	// --- Source Control ---
	SCMGroupChanged                               => "sky://scm/group/changed",
	SCMProviderAdded                              => "sky://scm/provider/added",
	SCMProviderChanged                            => "sky://scm/provider/changed",
	SCMProviderRemoved                            => "sky://scm/provider/removed",
	SCMRegister                                   => "sky://scm/register",
	SCMUpdateGroup                                => "sky://scm/updateGroup",

	// --- Status bar ---
	// Canonical prefix is `sky://statusbar/` (no hyphen). The earlier
	// `sky://status-bar/message` channel was an accidental fork produced by
	// a separate emit site and has been consolidated onto
	// `sky://statusbar/set-message`.
	StatusBarCreate                               => "sky://statusbar/create",
	StatusBarDispose                              => "sky://statusbar/dispose",
	StatusBarDisposeEntry                         => "sky://statusbar/dispose-entry",
	StatusBarDisposeMessage                       => "sky://statusbar/dispose-message",
	StatusBarSetEntry                             => "sky://statusbar/set-entry",
	StatusBarSetMessage                           => "sky://statusbar/set-message",
	StatusBarUpdate                               => "sky://statusbar/update",

	// --- Task ---
	TaskExecute                                   => "sky://task/execute",
	TaskTerminate                                 => "sky://task/terminate",

	// --- Terminal ---
	TerminalClosed                                => "sky://terminal/closed",
	TerminalCreate                                => "sky://terminal/create",
	TerminalData                                  => "sky://terminal/data",
	TerminalExit                                  => "sky://terminal/exit",
	TerminalHide                                  => "sky://terminal/hide",
	TerminalOpened                                => "sky://terminal/opened",
	TerminalProcessId                             => "sky://terminal/processId",
	TerminalResize                                => "sky://terminal/resize",
	TerminalShow                                  => "sky://terminal/show",

	// --- Test ---
	TestRegistered                                => "sky://test/registered",
	TestRunStarted                                => "sky://test/run-started",
	TestRunStatusChanged                          => "sky://test/run-status-changed",

	// --- Theme ---
	ThemeChange                                   => "sky://theme/change",

	// --- Tree view ---
	// Canonical prefix is `sky://tree-view/` (kebab-case). The earlier
	// `sky://treeView/register` camelCase channel was a parallel emission
	// from `CocoonService/TreeView.rs`; it has been collapsed into
	// `TreeViewCreate`, which every handler already subscribes to.
	TreeViewCreate                                => "sky://tree-view/create",
	TreeViewDispose                               => "sky://tree-view/dispose",
	TreeViewNodeExpanded                          => "sky://tree-view/node-expanded",
	TreeViewRefresh                               => "sky://tree-view/refresh",
	TreeViewRestoreState                          => "sky://tree-view/restore-state",
	TreeViewReveal                                => "sky://tree-view/reveal",
	TreeViewSelectionChanged                      => "sky://tree-view/selection-changed",
	TreeViewSetBadge                              => "sky://tree-view/set-badge",
	TreeViewSetMessage                            => "sky://tree-view/set-message",
	TreeViewSetTitle                              => "sky://tree-view/set-title",

	// --- UI ---
	// `UIShow{InputBox,QuickPick}Request` are deprecated aliases. The
	// Sky listener channels are `InputBoxShow` and `QuickPickShow`
	// declared earlier in this enum. `UserInterfaceProvider.rs` now
	// references those directly so the `UIShow*Request` channel names
	// below remain reachable only from older code paths and tests.
	UIShowInputBoxRequest                         => "sky://ui/show-input-box-request",
	UIShowMessageRequest                          => "sky://ui/show-message-request",
	UIShowQuickPickRequest                        => "sky://ui/show-quick-pick-request",

	// --- Virtual file system ---
	VFSFileChange                                 => "sky://vfs/fileChange",

	// --- Webview ---
	// Canonical form is kebab-case (`sky://webview/post-message`,
	// `sky://webview/set-html`). The `…CamelCase` aliases existed because
	// mod.rs emitted `sky://webview/postMessage` / `sky://webview/setHtml`
	// inline; those emit sites have been migrated to the enum so Sky only
	// ever sees the kebab-case form.
	WebviewCreate                                 => "sky://webview/create",
	WebviewCreated                                => "sky://webview/created",
	WebviewDispose                                => "sky://webview/dispose",
	WebviewDisposed                               => "sky://webview/disposed",
	WebviewMessage                                => "sky://webview/message",
	WebviewOptionsChanged                         => "sky://webview/options-changed",
	WebviewPostMessage                            => "sky://webview/post-message",
	WebviewRevealed                               => "sky://webview/revealed",
	WebviewSetHTML                                => "sky://webview/set-html",

	// --- Window ---
	WindowShowTextDocument                        => "sky://window/showTextDocument",

	// --- Workspace ---
	WorkspaceApplyEdit                            => "sky://workspace/applyEdit",
	WorkspacesChanged                             => "sky://workspaces/changed",
}

#[cfg(test)]
mod Tests {
	use super::SkyEvent;
	use std::str::FromStr;

	#[test]
	fn RoundTrip() {
		for Variant in SkyEvent::All() {
			let Wire = Variant.AsStr();
			let Parsed = SkyEvent::from_str(Wire).expect("round-trip");
			assert_eq!(*Variant, Parsed, "{} failed round-trip", Wire);
		}
	}

	#[test]
	fn EveryWireStartsWithSkyScheme() {
		for Variant in SkyEvent::All() {
			assert!(
				Variant.AsStr().starts_with("sky://"),
				"{} does not use the sky:// scheme",
				Variant.AsStr()
			);
		}
	}

	#[test]
	fn RejectsUnknown() {
		assert!(SkyEvent::from_str("mountain://nope").is_err());
		assert!(SkyEvent::from_str("").is_err());
	}

	/// Guards against drift between this Rust enum and its TS mirror at
	/// `Element/Wind/Source/IPC/SkyEvent.ts`. Both files are hand-edited,
	/// so the test scrapes the TS literal array and asserts every wire
	/// string here exists there, and vice versa. If this fails the two
	/// tables disagree - add or remove from whichever side is missing.
	#[test]
	fn RustAndTypeScriptTablesAgree() {
		use std::{collections::HashSet, path::PathBuf};

		let TsPath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("../Wind/Source/IPC/SkyEvent.ts");
		let Source = match std::fs::read_to_string(&TsPath) {
			Ok(S) => S,
			// In packaging contexts where Wind isn't checked out alongside
			// Common we skip the cross-check silently rather than
			// failing - the RoundTrip / UniqueWireStrings guards above
			// still cover the Rust side on its own.
			Err(_) => return,
		};

		let mut TsWires:HashSet<String> = HashSet::new();
		for Line in Source.lines() {
			if let Some(Start) = Line.find("\"sky://") {
				let Tail = &Line[Start + 1..];
				if let Some(End) = Tail.find('"') {
					TsWires.insert(Tail[..End].to_string());
				}
			}
		}

		let RsWires:HashSet<String> =
			SkyEvent::All().iter().map(|V| V.AsStr().to_string()).collect();

		let OnlyInRust:Vec<_> = RsWires.difference(&TsWires).collect();
		let OnlyInTs:Vec<_> = TsWires.difference(&RsWires).collect();

		assert!(
			OnlyInRust.is_empty() && OnlyInTs.is_empty(),
			"SkyEvent drift between Rust and TS:\n  only in Rust: {:?}\n  only in TS:   {:?}",
			OnlyInRust,
			OnlyInTs
		);
	}

	#[test]
	fn UniqueWireStrings() {
		let mut Seen = std::collections::HashSet::new();
		for Variant in SkyEvent::All() {
			assert!(Seen.insert(Variant.AsStr()), "duplicate wire: {}", Variant.AsStr());
		}
	}
}
