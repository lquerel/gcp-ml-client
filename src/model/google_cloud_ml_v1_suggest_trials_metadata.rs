//! Metadata field of a google.longrunning.Operation associated with a SuggestTrialsRequest.
use chrono::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__SuggestTrialsMetadata {
	/// The number of suggestions requested.
	pub suggestion_count: Option<i32>,
	/// The time operation was submitted.
	pub create_time: Option<DateTime<Utc>>,
	/// The identifier of the client that is requesting the suggestion.
	pub client_id: Option<String>,
	/// The name of the study that the trial belongs to.
	pub study: Option<String>,
}


