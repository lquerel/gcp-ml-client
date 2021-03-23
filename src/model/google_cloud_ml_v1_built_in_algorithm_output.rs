//! Represents output related to a built-in algorithm Job.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__BuiltInAlgorithmOutput {
	/// AI Platform runtime version on which the built-in algorithm was trained.
	pub runtime_version: Option<String>,
	/// Python version on which the built-in algorithm was trained.
	pub python_version: Option<String>,
	/// Framework on which the built-in algorithm was trained.
	pub framework: Option<String>,
	/// The Cloud Storage path to the `model/` directory where the training job saves the trained model. Only set for successful jobs that don't use hyperparameter tuning.
	pub model_path: Option<String>,
}


