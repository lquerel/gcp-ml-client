//! This message will be placed in the metadata field of a google.longrunning.Operation associated with a CheckTrialEarlyStoppingState request.
use chrono::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__CheckTrialEarlyStoppingStateMetatdata {
	/// The name of the study that the trial belongs to.
	pub study: Option<String>,
	/// The time at which the operation was submitted.
	pub create_time: Option<DateTime<Utc>>,
	/// The trial name.
	pub trial: Option<String>,
}


