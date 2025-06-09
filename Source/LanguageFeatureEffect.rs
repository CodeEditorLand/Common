// File: Common/Source/LanguageFeatureEffect.rs
// Responsibility: Responsibility could not be determined.
// Modified: 2025-06-06 23:36:34 UTC

// ---------------------------------------------------------------------------------------------
// Language Feature Provider Effect 
// ---------------------------------------------------------------------------------------------
// Defines DTOs for language features, the trait
// (`LanguageFeatureProviderRegistry`) for managing language feature provider
// interactions, and `ActionEffect` constructors for invoking these features or
// registering/unregistering providers. These effects abstract the interaction
// with the central registry, which is typically implemented by a concrete
// `Environment` accessible via `AppRuntime`.
// ---------------------------------------------------------------------------------------------

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value; // Used for document selectors, various DTO fields, and opaque structures
use url::Url;

use crate::{
	effect::ActionEffect,
	errors::CommonError,
	// The AppRuntime struct/trait and Environment trait are expected to be defined elsewhere.
	// AppRuntime should provide access to an Environment that implements
	// Requires<Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>>.
	// E.g. via a method like `fn get_environment(&self) -> Arc<dyn Environment>;`
	//
	// Example:
	// pub trait AppRuntime {
	//     type Env: Environment;
	//     fn get_environment(&self) -> Arc<Self::Env>;
	// }
	// pub trait Environment: Requires<Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>> + Send + Sync { /* ...
	// */ }
	//
	// The `Requires` trait (e.g., `crate::environment::Requires`) would be:
	// pub trait Requires<T> { fn require(&self) -> T; }
};

// --- Common DTOs (Data Transfer Objects) ---

/// DTO for vscode.Position (0-based) - matches extHost.protocol.IPosition
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
	pub line_number:u32, // VS Code protocol uses lineNumber for 0-based line
	pub column:u32,      // VS Code protocol uses column for 0-based character
}

/// DTO for vscode.Range (0-based) - matches extHost.protocol.IRange
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct RangeDto {
	pub start_line_number:u32,
	pub start_column:u32,
	pub end_line_number:u32,
	pub end_column:u32,
}

/// DTO for vscode.MarkdownString - matches extHost.protocol.IMarkdownString
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IMarkdownStringDto {
	pub value:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_trusted:Option<Value>, // Can be boolean or { enabledCommands: string[] }
	#[serde(skip_serializing_if = "Option::is_none")]
	pub support_theme_icons:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub support_html:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_uri:Option<Value>, // UriComponents DTO (Value for flexibility)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uris:Option<HashMap<String, Value>>, // Record<string, UriComponents DTO>
}

/// DTO for vscode.Hover result - matches extHost.protocol.IHoverDto
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HoverResultDto {
	pub contents:Vec<IMarkdownStringDto>, // Array of IMarkdownString DTOs
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range:Option<RangeDto>,
	// VS Code's extHost.protocol.ts HoverWithId also has `id: number` for verbosity cache.
	// If hover verbosity is implemented, add `pub id: Option<u32>` here.
}

/// DTO for vscode.CompletionContext - using Value as its structure can vary.
/// Typically includes `triggerKind: languages.CompletionTriggerKind` and
/// `triggerCharacter?: string`.
pub type CompletionContextDto = Value;

/// DTO for the result of a completion provider - matches
/// extHost.protocol.ISuggestResultDto.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestResultDto {
	#[serde(rename = "x", skip_serializing_if = "Option::is_none")]
	pub list_cache_id:Option<u32>, // CacheId for the list (VS Code `_debugCompletionSessionCached`)
	#[serde(rename = "b")]
	pub suggestions:Vec<Value>, // Array of ISuggestDataDto as raw JSON Value
	#[serde(rename = "a")]
	pub default_ranges:Value, // { insert: IRange, replace: IRange } as Value
	#[serde(rename = "c", skip_serializing_if = "Option::is_none")]
	pub is_incomplete:Option<bool>,
	// #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
	// pub duration: Option<f64>, // Optional: duration of the provider
}

