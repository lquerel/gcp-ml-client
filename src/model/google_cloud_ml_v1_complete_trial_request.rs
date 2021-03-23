//! The request message for the CompleteTrial service method.
use crate::model::google_cloud_ml_v1_measurement::GoogleCloudMlV1__Measurement;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__CompleteTrialRequest {
	/// Optional. A human readable reason why the trial was infeasible. This should only be provided if `trial_infeasible` is true.
	pub infeasible_reason: Option<String>,
	/// Optional. True if the trial cannot be run with the given Parameter, and final_measurement will be ignored.
	pub trial_infeasible: Option<bool>,
	/// Optional. If provided, it will be used as the completed trial's final_measurement; Otherwise, the service will auto-select a previously reported measurement as the final-measurement
	pub final_measurement: Option<GoogleCloudMlV1__Measurement>,
}


