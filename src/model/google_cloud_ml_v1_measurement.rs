//! A message representing a measurement.
use crate::model::google_cloud_ml_v1_measurement_metric::GoogleCloudMlV1_Measurement_Metric;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Measurement {
	/// The number of steps a machine learning model has been trained for. Must be non-negative.
	pub step_count: Option<i64>,
	/// Provides a list of metrics that act as inputs into the objective function.
	pub metrics: Option<Vec<GoogleCloudMlV1_Measurement_Metric>>,
	/// Output only. Time that the trial has been running at the point of this measurement.
	pub elapsed_time: Option<String>,
}


