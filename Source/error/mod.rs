

//
// @module error
// @description This module defines the universal, structured error type for the
// application and aggregates all error-related exports.
//

#![allow(non_snake_case, non_camel_case_types)]

mod CommonError;

//
// The primary, comprehensive error enum for all operations within the Common
// crate and the applications that use it.
// @see CommonError
//
pub use self::CommonError::CommonError;
