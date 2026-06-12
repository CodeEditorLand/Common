//! # SourceControlManagement DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! Source Control Management (SourceControlManagement) API.

/// DTO for creating a new source control instance.
pub mod SourceControlCreateDTO;

/// DTO for updating a source control group.
pub mod SourceControlGroupUpdateDTO;

/// DTO for input box state in SCM views.
pub mod SourceControlInputBoxDTO;

/// DTO for a source control group.
pub mod SourceControlManagementGroupDTO;

/// DTO for a source control management provider.
pub mod SourceControlManagementProviderDTO;

/// DTO for a source control resource (file change).
pub mod SourceControlManagementResourceDTO;

/// DTO for updating an existing source control instance.
pub mod SourceControlUpdateDTO;
