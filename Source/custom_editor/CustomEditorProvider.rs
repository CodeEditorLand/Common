use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::{environment::Environment, error::CommonError};

#[async_trait]
pub trait CustomEditorProvider: Environment + Send + Sync {
	async fn RegisterCustomEditorProvider(
		&self,
		ViewType:String,
		Options:Value, // DTO for options
	) -> Result<(), CommonError>;

	async fn UnregisterCustomEditorProvider(&self, ViewType:String) -> Result<(), CommonError>;

	// Called from Cocoon to Mountain
	async fn OnSaveCustomDocument(&self, ViewType:String, ResourceUri:Url) -> Result<(), CommonError>;

	// Called from Mountain to Cocoon
	async fn ResolveCustomEditor(
		&self,
		ViewType:String,
		ResourceUri:Url,
		WebviewPanelHandle:String,
	) -> Result<(), CommonError>;
}
