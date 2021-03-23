//! Represents results of a prediction job.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__PredictionOutput {
	/// Node hours used by the batch prediction job.
	pub node_hours: Option<f64>,
	/// The output Google Cloud Storage location provided at the job creation time.
	pub output_path: Option<String>,
	/// The number of generated predictions.
	pub prediction_count: Option<i64>,
	/// The number of data instances which resulted in errors.
	pub error_count: Option<i64>,
}


