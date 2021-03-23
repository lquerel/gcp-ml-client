use crate::model::google_cloud_ml_v1_location::GoogleCloudMlV1__Location;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListLocationsResponse {
	/// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
	pub next_page_token: Option<String>,
	/// Locations where at least one type of CMLE capability is available.
	pub locations: Option<Vec<GoogleCloudMlV1__Location>>,
}