// --- DTOs for WorkspaceEdit ---
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEditTypeDto {
	Text = 1,
	File = 2,
	Cell = 3,
	CellReplace = 4,
	Snippet = 5,
	CellMetadata = 6,
	DocumentMetadata = 7,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditEntryBaseDto {
	#[serde(rename = "_type")]
	pub edit_type:FileEditTypeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata:Option<Value>, // IWorkspaceEditEntryMetadataDto as Value
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceTextEditDto {
	#[serde(flatten)]
	pub base:WorkspaceEditEntryBaseDto, // _type = FileEditTypeDto::Text or ::Snippet
	pub resource:Value, // UriComponents DTO
	pub edit:Value,     // ISingleEditOperation DTO or Snippet DTO
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version_id:Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFileEditDto {
	#[serde(flatten)]
	pub base:WorkspaceEditEntryBaseDto, // _type = FileEditTypeDto::File
	#[serde(skip_serializing_if = "Option::is_none")]
	pub old_uri:Option<Value>, // UriComponents DTO (oldResource in protocol)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_uri:Option<Value>, // UriComponents DTO (newResource in protocol)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub options:Option<Value>, // WorkspaceFileEditOptions DTO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceCellEditDto {
	// Structure for notebook cell edits
	#[serde(flatten)]
	pub base:WorkspaceEditEntryBaseDto, // _type = Cell, CellReplace, CellMetadata, DocumentMetadata
	pub resource:Value,          // Notebook URI (UriComponents DTO)
	pub cell_edit_payload:Value, // Placeholder for various cell edit structures
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditDto {
	// Each element is a Value representing one of WorkspaceTextEditDto, WorkspaceFileEditDto, or WorkspaceCellEditDto.
	// Consumers should deserialize to Value and then attempt to convert to a specific type based on `_type`.
	pub edits:Vec<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata:Option<Value>, // IWorkspaceEditMetadataDto { label, description, iconPath }
}

// --- DTOs for Diagnostics (IMarkerData) ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedInformationDto {
	pub resource:Value, // UriComponents DTO
	pub message:String,
	pub start_line_number:u32, // 1-based
	pub start_column:u32,      // 1-based
	pub end_line_number:u32,
	pub end_column:u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDataDto {
	// Mirrors IMarkerData
	pub severity:u32, // Corresponds to MarkerSeverity enum values
	pub message:String,
	pub start_line_number:u32, // 1-based
	pub start_column:u32,      // 1-based
	pub end_line_number:u32,
	pub end_column:u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code:Option<Value>, // string | { value: string, target: UriComponentsDto }
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model_version_id:Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub related_information:Option<Vec<RelatedInformationDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags:Option<Vec<u32>>, // Array of MarkerTag enum values
}

// --- DTOs for Commands, Code Actions, Code Lenses ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommandDto {
	pub id:String,
	pub title:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tooltip:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments:Option<Vec<Value>>,
	#[serde(rename = "$ident", skip_serializing_if = "Option::is_none")]
	pub ident:Option<String>, // For cached commands
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionDto {
	pub title:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind:Option<String>, // CodeActionKind.value
	#[serde(skip_serializing_if = "Option::is_none")]
	pub diagnostics:Option<Vec<MarkerDataDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub edit:Option<WorkspaceEditDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command:Option<CommandDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_preferred:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub disabled:Option<String>, // Reason string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranges:Option<Vec<RangeDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_ai:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cache_id:Option<(u32, u32)>, // [listCacheId, itemCacheId]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub _is_synthetic:Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionListDto {
	pub actions:Vec<CodeActionDto>, // Protocol sends (Command | CodeActionDto)[], simplified here
	pub cache_id:u32,               // List cache ID
}

/// DTO for vscode.CodeActionContext - using Value as its structure can vary.
/// Typically includes `diagnostics: IMarkerData[]`, `only?: string`,
/// `triggerKind?: CodeActionTriggerKind`.
pub type CodeActionContextDto = Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensDto {
	pub range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command:Option<CommandDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cache_id:Option<(u32, u32)>, // [listCacheId, itemCacheId]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensListDto {
	pub lenses:Vec<CodeLensDto>,
	pub cache_id:u32, // List cache ID
}

// --- DTOs for Symbols ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolDto {
	pub name:String,
	pub detail:String,
	pub kind:u32,              // languages.SymbolKind (numeric enum)
	pub tags:Option<Vec<u32>>, // languages.SymbolTag[] (numeric enum)
	pub range:RangeDto,
	pub selection_range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub children:Option<Vec<DocumentSymbolDto>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolDto {
	pub name:String,
	pub kind:u32, // languages.SymbolKind
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags:Option<Vec<u32>>, // languages.SymbolTag[]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub container_name:Option<String>,
	pub location:Value, // ILocationDto (UriComponents + IRange) as Value
}

// --- DTOs for Signature Help ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInformationDto {
	pub label:Value, // string | [number, number] (substring indicators)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub documentation:Option<IMarkdownStringDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformationDto {
	pub label:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub documentation:Option<IMarkdownStringDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters:Option<Vec<ParameterInformationDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active_parameter:Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpResultDto {
	pub signatures:Vec<SignatureInformationDto>,
	pub active_signature:u32,
	pub active_parameter:u32,
	// `id?: number` if SignatureHelpResult has a cacheId for retriggering (not in base protocol)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SignatureHelpTriggerKindDto {
	// languages.SignatureHelpTriggerKind
	Invoke = 1,
	TriggerCharacter = 2,
	ContentChange = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpContextDto {
	pub trigger_kind:SignatureHelpTriggerKindDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trigger_character:Option<String>,
	pub is_retrigger:bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active_signature_help:Option<SignatureHelpResultDto>, // ISignatureHelpDto
}

// --- DTOs for Formatting ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextEditDto {
	// ISingleEditOperation in extHost.protocol
	pub range:RangeDto,
	pub text:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eol:Option<u32>, // EndOfLineSequence enum value (0 for LF, 1 for CRLF)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptionsDto {
	pub tab_size:u32,
	pub insert_spaces:bool,
	#[serde(flatten)]
	pub additional_properties:HashMap<String, Value>, // For other properties
}

// --- DTOs for Document Highlights ---
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DocumentHighlightKindDto {
	// languages.DocumentHighlightKind
	Text = 0,
	Read = 1,
	Write = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightDto {
	pub range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind:Option<DocumentHighlightKindDto>,
}

// --- DTOs for Document Links ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkDto {
	// languages.ILink
	pub range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url:Option<Value>, // UriComponents DTO as Value, or string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tooltip:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data:Option<Value>, // Opaque data for resolve step
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinksListDto {
	// languages.ILinksListDto
	pub links:Vec<LinkDto>,
	// `id?: number` if this list is cacheable for resolve (not in base protocol ILinsListDto)
}

// --- DTOs for Location & References ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationLinkDto {
	// languages.LocationLink
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_selection_range:Option<RangeDto>,
	pub target_uri:Value, // UriComponents DTO as Value
	pub target_range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target_selection_range:Option<RangeDto>,
}

// --- DTOs for Rename ---
// `prepareRename` can return IRange or { range: IRange, placeholder: string }.
// This is represented as `Option<Value>` in the trait method.

// --- DTOs for Folding Ranges ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeDto {
	// languages.FoldingRange
	pub start_line:u32, // 0-based
	pub end_line:u32,   // 0-based
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind:Option<u32>, // languages.FoldingRangeKind (Comment=1, Imports=2, Region=3)
}

// --- DTOs for Selection Ranges ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeDto {
	// languages.SelectionRange
	pub range:RangeDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent:Option<Box<SelectionRangeDto>>, // Recursive
}

// --- DTOs for Linked Editing Ranges ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangesDto {
	// languages.ILinkedEditingRanges
	pub ranges:Vec<RangeDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub word_pattern:Option<String>, // RegExp source string
}

// --- DTOs for Semantic Tokens ---
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegendDto {
	// languages.SemanticTokensLegend
	pub token_types:Vec<String>,
	pub token_modifiers:Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDto {
	// languages.SemanticTokens or extHostProtocol.ISemanticTokensDto
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result_id:Option<String>,
	pub data:Vec<u32>, // Delta-encoded tokens as u32 array
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensEditDto {
	// languages.SemanticTokensEdit
	pub start:u32,
	pub delete_count:u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data:Option<Vec<u32>>, // Delta-encoded tokens for this edit
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensEditsDto {
	// languages.SemanticTokensEdits or extHostProtocol.ISemanticTokensEditsDto
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result_id:Option<String>, // MUST be present for edits
	pub edits:Vec<SemanticTokensEditDto>,
}

// --- DTOs for Call Hierarchy & Type Hierarchy ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HierarchyItemDto {
	// extHostProtocol.ICallHierarchyItemDto / ITypeHierarchyItemDto
	#[serde(flatten)]
	pub symbol_info:DocumentSymbolDto, // Reuses DocumentSymbolDto fields
	pub _session_id:String, // UUID string for session management
	pub _item_id:String,    // UUID string for item identification
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomingCallDto {
	// extHostProtocol.IIncomingCallDto
	pub from:HierarchyItemDto,
	pub from_ranges:Vec<RangeDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingCallDto {
	// extHostProtocol.IOutgoingCallDto
	pub to:HierarchyItemDto,
	pub from_ranges:Vec<RangeDto>,
}

// --- DTOs for Inlay Hints ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintLabelPartDto {
	// languages.IInlayHintLabelPartDto
	pub value:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tooltip:Option<Value>, // IMarkdownStringDto or string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location:Option<Value>, // ILocationDto (UriComponents + IRange)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command:Option<CommandDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum InlayHintKindDto {
	// languages.InlayHintKind
	Type = 1,
	Parameter = 2,
	Other = 0, // Not standard, but for completeness if needed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintDto {
	// languages.IInlayHintDto
	pub label:Value, // string | InlayHintLabelPartDto[]
	pub position:PositionDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind:Option<InlayHintKindDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tooltip:Option<Value>, // IMarkdownStringDto or string
	#[serde(skip_serializing_if = "Option::is_none")]
	pub padding_left:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub padding_right:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_edits:Option<Vec<TextEditDto>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data:Option<Value>, // Opaque data for resolve step (e.g., cacheId)
}

// --- Provider Type Enum and Options DTO ---

/// Enum identifying the type of language feature provider.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderType {
	Hover,
	Completion,
	Definition,
	Declaration,
	Implementation,
	TypeDefinition,
	References,
	DocumentHighlight,
	DocumentSymbol,
	WorkspaceSymbol,
	CodeAction,
	CodeLens,
	Formatting, // Document Formatting
	RangeFormatting,
	OnTypeFormatting,
	Rename,
	DocumentLink,
	// Color, // Not explicitly added in snippets, can be added if needed
	FoldingRange,
	SelectionRange,
	CallHierarchy,
	TypeHierarchy,
	LinkedEditingRange,
	InlayHints,
	SemanticTokens, // For full document semantic tokens
	SemanticTokensRange, /* For range-based semantic tokens
	                 * Add other provider types as needed (e.g., DocumentDropEdit, PasteEdit) */
}

/// DTO for provider registration options, sent from client (e.g., Cocoon) to
/// server (e.g., Mountain).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOptionsDto {
	// General
	#[serde(skip_serializing_if = "Option::is_none")]
	pub display_name:Option<String>,

	// Event Handles (for providers with onDidChange... events)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub on_did_change_code_lenses_event_handle:Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub on_did_change_inlay_hints_event_handle:Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub on_did_change_folding_ranges_event_handle:Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub on_did_change_document_semantic_tokens_event_handle:Option<u32>,
	// TODO: Add other onDidChange... event handles

	// CompletionProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trigger_characters:Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completion_supports_resolve_details:Option<bool>, // Was supports_resolve_details

	// CodeActionProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_action_metadata_dto:Option<Value>, // ICodeActionProviderMetadataDto
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_action_supports_resolve:Option<bool>,

	// SignatureHelpProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub signature_help_metadata_dto:Option<Value>, /* ISignatureHelpProviderMetadataDto (contains trigger/retrigger
	                                                * chars) */

	// InlayHintsProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub inlay_hints_supports_resolve:Option<bool>,
	// pub inlay_hints_label: Option<String>, // Usually part of provider registration call, not options struct

	// DocumentLinkProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub document_link_supports_resolve:Option<bool>,

	// Formatting (Document & Range)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub formatter_can_format_multiple_ranges:Option<bool>, // For range formatter
	// pub formatter_display_name: Option<String>, // Covered by general display_name

	// RenameProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rename_supports_resolve_location:Option<bool>,

	// WorkspaceSymbolProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub workspace_symbol_supports_resolve:Option<bool>,

	// DocumentSymbolProvider
	// pub document_symbol_label: Option<String>, // Usually an argument to registration, not options struct

	// SemanticTokensProvider
	#[serde(skip_serializing_if = "Option::is_none")]
	pub semantic_tokens_legend:Option<SemanticTokensLegendDto>, // languages.SemanticTokensLegend DTO

	// DocumentDropEditProvider & PasteEditProvider (Generic metadata field)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub edit_provider_metadata_dto:Option<Value>,
}

// --- Language Feature Provider Registry Trait ---

/// Defines methods for registering, unregistering, and invoking language
/// feature providers. This trait is expected to be implemented by the
/// application's environment (e.g., MountainEnvironment).
#[async_trait]
pub trait LanguageFeatureProviderRegistry: Send + Sync {
	/// Registers a language feature provider.
	///
	/// # Argument
	/// * `sidecar_id` - ID of the sidecar registering the provider.
	/// * `provider_type` - The type of provider being registered.
	/// * `selector` - JSON Value representing DocumentSelector.
	/// * `extension_id` - JSON Value representing IExtensionIdentifier of the
	///   extension providing this feature.
	/// * `options` - Provider-specific options.
	///
	/// # Returns
	/// A handle (u32) for identification and unregistration.
	async fn register_provider(
		&self,
		sidecar_id:String,
		provider_type:ProviderType,
		selector:Value,     // DocumentFilter | DocumentFilter[]
		extension_id:Value, // IExtensionIdentifierDto
		options:Option<ProviderOptionsDto>,
	) -> Result<u32, CommonError>;

	/// Unregisters a previously registered provider using its handle.
	async fn unregister_provider(
		&self,
		handle:u32,
		// sidecar_id: Option<String>, // Consider for ownership validation if needed
	) -> Result<(), CommonError>;

	// --- Provider Invocation Methods ---

	async fn provide_hover(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		// cancellation_token_id_val: Option<Value>, // Optional cancellation token
	) -> Result<Option<HoverResultDto>, CommonError>;

	async fn provide_completions(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		context_dto:CompletionContextDto, // Value representing CompletionContext
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<SuggestResultDto>, CommonError>;

	/// Resolves additional details for a completion item from a list.
	async fn resolve_completion_item_for_list(
		&self,
		list_cache_id:u32,         // Handle for the completion list session
		item_to_resolve_dto:Value, // ISuggestDataDto as Value (contains its own cache info)
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Value>, CommonError>; // Returns updated ISuggestDataDto as Value

	async fn provide_code_actions(
		&self,
		document_uri:Url,
		language_id:String,
		range_or_selection_dto:Value,     // IRange or ISelection DTO as Value
		context_dto:CodeActionContextDto, // Value representing CodeActionContext
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<CodeActionListDto>, CommonError>;

	async fn resolve_code_action(
		&self,
		list_cache_id:u32,           // From item's cache_id[0]
		sidecar_id:String,           // Sidecar that owns the provider
		action_to_resolve_dto:Value, // CodeActionDto as Value
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<CodeActionDto>, CommonError>;

	async fn provide_code_lenses(
		&self,
		document_uri:Url,
		language_id:String,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<CodeLensListDto>, CommonError>;

	async fn resolve_code_lens(
		&self,
		list_cache_id:u32,         // From item's cache_id[0]
		sidecar_id:String,         // Sidecar that owns the provider
		lens_to_resolve_dto:Value, // CodeLensDto as Value
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<CodeLensDto>, CommonError>;

	async fn provide_document_symbols(
		&self,
		document_uri:Url,
		language_id:String,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<DocumentSymbolDto>>, CommonError>;

	async fn provide_workspace_symbols(
		&self,
		query:String,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<WorkspaceSymbolDto>>, CommonError>;
	// TODO: Add resolveWorkspaceSymbol if needed by protocol

	async fn provide_signature_help(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		context_dto:SignatureHelpContextDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<SignatureHelpResultDto>, CommonError>;

	async fn provide_document_formatting_edits(
		&self,
		document_uri:Url,
		language_id:String,
		options_dto:FormattingOptionsDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;

	async fn provide_document_range_formatting_edits(
		&self,
		document_uri:Url,
		language_id:String,
		range_dto:RangeDto,
		options_dto:FormattingOptionsDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;

	async fn provide_on_type_formatting_edits(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		ch:String, // Character that was typed
		options_dto:FormattingOptionsDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<TextEditDto>>, CommonError>;

	async fn provide_document_highlights(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<DocumentHighlightDto>>, CommonError>;

	async fn provide_document_links(
		&self,
		document_uri:Url,
		language_id:String,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<LinksListDto>, CommonError>;

	async fn resolve_document_link(
		&self,
		// provider_handle might be encoded in link_to_resolve_dto.data if needed, or passed separately
		// For now, assuming link_to_resolve_dto.data has enough context or a global lookup is done.
		// If specific provider context is needed, add provider_handle and sidecar_id.
		link_to_resolve_dto:Value, // LinkDto as Value, potentially containing its own context/ID in `data`
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<LinkDto>, CommonError>;

	async fn provide_references(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		context_dto:Value, // languages.ReferenceContext DTO { includeDeclaration: boolean }
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<LocationLinkDto>>, CommonError>;

	async fn prepare_rename(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Value>, CommonError>; // Returns Option<IRangeDto | { range: IRangeDto, placeholder: string }> as Value

	async fn provide_rename_edits(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		new_name:String,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<WorkspaceEditDto>, CommonError>;

	async fn provide_folding_ranges(
		&self,
		document_uri:Url,
		language_id:String,
		context_dto:Value, // languages.FoldingContext DTO (not detailed in snippets)
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<FoldingRangeDto>>, CommonError>;

	async fn provide_selection_ranges(
		&self,
		document_uri:Url,
		language_id:String,
		positions_dto:Vec<PositionDto>,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<SelectionRangeDto>>, CommonError>;

	async fn provide_linked_editing_ranges(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<LinkedEditingRangesDto>, CommonError>;

	async fn provide_document_semantic_tokens(
		&self,
		document_uri:Url,
		language_id:String,
		previous_result_id:Option<String>, // For delta updates, if supported by provider
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<SemanticTokensDto>, CommonError>; // Returns full tokens

	async fn provide_document_semantic_tokens_edits(
		&self,
		document_uri:Url,
		language_id:String,
		previous_result_id:String, // Required for edits
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Value>, CommonError>; // Returns Option<SemanticTokensDto | SemanticTokensEditsDto as Value>

	async fn provide_document_range_semantic_tokens(
		&self,
		document_uri:Url,
		language_id:String,
		range_dto:RangeDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<SemanticTokensDto>, CommonError>;

	async fn prepare_call_hierarchy(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;

	async fn provide_call_hierarchy_incoming_calls(
		&self,
		// sidecar_id: String, // Needed if routing to specific sidecar that owns the session
		item_dto:HierarchyItemDto, // Contains _sessionId, _itemId
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<IncomingCallDto>>, CommonError>;

	async fn provide_call_hierarchy_outgoing_calls(
		&self,
		// sidecar_id: String,
		item_dto:HierarchyItemDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<OutgoingCallDto>>, CommonError>;

	async fn prepare_type_hierarchy(
		&self,
		document_uri:Url,
		language_id:String,
		position_dto:PositionDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;

	async fn provide_type_hierarchy_supertypes(
		&self,
		// sidecar_id: String,
		item_dto:HierarchyItemDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;

	async fn provide_type_hierarchy_subtypes(
		&self,
		// sidecar_id: String,
		item_dto:HierarchyItemDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<HierarchyItemDto>>, CommonError>;

	async fn provide_inlay_hints(
		&self,
		document_uri:Url,
		language_id:String,
		range_dto:RangeDto,
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<Vec<InlayHintDto>>, CommonError>;

	async fn resolve_inlay_hint(
		&self,
		// provider_handle: u32, // If hint_to_resolve_dto.data doesn't contain enough context
		// sidecar_id: String,   // If hint_to_resolve_dto.data doesn't contain enough context
		hint_to_resolve_dto:Value, // InlayHintDto as Value, potentially with context in `data`
		cancellation_token_id_val:Option<Value>,
	) -> Result<Option<InlayHintDto>, CommonError>;

	// TODO: Add methods for other providers:
	// Definition, Declaration, Implementation, TypeDefinition (these often return
	// LocationLinkDto or Vec<LocationLinkDto>) Color providers, etc.
}

// --- Effect Constructors ---
// Helper for boilerplate ActionEffect creation with AppRuntime
#[allow(dead_code)] // May not be used by all effects if they have unique structures
fn create_effect<F, Fut, R, E, RT>(f:F) -> ActionEffect<Arc<RT>, E, R>
where
	F: Fn(Arc<RT>, Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>) -> Fut + Send + Sync + 'static,
	Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
	R: Send + Sync + 'static,
	E: Send + Sync + 'static,
	// Assume AppRuntime has a get_environment method returning an Environment
	// And Environment implements Requires<Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>>
	// This generic RT represents the AppRuntime type.
	RT: Send + Sync + 'static + HasEnvironment, {
	ActionEffect::new(Arc::new(move |runtime_accessor:Arc<RT>| {
		let env = runtime_accessor.get_environment();
		let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
		Box::pin(f(runtime_accessor, registry))
	}))
}

// Trait to generalize AppRuntime access, assuming it's defined elsewhere
pub trait HasEnvironment {
	type Env: crate::environment::Requires<Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>> + Send + Sync; // Assuming Requires is in crate::environment
	fn get_environment(&self) -> Arc<Self::Env>;
}

/// Effect to register any language feature provider.
pub fn register_provider_effect<RT>(
	provider_type:ProviderType,
	selector:Value,
	sidecar_id:String,
	extension_id:Value, // IExtensionIdentifierDto as Value
	options:Option<ProviderOptionsDto>,
) -> ActionEffect<Arc<RT>, CommonError, u32>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let selector_clone = selector.clone();
		let sidecar_id_clone = sidecar_id.clone();
		let extension_id_clone = extension_id.clone();
		let options_clone = options.clone();
		let provider_type_clone = provider_type;

		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry
				.register_provider(
					sidecar_id_clone,
					provider_type_clone,
					selector_clone,
					extension_id_clone,
					options_clone,
				)
				.await
		})
	}))
}

/// Effect to unregister any language feature provider using its handle.
pub fn unregister_provider_effect<RT>(handle:u32) -> ActionEffect<Arc<RT>, CommonError, ()>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry.unregister_provider(handle).await
		})
	}))
}

/// Effect for invoking a hover provider.
pub fn provide_hover_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
) -> ActionEffect<Arc<RT>, CommonError, Option<HoverResultDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let doc_uri_clone = document_uri.clone();
		let lang_id_clone = language_id.clone();
		let pos_dto_clone = position_dto; // PositionDto is Copy

		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry.provide_hover(doc_uri_clone, lang_id_clone, pos_dto_clone).await
		})
	}))
}

/// Effect for providing completions.
pub fn provide_completions_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
	context_dto:CompletionContextDto, // Value
	cancellation_token_id_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<SuggestResultDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let doc_uri_clone = document_uri.clone();
		let lang_id_clone = language_id.clone();
		let pos_dto_clone = position_dto; // PositionDto is Copy
		let ctx_dto_clone = context_dto.clone();
		let token_val_clone = cancellation_token_id_val.clone();

		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry
				.provide_completions(doc_uri_clone, lang_id_clone, pos_dto_clone, ctx_dto_clone, token_val_clone)
				.await
		})
	}))
}

/// Effect for resolving a completion item.
pub fn resolve_completion_item_for_list_effect<RT>(
	list_cache_id:u32,
	item_to_resolve_dto:Value, // ISuggestDataDto as Value
	cancellation_token_id_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Value>>
// Returns Option<ISuggestDataDto as Value>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let list_id_clone = list_cache_id;
		let item_dto_clone = item_to_resolve_dto.clone();
		let token_clone = cancellation_token_id_val.clone();
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry
				.resolve_completion_item_for_list(list_id_clone, item_dto_clone, token_clone)
				.await
		})
	}))
}

/// Effect for providing document symbols.
pub fn provide_document_symbols_effect<RT>(
	document_uri:Url,
	language_id:String,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<DocumentSymbolDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let doc_uri_c = document_uri.clone();
		let lang_id_c = language_id.clone();
		let token_c = token_val.clone();
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry.provide_document_symbols(doc_uri_c, lang_id_c, token_c).await
		})
	}))
}

