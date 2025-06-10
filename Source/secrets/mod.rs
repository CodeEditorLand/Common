

/**
 * @module secrets
 * @description This module defines the abstract contract for the Secrets service,
 * which provides secure storage capabilities for extensions. It includes the
 * `SecretsProvider` trait and the `ActionEffect` constructors for all secret
 * management operations.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod SecretsProvider;
pub use self::SecretsProvider::SecretsProvider;

// --- Effect Constructors ---
mod DeleteSecret;
mod GetSecret;
mod StoreSecret;

pub use self::DeleteSecret::DeleteSecret;
pub use self::GetSecret::GetSecret;
pub use self::StoreSecret::StoreSecret;
