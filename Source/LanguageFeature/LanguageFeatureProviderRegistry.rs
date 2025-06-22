// File: Common/Source/LanguageFeature/LanguageFeatureProviderRegistry.rs
// Role: Defines the abstract service trait for managing and invoking all
// language       feature providers. This serves as the central contract for all
// language       intelligence capabilities.
// Responsibilities:
//   - Provide a contract for registering and unregistering providers.
//   - Define the invocation signature for every language feature (e.g., hover,
//     completion).

//! # LanguageFeatureProviderRegistry Trait
//!
//! Defines the abstract service trait for managing and invoking all language
//! feature providers. This serves as the central contract for all language
//! intelligence capabilities.

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use super::DTO::{
	CompletionContextDTO::CompletionContextDTO,
	CompletionListDTO::CompletionListDTO,
	HoverResultDTO::HoverResultDTO,
	LocationDTO::LocationDTO,
	PositionDTO::PositionDTO,
	ProviderType::ProviderType,
	TextEditDTO::TextEditDTO,
};
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
		ProviderType:ProviderType,
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

	async fn ProvideCodeActions(
		&self,
		DocumentURI:Url,
		// Range DTO
		RangeOrSelectionDTO:Value,
		// CodeActionContextDTO
		ContextDTO:Value,
	) -> Result<Option<Value /* CodeActionListDTO */>, CommonError>;

	async fn ProvideCodeLenses(&self, DocumentURI:Url) -> Result<Option<Value /* CodeLensListDTO */>, CommonError>;

	async fn ProvideCompletions(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		ContextDTO:CompletionContextDTO,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<CompletionListDTO>, CommonError>;

	async fn ProvideDefinition(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
	) -> Result<Option<Vec<LocationDTO>>, CommonError>;

	async fn ProvideDocumentFormattingEdits(
		&self,
		DocumentURI:Url,
		// FormattingOptions DTO
		OptionsDTO:Value,
	) -> Result<Option<Vec<TextEditDTO>>, CommonError>;

	async fn ProvideDocumentHighlights(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
	) -> Result<Option<Value /* Vec<DocumentHighlightDTO> */>, CommonError>;

	async fn ProvideDocumentLinks(&self, DocumentURI:Url) -> Result<Option<Value /* LinksListDTO */>, CommonError>;

	async fn ProvideDocumentRangeFormattingEdits(
		&self,
		DocumentURI:Url,
		// Range DTO
		RangeDTO:Value,
		// FormattingOptions DTO
		OptionsDTO:Value,
	) -> Result<Option<Vec<TextEditDTO>>, CommonError>;

	async fn ProvideHover(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
	) -> Result<Option<HoverResultDTO>, CommonError>;

	async fn ProvideReferences(
		&self,
		DocumentURI:Url,
		PositionDTO:PositionDTO,
		// ReferenceContext DTO
		ContextDTO:Value,
	) -> Result<Option<Vec<LocationDTO>>, CommonError>;

	async fn PrepareRename(&self, DocumentURI:Url, PositionDTO:PositionDTO) -> Result<Option<Value>, CommonError>;

	// ... other provider methods will be added here.
}
