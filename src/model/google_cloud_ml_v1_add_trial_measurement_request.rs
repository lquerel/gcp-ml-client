//! The request message for the AddTrialMeasurement service method.
use crate::model::google_cloud_ml_v1_measurement::GoogleCloudMlV1__Measurement;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__AddTrialMeasurementRequest {
	/// Required. The measurement to be added to a trial.
	pub measurement: GoogleCloudMlV1__Measurement,
}


