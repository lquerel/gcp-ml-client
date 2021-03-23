//! Response message for the ListVersions method.
use crate::model::google_cloud_ml_v1_version::GoogleCloudMlV1__Version;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListVersionsResponse {
	/// The list of versions.
	pub versions: Option<Vec<GoogleCloudMlV1__Version>>,
	/// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
	pub next_page_token: Option<String>,
}


