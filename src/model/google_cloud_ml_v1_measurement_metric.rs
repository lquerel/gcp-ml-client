//! A message representing a metric in the measurement.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_Measurement_Metric {
	/// Required. The value for this metric.
	pub value: f64,
	/// Required. Metric name.
	pub metric: String,
}


