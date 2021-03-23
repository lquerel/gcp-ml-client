//! The response message for Operations.ListOperations.
use crate::model::google_longrunning_operation::GoogleLongrunning__Operation;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleLongrunning__ListOperationsResponse {
	/// A list of operations that matches the specified filter in the request.
	pub operations: Option<Vec<GoogleLongrunning__Operation>>,
	/// The standard List next-page token.
	pub next_page_token: Option<String>,
}


