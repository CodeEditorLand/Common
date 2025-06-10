

/**
 * @module dto (LanguageFeatures)
 * @description Aggregates and re-exports all Data Transfer Objects (DTOs)
 * related to the various Language Feature APIs (e.g., completions, hovers, symbols).
 */

#![allow(non_snake_case, non_camel_case_types)]

mod CodeActionContextDto;
mod CodeActionDto;
mod CodeActionListDto;
mod CodeLensDto;
mod CodeLensListDto;
mod CommandDto;
mod CompletionContextDto;
mod DocumentHighlightDto;
mod DocumentHighlightKindDto;
mod DocumentSymbolDto;
mod FileEditTypeDto;
mod FoldingRangeDto;
mod FormattingOptionsDto;
mod HierarchyItemDto;
mod HoverResultDto;
mod IMarkdownStringDto;
mod IncomingCallDto;
mod InlayHintDto;
mod InlayHintKindDto;
mod InlayHintLabelPartDto;
mod LinkDto;
mod LinkedEditingRangesDto;
mod LinksListDto;
mod LocationLinkDto;
mod MarkerDataDto;
mod OutgoingCallDto;
mod ParameterInformationDto;
mod PositionDto;
mod ProviderDescriptionDto;
mod ProviderOptionsDto;
mod ProviderType;
mod RangeDto;
mod RelatedInformationDto;
mod SelectionRangeDto;
mod SemanticTokensDto;
mod SemanticTokensEditDto;
mod SemanticTokensEditsDto;
mod SemanticTokensLegendDto;
mod SignatureHelpContextDto;
mod SignatureHelpResultDto;
mod SignatureHelpTriggerKindDto;
mod SignatureInformationDto;
mod SuggestResultDto;
mod TextEditDto;
mod WorkspaceCellEditDto;
mod WorkspaceEditDto;
mod WorkspaceFileEditDto;
mod WorkspaceSymbolDto;
mod WorkspaceTextEditDto;

pub use self::CodeActionContextDto::CodeActionContextDto;
pub use self::CodeActionDto::CodeActionDto;
pub use self::CodeActionListDto::CodeActionListDto;
pub use self::CodeLensDto::CodeLensDto;
pub use self::CodeLensListDto::CodeLensListDto;
pub use self::CommandDto::CommandDto;
pub use self::CompletionContextDto::CompletionContextDto;
pub use self::DocumentHighlightDto::DocumentHighlightDto;
pub use self::DocumentHighlightKindDto::DocumentHighlightKindDto;
pub use self::DocumentSymbolDto::DocumentSymbolDto;
pub use self::FileEditTypeDto::FileEditTypeDto;
pub use self::FoldingRangeDto::FoldingRangeDto;
pub use self::FormattingOptionsDto::FormattingOptionsDto;
pub use self::HierarchyItemDto::HierarchyItemDto;
pub use self::HoverResultDto::HoverResultDto;
pub use self::IMarkdownStringDto::IMarkdownStringDto;
pub use self::IncomingCallDto::IncomingCallDto;
pub use self::InlayHintDto::InlayHintDto;
pub use self::InlayHintKindDto::InlayHintKindDto;
pub use self::InlayHintLabelPartDto::InlayHintLabelPartDto;
pub use self::LinkDto::LinkDto;
pub use self::LinkedEditingRangesDto::LinkedEditingRangesDto;
pub use self::LinksListDto::LinksListDto;
pub use self::LocationLinkDto::LocationLinkDto;
pub use self::MarkerDataDto::MarkerDataDto;
pub use self::OutgoingCallDto::OutgoingCallDto;
pub use self::ParameterInformationDto::ParameterInformationDto;
pub use self::PositionDto::PositionDto;
pub use self::ProviderDescriptionDto::ProviderDescriptionDto;
pub use self::ProviderOptionsDto::*; // Re-export all enums and structs from this file
pub use self::ProviderType::ProviderType;
pub use self::RangeDto::RangeDto;
pub use self::RelatedInformationDto::RelatedInformationDto;
pub use self::SelectionRangeDto::SelectionRangeDto;
pub use self::SemanticTokensDto::SemanticTokensDto;
pub use self::SemanticTokensEditDto::SemanticTokensEditDto;
pub use self::SemanticTokensEditsDto::SemanticTokensEditsDto;
pub use self::SemanticTokensLegendDto::SemanticTokensLegendDto;
pub use self::SignatureHelpContextDto::SignatureHelpContextDto;
pub use self::SignatureHelpResultDto::SignatureHelpResultDto;
pub use self::SignatureHelpTriggerKindDto::SignatureHelpTriggerKindDto;
pub use self::SignatureInformationDto::SignatureInformationDto;
pub use self::SuggestResultDto::SuggestResultDto;
pub use self::TextEditDto::TextEditDto;
pub use self::WorkspaceCellEditDto::WorkspaceCellEditDto;
pub use self::WorkspaceEditDto::WorkspaceEditDto;
pub use self::WorkspaceFileEditDto::WorkspaceFileEditDto;
pub use self::WorkspaceSymbolDto::WorkspaceSymbolDto;
pub use self::WorkspaceTextEditDto::WorkspaceTextEditDto;
