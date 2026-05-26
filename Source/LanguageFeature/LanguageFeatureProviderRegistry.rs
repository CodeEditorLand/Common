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
	/// * `SideCarIdentifier`: The ID of the sidecar hosting the provider.
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

		SideCarIdentifier:String,

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

	/// Provides rename edits for a symbol at the given position.
	async fn ProvideRenameEdits(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,

		NewName:String,
	) -> Result<Option<Value /* WorkspaceEditDTO */>, CommonError>;

	/// Provides document symbols (outline) for the given document.
	async fn ProvideDocumentSymbols(
		&self,

		DocumentURI:Url,
	) -> Result<Option<Value /* Vec<DocumentSymbolDTO> */>, CommonError>;

	/// Provides workspace symbols matching the given query.
	async fn ProvideWorkspaceSymbols(
		&self,

		Query:String,
	) -> Result<Option<Value /* Vec<WorkspaceSymbolDTO> */>, CommonError>;

	/// Provides signature help at the given position.
	async fn ProvideSignatureHelp(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,

		ContextDTO:Value,
	) -> Result<Option<Value /* SignatureHelpDTO */>, CommonError>;

	/// Provides folding ranges for the given document.
	async fn ProvideFoldingRanges(
		&self,

		DocumentURI:Url,
	) -> Result<Option<Value /* Vec<FoldingRangeDTO> */>, CommonError>;

	/// Provides selection ranges at the given positions.
	async fn ProvideSelectionRanges(
		&self,

		DocumentURI:Url,

		Positions:Vec<PositionDTO>,
	) -> Result<Option<Value /* Vec<SelectionRangeDTO> */>, CommonError>;

	/// Provides semantic tokens for the full document.
	async fn ProvideSemanticTokensFull(
		&self,

		DocumentURI:Url,
	) -> Result<Option<Value /* SemanticTokensDTO */>, CommonError>;

	/// Provides inlay hints within the given range.
	async fn ProvideInlayHints(
		&self,

		DocumentURI:Url,

		RangeDTO:Value,
	) -> Result<Option<Value /* Vec<InlayHintDTO> */>, CommonError>;

	/// Provides type hierarchy supertypes for the given item.
	async fn ProvideTypeHierarchySupertypes(
		&self,

		ItemDTO:Value,
	) -> Result<Option<Value /* Vec<TypeHierarchyItemDTO> */>, CommonError>;

	/// Prepares a call hierarchy session at the given position.
	/// Returns the root `CallHierarchyItem` or `None` if the provider
	/// has no hierarchy at that location. This is the entry point call;
	/// Mountain must issue this before requesting incoming/outgoing calls.
	async fn PrepareCallHierarchy(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,
	) -> Result<Option<Value /* Vec<CallHierarchyItemDTO> */>, CommonError>;

	/// Prepares a type hierarchy session at the given position.
	/// Returns the root `TypeHierarchyItem` or `None`.
	async fn PrepareTypeHierarchy(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,
	) -> Result<Option<Value /* Vec<TypeHierarchyItemDTO> */>, CommonError>;

	/// Provides type hierarchy subtypes for the given item.
	async fn ProvideTypeHierarchySubtypes(
		&self,

		ItemDTO:Value,
	) -> Result<Option<Value /* Vec<TypeHierarchyItemDTO> */>, CommonError>;

	/// Provides call hierarchy incoming calls for the given item.
	async fn ProvideCallHierarchyIncomingCalls(
		&self,

		ItemDTO:Value,
	) -> Result<Option<Value /* Vec<CallHierarchyCallDTO> */>, CommonError>;

	/// Provides call hierarchy outgoing calls for the given item.
	async fn ProvideCallHierarchyOutgoingCalls(
		&self,

		ItemDTO:Value,
	) -> Result<Option<Value /* Vec<CallHierarchyCallDTO> */>, CommonError>;

	/// Provides linked editing ranges at the given position.
	async fn ProvideLinkedEditingRanges(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,
	) -> Result<Option<Value /* LinkedEditingRangesDTO */>, CommonError>;

	/// Provides file decoration (badge, tooltip, colour) for a resource URI.
	/// Called by the file explorer / SCM tree when rendering resource state.
	async fn ProvideFileDecoration(&self, ResourceURI:Url) -> Result<Option<Value>, CommonError>;

	/// Provides inline completion items (ghost text) consumed by extensions
	/// such as GitHub Copilot, Roo Code, and Continue.
	async fn ProvideInlineCompletionItems(
		&self,

		DocumentURI:Url,

		PositionDTO_:PositionDTO,

		ContextDTO:Value,
	) -> Result<Option<Value>, CommonError>;

	/// Provides on-type formatting edits.
	async fn ProvideOnTypeFormattingEdits(
		&self,

		DocumentURI:Url,

		PositionDTO:PositionDTO,

		Character:String,

		OptionsDTO:Value,
	) -> Result<Option<Vec<TextEditDTO>>, CommonError>;
}
