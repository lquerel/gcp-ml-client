
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec {
	/// Must be specified if type is `CATEGORICAL`. The list of possible categories.
	pub values: Option<Vec<String>>,
}


