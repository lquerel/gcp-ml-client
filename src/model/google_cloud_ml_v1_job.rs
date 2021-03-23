//! Represents a training or prediction job.
use crate::model::google_cloud_ml_v1_prediction_output::GoogleCloudMlV1__PredictionOutput;
use chrono::DateTime;
use crate::model::google_cloud_ml_v1_training_output::GoogleCloudMlV1__TrainingOutput;
use crate::model::google_cloud_ml_v1_prediction_input::GoogleCloudMlV1__PredictionInput;
use chrono::Utc;
use crate::model::google_cloud_ml_v1_training_input::GoogleCloudMlV1__TrainingInput;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Job {
	/// Input parameters to create a prediction job.
	pub prediction_input: Option<GoogleCloudMlV1__PredictionInput>,
	/// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a job from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform job updates in order to avoid race conditions: An `etag` is returned in the response to `GetJob`, and systems are expected to put that etag in the request to `UpdateJob` to ensure that their change will be applied to the same version of the job.
	pub etag: Option<String>,
	/// Output only. When the job processing was started.
	pub start_time: Option<DateTime<Utc>>,
	/// Input parameters to create a training job.
	pub training_input: Option<GoogleCloudMlV1__TrainingInput>,
	/// The current training job result.
	pub training_output: Option<GoogleCloudMlV1__TrainingOutput>,
	/// Output only. When the job processing was completed.
	pub end_time: Option<DateTime<Utc>>,
	/// Output only. When the job was created.
	pub create_time: Option<DateTime<Utc>>,
	/// Output only. The details of a failure or a cancellation.
	pub error_message: Option<String>,
	/// Required. The user-specified id of the job.
	pub job_id: String,
	/// The current prediction job result.
	pub prediction_output: Option<GoogleCloudMlV1__PredictionOutput>,
	/// Output only. The detailed state of a job.
	pub state: Option<State>,
	/// Optional. One or more labels that you can add, to organize your jobs. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels.
	pub labels: Option<HashMap<String, String>>,
}


/// Output only. The detailed state of a job.
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


