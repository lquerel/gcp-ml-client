//! An observed value of a metric.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric {
	/// The global training step for this metric.
	pub training_step: Option<i64>,
	/// The objective value at this training step.
	pub objective_value: Option<f64>,
}


