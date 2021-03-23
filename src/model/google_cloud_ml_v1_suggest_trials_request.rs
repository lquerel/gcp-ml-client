//! The request message for the SuggestTrial service method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__SuggestTrialsRequest {
	/// Required. The number of suggestions requested.
	pub suggestion_count: i32,
	/// Required. The identifier of the client that is requesting the suggestion. If multiple SuggestTrialsRequests have the same `client_id`, the service will return the identical suggested trial if the trial is pending, and provide a new trial if the last suggested trial was completed.
	pub client_id: String,
}


