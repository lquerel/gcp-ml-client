//! The response message for the ListOptimalTrials method.
use crate::model::google_cloud_ml_v1_trial::GoogleCloudMlV1__Trial;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListOptimalTrialsResponse {
	/// The pareto-optimal trials for multiple objective study or the optimal trial for single objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency
	pub trials: Option<Vec<GoogleCloudMlV1__Trial>>,
}