/// Effect for providing workspace symbols.
pub fn provide_workspace_symbols_effect<RT>(
	query:String,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<WorkspaceSymbolDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let q_c = query.clone();
		let token_c = token_val.clone();
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry.provide_workspace_symbols(q_c, token_c).await
		})
	}))
}

/// Effect for providing signature help.
pub fn provide_signature_help_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
	context_dto:SignatureHelpContextDto,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<SignatureHelpResultDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let doc_uri_c = document_uri.clone();
		let lang_id_c = language_id.clone();
		let pos_c = position_dto; // PositionDto is Copy
		let ctx_c = context_dto.clone(); // SignatureHelpContextDto may not be Copy
		let token_c = token_val.clone();
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry
				.provide_signature_help(doc_uri_c, lang_id_c, pos_c, ctx_c, token_c)
				.await
		})
	}))
}

/// Effect for providing document formatting edits.
pub fn provide_document_formatting_edits_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	opts:FormattingOptionsDto,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<TextEditDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |runtime_env_accessor:Arc<RT>| {
		let (uri_c, lid_c, o_c, t_c) = (doc_uri.clone(), lang_id.clone(), opts.clone(), token.clone());
		Box::pin(async move {
			let env = runtime_env_accessor.get_environment();
			let registry:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			registry.provide_document_formatting_edits(uri_c, lid_c, o_c, t_c).await
		})
	}))
}

