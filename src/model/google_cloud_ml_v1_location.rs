use crate::model::google_cloud_ml_v1_capability::GoogleCloudMlV1__Capability;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Location {
	pub name: Option<String>,
	/// Capabilities available in the location.
	pub capabilities: Option<Vec<GoogleCloudMlV1__Capability>>,
}


