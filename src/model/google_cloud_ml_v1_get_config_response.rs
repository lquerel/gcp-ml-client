//! Returns service account information associated with a project.
use crate::model::google_cloud_ml_v1_config::GoogleCloudMlV1__Config;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__GetConfigResponse {
	pub config: Option<GoogleCloudMlV1__Config>,
	/// The service account Cloud ML uses to access resources in the project.
	pub service_account: Option<String>,
	/// The project number for `service_account`.
	pub service_account_project: Option<i64>,
}