// TODO: Add effect constructors for all other trait methods:
// provide_document_range_formatting_edits, provide_on_type_formatting_edits,
// provide_document_highlights, provide_document_links, resolve_document_link,
// provide_references, prepare_rename, provide_rename_edits,
// provide_folding_ranges, provide_selection_ranges,
// provide_linked_editing_ranges, provide_document_semantic_tokens (full),
// provide_document_semantic_tokens_edits,
// provide_document_range_semantic_tokens, prepare_call_hierarchy,
// provide_call_hierarchy_incoming_calls, provide_call_hierarchy_outgoing_calls,
// prepare_type_hierarchy, provide_type_hierarchy_supertypes,
// provide_type_hierarchy_subtypes, provide_inlay_hints, resolve_inlay_hint.
// And for CodeActions and CodeLenses: provide_code_actions,
// resolve_code_action, provide_code_lenses, resolve_code_lens.

/// Effect for providing references.
pub fn provide_references_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
	context_dto:Value,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<LocationLinkDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, pos_c, ctx_c, tok_c) = (
			document_uri.clone(),
			language_id.clone(),
			position_dto,
			context_dto.clone(),
			token_val.clone(),
		);
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_references(uri_c, lid_c, pos_c, ctx_c, tok_c).await
		})
	}))
}

