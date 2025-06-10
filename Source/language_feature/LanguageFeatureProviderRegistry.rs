use async_trait::async_trait;
use serde_json::Value;
use url::Url;

/// @module LanguageFeatureProviderRegistry
/// @description Defines the abstract service trait for managing and invoking
/// all language feature providers. This is the central contract for language
/// intelligence.
use super::dto::*;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can register,
/// unregister, and invoke all types of language feature providers.
#[async_trait]
pub trait LanguageFeatureProviderRegistry: Environment + Send + Sync {
	// --- Provider Management ---

	async fn RegisterProvider(
		&self,
		SidecarIdentifier:String,
		ProviderType:ProviderType,
		SelectorDto:Value,
		ExtensionIdentifierDto:Value,
		OptionsDto:Option<ProviderOptionsDto>,
	) -> Result<u32, CommonError>;

	async fn UnregisterProvider(&self, Handle:u32) -> Result<(), CommonError>;

	// --- Invocation Methods (sorted alphabetically) ---

	async fn PrepareCallHierarchy(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;
	async fn PrepareRename(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value>, CommonError>;
	async fn PrepareTypeHierarchy(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;
	async fn ProvideCallHierarchyIncomingCalls(
		&self,
		ItemDto:HierarchyItemDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<IncomingCallDto>>, CommonError>;
	async fn ProvideCallHierarchyOutgoingCalls(
		&self,
		ItemDto:HierarchyItemDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<OutgoingCallDto>>, CommonError>;
	async fn ProvideCodeActions(
		&self,
		DocumentUri:Url,
		RangeOrSelectionDto:Value,
		ContextDto:CodeActionContextDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<CodeActionListDto>, CommonError>;
	async fn ProvideCodeLenses(
		&self,
		DocumentUri:Url,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<CodeLensListDto>, CommonError>;
	async fn ProvideCompletions(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		ContextDto:CompletionContextDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<SuggestResultDto>, CommonError>;
	async fn ProvideDocumentFormattingEdits(
		&self,
		DocumentUri:Url,
		OptionsDto:FormattingOptionsDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;
	async fn ProvideDocumentHighlights(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<DocumentHighlightDto>>, CommonError>;
	async fn ProvideDocumentLinks(
		&self,
		DocumentUri:Url,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<LinksListDto>, CommonError>;
	async fn ProvideDocumentRangeFormattingEdits(
		&self,
		DocumentUri:Url,
		RangeDto:RangeDto,
		OptionsDto:FormattingOptionsDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;
	async fn ProvideDocumentRangeSemanticTokens(
		&self,
		DocumentUri:Url,
		RangeDto:RangeDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<SemanticTokensDto>, CommonError>;
	async fn ProvideDocumentSemanticTokens(
		&self,
		DocumentUri:Url,
		PreviousResultIdentifier:Option<String>,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<SemanticTokensDto>, CommonError>;
	async fn ProvideDocumentSemanticTokensEdits(
		&self,
		DocumentUri:Url,
		PreviousResultIdentifier:String,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value>, CommonError>;
	async fn ProvideDocumentSymbols(
		&self,
		DocumentUri:Url,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<DocumentSymbolDto>>, CommonError>;
	async fn ProvideFoldingRanges(
		&self,
		DocumentUri:Url,
		ContextDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<FoldingRangeDto>>, CommonError>;
	async fn ProvideHover(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
	) -> Result<Option<HoverResultDto>, CommonError>;
	async fn ProvideInlayHints(
		&self,
		DocumentUri:Url,
		RangeDto:RangeDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<InlayHintDto>>, CommonError>;
	async fn ProvideLinkedEditingRanges(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<LinkedEditingRangesDto>, CommonError>;
	async fn ProvideOnTypeFormattingEdits(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		Character:String,
		OptionsDto:FormattingOptionsDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;
	async fn ProvideReferences(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		ContextDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<LocationLinkDto>>, CommonError>;
	async fn ProvideRenameEdits(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		NewName:String,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<WorkspaceEditDto>, CommonError>;
	async fn ProvideSelectionRanges(
		&self,
		DocumentUri:Url,
		PositionsDto:Vec<PositionDto>,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<SelectionRangeDto>>, CommonError>;
	async fn ProvideSignatureHelp(
		&self,
		DocumentUri:Url,
		PositionDto:PositionDto,
		ContextDto:SignatureHelpContextDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<SignatureHelpResultDto>, CommonError>;
	async fn ProvideTypeHierarchySubtypes(
		&self,
		ItemDto:HierarchyItemDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;
	async fn ProvideTypeHierarchySupertypes(
		&self,
		ItemDto:HierarchyItemDto,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;
	async fn ProvideWorkspaceSymbols(
		&self,
		Query:String,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Vec<WorkspaceSymbolDto>>, CommonError>;

	// --- Resolver Methods ---

	async fn ResolveCodeAction(
		&self,
		ListCacheIdentifier:u32,
		ActionToResolveDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<CodeActionDto>, CommonError>;
	async fn ResolveCodeLens(
		&self,
		ListCacheIdentifier:u32,
		LensToResolveDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<CodeLensDto>, CommonError>;
	async fn ResolveCompletionItem(
		&self,
		ListCacheIdentifier:u32,
		ItemToResolveDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<Value>, CommonError>;
	async fn ResolveDocumentLink(
		&self,
		LinkToResolveDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<LinkDto>, CommonError>;
	async fn ResolveInlayHint(
		&self,
		HintToResolveDto:Value,
		CancellationTokenValue:Option<Value>,
	) -> Result<Option<InlayHintDto>, CommonError>;
}
