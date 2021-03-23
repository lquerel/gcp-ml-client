//! Request for predictions to be issued against a trained model.
use crate::model::google_api_http_body::GoogleApi__HttpBody;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__PredictRequest {
	///  Required. The prediction request body. Refer to the [request body details section](#request-body-details) for more information on how to structure your request.
	pub http_body: GoogleApi__HttpBody,
}