/// Effect for preparing a rename operation.
pub fn prepare_rename_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Value>>
// Returns Option<Value>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, pos_c, tok_c) = (document_uri.clone(), language_id.clone(), position_dto, token_val.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.prepare_rename(uri_c, lid_c, pos_c, tok_c).await
		})
	}))
}

/// Effect for providing rename edits.
pub fn provide_rename_edits_effect<RT>(
	document_uri:Url,
	language_id:String,
	position_dto:PositionDto,
	new_name:String,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<WorkspaceEditDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, pos_c, nn_c, tok_c) = (
			document_uri.clone(),
			language_id.clone(),
			position_dto,
			new_name.clone(),
			token_val.clone(),
		);
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_rename_edits(uri_c, lid_c, pos_c, nn_c, tok_c).await
		})
	}))
}

/// Effect for providing folding ranges.
pub fn provide_folding_ranges_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	ctx_dto:Value,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<FoldingRangeDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, c_c, t_c) = (doc_uri.clone(), lang_id.clone(), ctx_dto.clone(), token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_folding_ranges(uri_c, lid_c, c_c, t_c).await
		})
	}))
}

/// Effect for providing selection ranges.
pub fn provide_selection_ranges_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	pos_dtos:Vec<PositionDto>,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<SelectionRangeDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, p_dtos_c, t_c) = (doc_uri.clone(), lang_id.clone(), pos_dtos.clone(), token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_selection_ranges(uri_c, lid_c, p_dtos_c, t_c).await
		})
	}))
}

