//! Represents the metadata of the long-running operation.
use chrono::Utc;
use std::collections::HashMap;
use crate::model::google_cloud_ml_v1_version::GoogleCloudMlV1__Version;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__OperationMetadata {
	/// Contains the project number associated with the operation.
	pub project_number: Option<i64>,
	/// The operation type.
	pub operation_type: Option<OperationType>,
	/// The user labels, inherited from the model or the model version being operated on.
	pub labels: Option<HashMap<String, String>>,
	/// Indicates whether a request to cancel this operation has been made.
	pub is_cancellation_requested: Option<bool>,
	/// The time operation processing completed.
	pub end_time: Option<DateTime<Utc>>,
	/// The time operation processing started.
	pub start_time: Option<DateTime<Utc>>,
	/// Contains the name of the model associated with the operation.
	pub model_name: Option<String>,
	/// Contains the version associated with the operation.
	pub version: Option<GoogleCloudMlV1__Version>,
	/// The time the operation was submitted.
	pub create_time: Option<DateTime<Utc>>,
}


/// The operation type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationType {
	/// Unspecified operation type.
	OperationTypeUnspecified,
	/// An operation to create a new version.
	CreateVersion,
	/// An operation to delete an existing version.
	DeleteVersion,
	/// An operation to delete an existing model.
	DeleteModel,
	/// An operation to update an existing model.
	UpdateModel,
	/// An operation to update an existing version.
	UpdateVersion,
	/// An operation to update project configuration.
	UpdateConfig,
}


