//! Request for explanations to be issued against a trained model.
use crate::model::google_api_http_body::GoogleApi__HttpBody;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ExplainRequest {
	/// Required. The explanation request body.
	pub http_body: GoogleApi__HttpBody,
}


