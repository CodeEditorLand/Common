

/**
 * @module dto (Webview)
 * @description Aggregates and re-exports all Data Transfer Objects (DTOs)
 * related to the Webview and WebviewPanel APIs.
 */

#![allow(non_snake_case, non_camel_case_types)]

mod WebviewContentOptionsDto;
mod WebviewExtensionDescriptionDto;
mod WebviewPanelOptionsDto;
mod WebviewPanelViewStateDto;
mod WebviewShowOptionsDto;

pub use self::WebviewContentOptionsDto::WebviewContentOptionsDto;
pub use self::WebviewExtensionDescriptionDto::WebviewExtensionDescriptionDto;
pub use self::WebviewPanelOptionsDto::WebviewPanelOptionsDto;
pub use self::WebviewPanelViewStateDto::WebviewPanelViewStateDto;
pub use self::WebviewShowOptionsDto::WebviewShowOptionsDto;
