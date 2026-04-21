# Changelog

All notable changes to Common (Core Library) are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/).

## [v2.1] - Q2 2026: Full Workbench Lift

### Added

- PascalCase enforcement across 34 Rust files in Source/LanguageFeature/,
  Source/Transport/, Source/TreeView/ (514 insertions, 705 deletions)
- EvaluatableExpression + InlineValues provider types in ProviderType.rs
- 19 LanguageFeature provider methods: ProvideCallHierarchy,
  ProvideCodeActions, ProvideCodeLenses, ProvideCompletions,
  ProvideDefinition, ProvideDocumentFormatting, ProvideDocumentHighlights,
  ProvideDocumentLinks, ProvideDocumentRangeFormatting,
  ProvideDocumentSymbols, ProvideFoldingRanges, ProvideHover,
  ProvideInlayHints, ProvideLinkedEditingRanges, ProvideOnTypeFormatting,
  ProvideReferences, ProvideRenameEdits, ProvideSelectionRanges,
  ProvideSemanticTokensFull
- Transport layer refactoring: CircuitBreaker.rs, IPC.rs, gRPC.rs naming
- 36 new TypeScript declaration files (.d.ts + .d.ts.map) for Workers API
- README expanded: 243 insertions with comprehensive documentation

## [v1.3] - Q4 2025: Dependency Maintenance

### Changed

- @cloudflare/workers-types kept current through Q4
- No source file changes; pure dependency stabilization

## [v1.2] - Q3 2025: Full Stack Integration

### Changed

- @cloudflare/workers-types: 4.20240721 → 4.20240925 series
- Documentation styling: 98 CSS lines modified in TypeDoc assets
- Source/Function/Access.ts, Source/Function/Encrypt.ts minor updates

## [v1.1] - Q2 2025: Architecture Buildout

### Added

- Cloudflare Workers library established (crypto operations focus)
- 8 Target TypeScript declaration files (Function/*.d.ts, Interface/*.d.ts)

### Changed

- Source/Function/Access.ts refactored for PascalCase compliance

## [v1.0] - Q1 2025: Integration Phase

### Changed

- CODE_OF_CONDUCT.md updated, README expanded (12 lines)
- Cloudflare workers-types continuous updates
- Maintenance-heavy quarter; no major features

## [v0.2] - Q4 2024: Architecture Solidification

### Added

- License expanded: 42 → 131 lines (AGPL-3.0 addition, October 4)
- TypeScript declarations restructured: 172 insertions across 18 Target/*.d.ts

### Changed

- Source/Interface/*.ts PascalCase conversions
- 146 deletions of old-format declarations, 172 insertions of new format

## [v0.1] - Q3 2024: Rapid Development

### Changed

- Documentation UI assets refactored (icons.js, main.js, style.css)
- @cloudflare/workers-types continuous updates
- Stable quarter; mostly dependency maintenance

## [Common/v0.0.6] - 2024-06-30

### Added

- Full Documentation rebuild (TypeDoc): 1,802 insertions in style.css + assets

## [Common/v0.0.5] - 2024-06-30

### Added

- 69 files changed, 1,802 insertions (documentation generation)

## [v0.0] - Q2 2024: Project Inception

### Added

- Source/Function/Access.ts, Source/Function/Encrypt.ts
- Cloudflare Workers API type definitions
- TypeDoc documentation generation pipeline
- GitHub Actions workflows, Dependabot configuration
