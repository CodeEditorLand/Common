# Changelog - Common

Common is our abstract core library - the Rust crate that holds the trait
definitions, the `ActionEffect` system, and the DTOs every other element
in the fleet shares. If a shape is spoken across the IPC boundary, it
lives here. This file records what we built in our voice, version by
version. Format adapted from [Keep a Changelog](https://keepachangelog.com/).

## [v2.1] - Full Workbench Lift

We turned Common into the single source of truth for the language-feature
provider surface and the transport vocabulary the rest of the fleet
consumes.

### Added

- **PascalCase enforcement** across 34 Rust files in
  `Source/LanguageFeature/`, `Source/Transport/`, and `Source/TreeView/`
  (514 insertions, 705 deletions). Brought Common in line with the
  project-wide naming rule end-to-end.
- **`EvaluatableExpression` + `InlineValues` provider types** in
  `ProviderType.rs` so the workbench's debug surface has the shapes it
  reaches for at evaluation time.
- **19 `LanguageFeature` provider methods** standardised: `ProvideCallHierarchy`,
  `ProvideCodeActions`, `ProvideCodeLenses`, `ProvideCompletions`,
  `ProvideDefinition`, `ProvideDocumentFormatting`, `ProvideDocumentHighlights`,
  `ProvideDocumentLinks`, `ProvideDocumentRangeFormatting`,
  `ProvideDocumentSymbols`, `ProvideFoldingRanges`, `ProvideHover`,
  `ProvideInlayHints`, `ProvideLinkedEditingRanges`, `ProvideOnTypeFormatting`,
  `ProvideReferences`, `ProvideRenameEdits`, `ProvideSelectionRanges`,
  `ProvideSemanticTokensFull`.
- **Transport-layer refactor** giving us `CircuitBreaker.rs`, `IPC.rs`,
  and `gRPC.rs` as the named primitives every other element consumes.
- **36 new TypeScript declaration files** (`.d.ts` + `.d.ts.map`) for
  the Workers API surface so the renderer side gets accurate types.

### Changed

- **README expansion**: +243 lines of comprehensive documentation
  covering the trait surface, the DTO conventions, and the transport
  vocabulary.

## [v1.3] - Dependency Maintenance

`@cloudflare/workers-types` rolled forward through the cycle. No source
changes; pure dependency stabilisation.

## [v1.2] - Full-Stack Integration

`@cloudflare/workers-types` 4.20240721 → 4.20240925 series.
Documentation styling refreshed (98 CSS lines in TypeDoc assets).
Minor updates in `Source/Function/Access.ts` and
`Source/Function/Encrypt.ts`.

## [v1.1] - Architecture Buildout

We stood up Common's Cloudflare Workers library with a crypto-operations
focus, plus the first 8 generated TypeScript declaration files
(`Function/*.d.ts`, `Interface/*.d.ts`). `Source/Function/Access.ts`
moved to PascalCase in the same window.

## [v1.0] - Integration Phase

`CODE_OF_CONDUCT.md` refresh, README expanded by 12 lines, continuous
`@cloudflare/workers-types` rolls. Maintenance-heavy cycle - no major
new features.

## [v0.2] - Architecture Solidification

### Added

- **License expansion** from 42 to 131 lines (AGPL-3.0 addition,
  October 4).
- **TypeScript declarations restructured**: 172 insertions across 18
  `Target/*.d.ts` files in the new format.

### Changed

- **PascalCase conversions** across `Source/Interface/*.ts`.
- 146 deletions of old-format declarations, 172 insertions of new
  format - the cutover was atomic.

## [v0.1] - Rapid Development

We refreshed the documentation UI assets (`icons.js`, `main.js`,
`style.css`) and rolled `@cloudflare/workers-types` continuously.
Stable cycle, mostly dependency maintenance.

## [v0.0.6] - 2024-06-30

Full TypeDoc rebuild: 1,802 insertions in `style.css` and asset bundle.

## [v0.0.5] - 2024-06-30

69 files changed, 1,802 insertions - the first full documentation
generation pass.

## [v0.0] - Project Inception

The first scaffold:

- **`Source/Function/Access.ts`** and **`Source/Function/Encrypt.ts`** -
  the initial crypto-operation surface.
- **Cloudflare Workers API type definitions**.
- **TypeDoc documentation pipeline**.
- **GitHub Actions workflows** + Dependabot configuration.
