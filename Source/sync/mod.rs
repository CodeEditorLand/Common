

//
// @module sync
// @description This module defines the abstract contract for the User Data Sync
// service, responsible for synchronizing settings, snippets, and other user
// data across different instances of the application.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod SyncProvider;
pub use self::SyncProvider::SyncProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod PullUserData;
mod PushUserData;

pub use self::PullUserData::PullUserData;
pub use self::PushUserData::PushUserData;