/// Effect for providing linked editing ranges.
pub fn provide_linked_editing_ranges_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	pos_dto:PositionDto,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<LinkedEditingRangesDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, p_dto_c, t_c) = (doc_uri.clone(), lang_id.clone(), pos_dto, token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_linked_editing_ranges(uri_c, lid_c, p_dto_c, t_c).await
		})
	}))
}

/// Effect for providing full document semantic tokens.
pub fn provide_document_semantic_tokens_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	prev_id:Option<String>,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<SemanticTokensDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, pid_c, t_c) = (doc_uri.clone(), lang_id.clone(), prev_id.clone(), token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_document_semantic_tokens(uri_c, lid_c, pid_c, t_c).await
		})
	}))
}

/// Effect for providing semantic token edits.
pub fn provide_document_semantic_tokens_edits_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	prev_id:String,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Value>>
// Returns Value (SemanticTokensDto | SemanticTokensEditsDto)
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, pid_c, t_c) = (doc_uri.clone(), lang_id.clone(), prev_id.clone(), token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_document_semantic_tokens_edits(uri_c, lid_c, pid_c, t_c).await
		})
	}))
}

/// Effect for providing range-based semantic tokens.
pub fn provide_document_range_semantic_tokens_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	range:RangeDto,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<SemanticTokensDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, r_c, t_c) = (doc_uri.clone(), lang_id.clone(), range, token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_document_range_semantic_tokens(uri_c, lid_c, r_c, t_c).await
		})
	}))
}

