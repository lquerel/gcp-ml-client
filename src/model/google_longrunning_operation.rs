//! This resource represents a long-running operation that is the result of a network API call.
use crate::model::google_rpc_status::GoogleRpc__Status;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleLongrunning__Operation {
	/// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
	pub done: Option<bool>,
	/// The error result of the operation in case of failure or cancellation.
	pub error: Option<GoogleRpc__Status>,
	/// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
	pub name: Option<String>,
	/// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
	pub metadata: Option<HashMap<String, String>>,
	/// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
	pub response: Option<HashMap<String, String>>,
}


