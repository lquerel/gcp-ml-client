//! Response message for the ListModels method.
use crate::model::google_cloud_ml_v1_model::GoogleCloudMlV1__Model;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListModelsResponse {
	/// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
	pub next_page_token: Option<String>,
	/// The list of models.
	pub models: Option<Vec<GoogleCloudMlV1__Model>>,
}


