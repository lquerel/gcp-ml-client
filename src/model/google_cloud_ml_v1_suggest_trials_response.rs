//! This message will be placed in the response field of a completed google.longrunning.Operation associated with a SuggestTrials request.
use crate::model::google_cloud_ml_v1_trial::GoogleCloudMlV1__Trial;
use chrono::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__SuggestTrialsResponse {
	/// A list of trials.
	pub trials: Option<Vec<GoogleCloudMlV1__Trial>>,
	/// The time at which the operation was started.
	pub start_time: Option<DateTime<Utc>>,
	/// The state of the study.
	pub study_state: Option<StudyState>,
	/// The time at which operation processing completed.
	pub end_time: Option<DateTime<Utc>>,
}


/// The state of the study.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StudyState {
	/// The study state is unspecified.
	StateUnspecified,
	/// The study is active.
	Active,
	/// The study is stopped due to an internal error.
	Inactive,
	/// The study is done when the service exhausts the parameter search space or max_trial_count is reached.
	Completed,
}