/// Effect for providing incoming calls for a call hierarchy item.
pub fn provide_call_hierarchy_incoming_calls_effect<RT>(
	// sidecar_id_of_provider: String, // Pass if needed for routing to the correct provider session
	item_dto:HierarchyItemDto,
	token_val:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<IncomingCallDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let item_c = item_dto.clone();
		let token_c = token_val.clone();
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			// If sidecar_id is needed, pass it:
			// reg.provide_call_hierarchy_incoming_calls(sidecar_id_of_provider.clone(),
			// item_c, token_c).await
			reg.provide_call_hierarchy_incoming_calls(item_c, token_c).await
		})
	}))
}

// TODO: Add effect constructors for provide_call_hierarchy_outgoing_calls,
// prepare_type_hierarchy, etc.

/// Effect for providing inlay hints.
pub fn provide_inlay_hints_effect<RT>(
	doc_uri:Url,
	lang_id:String,
	range_dto:RangeDto,
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<Vec<InlayHintDto>>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let (uri_c, lid_c, r_c, t_c) = (doc_uri.clone(), lang_id.clone(), range_dto, token.clone());
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.provide_inlay_hints(uri_c, lid_c, r_c, t_c).await
		})
	}))
}

/// Effect for resolving an inlay hint.
pub fn resolve_inlay_hint_effect<RT>(
	// provider_handle: u32, // Add if needed
	// sidecar_id: String,   // Add if needed
	hint_dto_val:Value, // InlayHintDto as Value
	token:Option<Value>,
) -> ActionEffect<Arc<RT>, CommonError, Option<InlayHintDto>>
where
	RT: HasEnvironment + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |rt_acc:Arc<RT>| {
		let hint_c = hint_dto_val.clone();
		let t_c = token.clone();
		Box::pin(async move {
			let env = rt_acc.get_environment();
			let reg:Arc<dyn LanguageFeatureProviderRegistry + Send + Sync> = env.require();
			reg.resolve_inlay_hint(hint_c, t_c).await
			// If handle/sidecar needed: reg.resolve_inlay_hint(provider_handle,
			// sidecar_id.clone(), hint_c, t_c).await
		})
	}))
}

