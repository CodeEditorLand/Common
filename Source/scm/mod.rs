

/**
 * @module scm
 * @description This module defines the abstract contract for the Source Control
 * Management (SCM) service, which is responsible for integrating with version
 * control systems like Git.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod ScmProvider;
pub use self::ScmProvider::ScmProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod RegisterScmProvider;
mod UpdateScmGroup;

pub use self::RegisterScmProvider::RegisterScmProvider;
pub use self::UpdateScmGroup::UpdateScmGroup;
