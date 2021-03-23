//! A message representing a Study.
use chrono::Utc;
use crate::model::google_cloud_ml_v1_study_config::GoogleCloudMlV1__StudyConfig;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Study {
	/// Output only. Time at which the study was created.
	pub create_time: Option<DateTime<Utc>>,
	/// Output only. The name of a study.
	pub name: Option<String>,
	/// Required. Configuration of the study.
	pub study_config: GoogleCloudMlV1__StudyConfig,
	/// Output only. The detailed state of a study.
	pub state: Option<State>,
	/// Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED.
	pub inactive_reason: Option<String>,
}


/// Output only. The detailed state of a study.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
	/// The study state is unspecified.
	StateUnspecified,
	/// The study is active.
	Active,
	/// The study is stopped due to an internal error.
	Inactive,
	/// The study is done when the service exhausts the parameter search space or max_trial_count is reached.
	Completed,
}


