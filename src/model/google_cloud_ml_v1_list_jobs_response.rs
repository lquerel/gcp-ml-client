//! Response message for the ListJobs method.
use crate::model::google_cloud_ml_v1_job::GoogleCloudMlV1__Job;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ListJobsResponse {
	/// The list of jobs.
	pub jobs: Option<Vec<GoogleCloudMlV1__Job>>,
	/// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
	pub next_page_token: Option<String>,
}


