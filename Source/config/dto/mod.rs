

//
// @module dto (Config)
// @description Aggregates and re-exports all Data Transfer Objects (DTOs)
// related to the Configuration service.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- DTO Definitions ---
mod ConfigurationInitDataDto;
mod ConfigurationOverridesDto;
mod ConfigurationScope;
mod ConfigurationTarget;
mod InspectResultDataDto;

// --- Public Re-exports ---
pub use self::ConfigurationInitDataDto::ConfigurationInitDataDto;
pub use self::ConfigurationOverridesDto::ConfigurationOverridesDto;
pub use self::ConfigurationScope::ConfigurationScope;
pub use self::ConfigurationTarget::ConfigurationTarget;
pub use self::InspectResultDataDto::InspectResultDataDto;
