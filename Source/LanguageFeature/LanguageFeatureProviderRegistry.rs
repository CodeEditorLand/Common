//! # LanguageFeatureProviderRegistry Trait
//!
//! Defines the abstract service trait for managing and invoking all language
//! feature providers. This serves as the central contract for all language
//! intelligence capabilities.

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use super::DTO::{HoverResultDTO::HoverResultDTO, PositionDTO::PositionDTO};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can register,
/// unregister, and invoke all types of language feature providers (e.g., for
/// completions, hovers, definitions).
///
/// By consolidating all features into a single registry, we avoid the need for
/// dozens of separate provider traits, simplifying the overall architecture.
#[async_trait]
pub trait LanguageFeatureProviderRegistry: Environment + Send + Sync {
	// --- Provider Management ---

	/// Registers a new language feature provider.
	///
	/// # Parameters
	/// * `SidecarIdentifier`: The ID of the sidecar hosting the provider.
	/// * `ProviderType`: The type of feature this provider implements.
	/// * `SelectorDTO`: The document selector that determines which documents
	///   this provider applies to.
	/// * `ExtensionIdentifierDTO`: The ID of the extension contributing the
	///   provider.
	/// * `OptionsDTO`: Optional, feature-specific options.
	///
	/// # Returns
	/// A `Result` containing a unique handle (u32) for the new registration.
	async fn RegisterProvider(
		&self,
		SidecarIdentifier:String,
		ProviderType:ProviderType, // This DTO will be added in a future batch
		SelectorDTO:Value,
		ExtensionIdentifierDTO:Value,
		OptionsDTO:Option<Value /* ProviderOptionsDTO */>,
	) -> Result<u32, CommonError>;

	/// Unregisters a previously registered provider.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the provider registration to remove.
	async fn UnregisterProvider(&self, Handle:u32) -> Result<(), CommonError>;

	// --- Invocation Methods (sorted alphabetically) ---
	// Note: Most are placeholders with `Value` types for now. They will be
	// fleshed out with concrete DTOs in subsequent batches.

	async fn PrepareCallHierarchy(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<HierarchyItemDTO> */>, CommonError>;

	async fn PrepareRename(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value>, CommonError>;

	async fn PrepareTypeHierarchy(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<HierarchyItemDTO> */>, CommonError>;

	async fn ProvideCallHierarchyIncomingCalls(
		&self,
		ItemDTO:Value, // HierarchyItemDTO
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<IncomingCallDTO> */>, CommonError>;

	async fn ProvideCallHierarchyOutgoingCalls(
		&self,
		ItemDTO:Value, // HierarchyItemDTO
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<OutgoingCallDTO> */>, CommonError>;

	async fn ProvideCodeActions(
		&self,
		DocumentURI:Url,
		RangeOrSelectionDTO:Value,
		ContextDTO:Value, // CodeActionContextDTO
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* CodeActionListDTO */>, CommonError>;

	async fn ProvideCodeLenses(
		&self,
		DocumentURI:Url,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* CodeLensListDTO */>, CommonError>;

	async fn ProvideCompletions(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		ContextDTO:Value, // CompletionContextDTO
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* SuggestResultDTO */>, CommonError>;

	async fn ProvideDocumentFormattingEdits(
		&self,
		DocumentURI:Url,
		OptionsDTO:Value, // FormattingOptionsDTO
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<TextEditDTO> */>, CommonError>;

	async fn ProvideDocumentHighlights(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* Vec<DocumentHighlightDTO> */>, CommonError>;

	async fn ProvideDocumentLinks(
		&self,
		DocumentURI:Url,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value /* LinksListDTO */>, CommonError>;

	async fn ProvideHover(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
	) -> Result<Option<HoverResultDTO>, CommonError>;

	// ... other 20+ provider methods will be added here.
}
