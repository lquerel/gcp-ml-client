
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Config {
	/// The service account Cloud ML uses to run on TPU node.
	pub tpu_service_account: Option<String>,
}


