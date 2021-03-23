//! Represents a set of hyperparameters to optimize.
use crate::model::google_cloud_ml_v1_parameter_spec::GoogleCloudMlV1__ParameterSpec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__HyperparameterSpec {
	/// Optional. The number of failed trials that need to be seen before failing the hyperparameter tuning job. You can specify this field to override the default failing criteria for AI Platform hyperparameter tuning jobs. Defaults to zero, which means the service decides when a hyperparameter job should fail.
	pub max_failed_trials: Option<i32>,
	/// Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified.
	pub algorithm: Option<Algorithm>,
	/// Optional. The TensorFlow summary tag name to use for optimizing trials. For current versions of TensorFlow, this tag name should exactly match what is shown in TensorBoard, including all scopes. For versions of TensorFlow prior to 0.12, this should be only the tag passed to tf.Summary. By default, "training/hptuning/metric" will be used.
	pub hyperparameter_metric_tag: Option<String>,
	/// Optional. The prior hyperparameter tuning job id that users hope to continue with. The job id will be used to find the corresponding vizier study guid and resume the study.
	pub resume_previous_job_id: Option<String>,
	/// Optional. Indicates if the hyperparameter tuning job enables auto trial early stopping.
	pub enable_trial_early_stopping: Option<bool>,
	/// Optional. How many training trials should be attempted to optimize the specified hyperparameters. Defaults to one.
	pub max_trials: Option<i32>,
	/// Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`.
	pub goal: Goal,
	/// Optional. The number of training trials to run concurrently. You can reduce the time it takes to perform hyperparameter tuning by adding trials in parallel. However, each trail only benefits from the information gained in completed trials. That means that a trial does not get access to the results of trials running at the same time, which could reduce the quality of the overall optimization. Each trial will use the same scale tier and machine types. Defaults to one.
	pub max_parallel_trials: Option<i32>,
	/// Required. The set of parameters to tune.
	pub params: Vec<GoogleCloudMlV1__ParameterSpec>,
}


/// Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Algorithm {
	/// The default algorithm used by the hyperparameter tuning service. This is a Bayesian optimization algorithm.
	AlgorithmUnspecified,
	/// Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`.
	GridSearch,
	/// Simple random search within the feasible space.
	RandomSearch,
}


/// Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Goal {
	/// Goal Type will default to maximize.
	GoalTypeUnspecified,
	/// Maximize the goal metric.
	Maximize,
	/// Minimize the goal metric.
	Minimize,
}


