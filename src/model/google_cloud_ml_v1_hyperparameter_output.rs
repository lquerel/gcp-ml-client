//! Represents the result of a single hyperparameter tuning trial from a training job. The TrainingOutput object that is returned on successful completion of a training job with hyperparameter tuning includes a list of HyperparameterOutput objects, one for each successful trial.
use crate::model::google_cloud_ml_v1_hyperparameter_output_hyperparameter_metric::GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric;
use std::collections::HashMap;
use crate::model::google_cloud_ml_v1_built_in_algorithm_output::GoogleCloudMlV1__BuiltInAlgorithmOutput;
use chrono::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__HyperparameterOutput {
	/// All recorded object metrics for this trial. This field is not currently populated.
	pub all_metrics: Option<Vec<GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric>>,
	/// The trial id for these results.
	pub trial_id: Option<String>,
	/// Details related to built-in algorithms jobs. Only set for trials of built-in algorithms jobs that have succeeded.
	pub built_in_algorithm_output: Option<GoogleCloudMlV1__BuiltInAlgorithmOutput>,
	/// Output only. End time for the trial.
	pub end_time: Option<DateTime<Utc>>,
	/// The hyperparameters given to this trial.
	pub hyperparameters: Option<HashMap<String, String>>,
	/// Output only. Start time for the trial.
	pub start_time: Option<DateTime<Utc>>,
	/// The final objective metric seen for this trial.
	pub final_metric: Option<GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric>,
	/// Output only. The detailed state of the trial.
	pub state: Option<State>,
	/// True if the trial is stopped early.
	pub is_trial_stopped_early: Option<bool>,
}


/// Output only. The detailed state of the trial.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
	/// The job state is unspecified.
	StateUnspecified,
	/// The job has been just created and processing has not yet begun.
	Queued,
	/// The service is preparing to run the job.
	Preparing,
	/// The job is in progress.
	Running,
	/// The job completed successfully.
	Succeeded,
	/// The job failed. `error_message` should contain the details of the failure.
	Failed,
	/// The job is being cancelled. `error_message` should describe the reason for the cancellation.
	Cancelling,
	/// The job has been cancelled. `error_message` should describe the reason for the cancellation.
	Cancelled,
}


