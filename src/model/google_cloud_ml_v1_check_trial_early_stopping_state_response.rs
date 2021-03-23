//! The message will be placed in the response field of a completed google.longrunning.Operation associated with a CheckTrialEarlyStoppingState request.
use chrono::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__CheckTrialEarlyStoppingStateResponse {
	/// The time at which the operation was started.
	pub start_time: Option<DateTime<Utc>>,
	/// The time at which operation processing completed.
	pub end_time: Option<DateTime<Utc>>,
	/// True if the Trial should stop.
	pub should_stop: Option<bool>,
}


