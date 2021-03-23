use crate::model::google_cloud_ml_v1_study::GoogleCloudMlV1__Study;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListStudiesResponse {
	/// The studies associated with the project.
	pub studies: Option<Vec<GoogleCloudMlV1__Study>>,
}


