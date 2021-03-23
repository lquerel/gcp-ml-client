//! The response message for the ListTrials method.
use crate::model::google_cloud_ml_v1_trial::GoogleCloudMlV1__Trial;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListTrialsResponse {
	/// The trials associated with the study.
	pub trials: Option<Vec<GoogleCloudMlV1__Trial>>,
}