// --- Trait Requirement for Environment ---
// The `Requires<Arc<dyn LanguageFeatureProviderRegistry>>` trait must be
// implemented on the concrete `Environment` struct (e.g.,
// `MountainEnvironment`) that `AppRuntime::get_environment()` returns.
// Example (to be placed in the environment's module):
//
//   use crate::common::language_feature_effects::LanguageFeatureProviderRegistry;
//   use crate::environment::Requires; // Assuming Requires is in this path
//   use std::sync::Arc;
//
//   impl Requires<Arc<dyn LanguageFeatureProviderRegistry + Send + Sync>> for
// MyEnvironment {       fn require(&self) -> Arc<dyn
// LanguageFeatureProviderRegistry + Send + Sync> {           // Assuming
// MyEnvironment itself implements the LanguageFeatureProviderRegistry trait:
//           // self.clone()
//           // Or if it holds a dedicated service:
//           // self.language_feature_service.clone()
//           todo!("Return the actual LanguageFeatureProviderRegistry
// implementation")       }
//   }
//
//   // And the HasEnvironment trait for AppRuntime:
//   // use crate::common::language_feature_effects::HasEnvironment;
//   //
//   // impl HasEnvironment for AppRuntime {
//   //    type Env = MyEnvironment; // Or Arc<MyEnvironment> if get_environment
// returns Arc   //    fn get_environment(&self) -> Arc<Self::Env> {
// self.my_env_instance.clone() }   // }
