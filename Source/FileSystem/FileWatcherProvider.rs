//! # FileWatcherProvider Trait
//!
//! Defines the abstract contract for a service that registers recursive
//! filesystem watchers backed by the host platform's native notification
//! mechanism (inotify / FSEvents / ReadDirectoryChangesW). Watch events are
//! streamed back to the extension host as `$fileWatcher:event` notifications
//! so language features, TS watch-mode, HMR, and prettier-on-save all keep
//! working without polling.

use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// The kind of event a watcher observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchEventKind {
	Create,

	Change,

	Delete,
}

impl WatchEventKind {
	pub fn AsString(&self) -> &'static str {
		match self {
			WatchEventKind::Create => "create",

			WatchEventKind::Change => "change",

			WatchEventKind::Delete => "delete",
		}
	}
}

/// A single watch event emitted from the underlying notifier.
#[derive(Debug, Clone)]
pub struct WatchEvent {
	pub Handle:String,

	pub Kind:WatchEventKind,

	pub Path:PathBuf,
}

/// An abstract service contract for registering and cancelling recursive
/// filesystem watchers.
#[async_trait]
pub trait FileWatcherProvider: Environment + Send + Sync {
	/// Register a new watcher. `Handle` is a caller-supplied identifier
	/// (e.g. `"watcher:7"`) that must be echoed back in every emitted event so
	/// the extension host can route events to the right subscriber.
	///
	/// # Parameters
	/// * `Handle`:      Caller-supplied watcher identifier.
	/// * `Root`:        Absolute path of the directory to watch.
	/// * `IsRecursive`: When `true`, observe children recursively.
	/// * `Pattern`:     Optional glob pattern (e.g. `**/*.ts`). When present,
	///   events whose path does not match the compiled pattern are dropped
	///   before crossing the IPC boundary - this is critical for performance
	///   under TypeScript-style extensions that register 10+ watchers per
	///   activation.
	async fn RegisterWatcher(
		&self,

		Handle:String,

		Root:PathBuf,

		IsRecursive:bool,

		Pattern:Option<String>,
	) -> Result<(), CommonError>;

	/// Cancel a watcher registered with `RegisterWatcher`. Unknown handles
	/// resolve to `Ok(())` so callers may safely call `dispose()` without
	/// tracking registration state.
	async fn UnregisterWatcher(&self, Handle:String) -> Result<(), CommonError>;
}
