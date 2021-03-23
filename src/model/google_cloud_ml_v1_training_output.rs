//! Represents results of a training job. Output only.
use crate::model::google_cloud_ml_v1_hyperparameter_output::GoogleCloudMlV1__HyperparameterOutput;
use crate::model::google_cloud_ml_v1_built_in_algorithm_output::GoogleCloudMlV1__BuiltInAlgorithmOutput;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__TrainingOutput {
	/// Whether this job is a built-in Algorithm job.
	pub is_built_in_algorithm_job: Option<bool>,
	/// Results for individual Hyperparameter trials. Only set for hyperparameter tuning jobs.
	pub trials: Option<Vec<GoogleCloudMlV1__HyperparameterOutput>>,
	/// Details related to built-in algorithms jobs. Only set for built-in algorithms jobs.
	pub built_in_algorithm_output: Option<GoogleCloudMlV1__BuiltInAlgorithmOutput>,
	/// Whether this job is a hyperparameter tuning job.
	pub is_hyperparameter_tuning_job: Option<bool>,
	/// The TensorFlow summary tag name used for optimizing hyperparameter tuning trials. See [`HyperparameterSpec.hyperparameterMetricTag`](#HyperparameterSpec.FIELDS.hyperparameter_metric_tag) for more information. Only set for hyperparameter tuning jobs.
	pub hyperparameter_metric_tag: Option<String>,
	/// The amount of ML units consumed by the job.
	pub consumed_ml_units: Option<f64>,
	/// The number of hyperparameter tuning trials that completed successfully. Only set for hyperparameter tuning jobs.
	pub completed_trial_count: Option<i64>,
}


