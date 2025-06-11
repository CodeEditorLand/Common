

//
// @module dto (Scm)
// @description Aggregates and re-exports all Data Transfer Objects (DTOs)
// related to the Source Control Management (SCM) API.
//

#![allow(non_snake_case, non_camel_case_types)]

mod ScmGroupDto;
mod ScmProviderDto;
mod ScmResourceDto;

pub use self::ScmGroupDto::ScmGroupDto;
pub use self::ScmProviderDto::ScmProviderDto;
pub use self::ScmResourceDto::ScmResourceDto;
