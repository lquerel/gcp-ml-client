
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec {
	/// Must be specified if type is `INTEGER`. Minimum value of the parameter.
	pub min_value: Option<i64>,
	/// Must be specified if type is `INTEGER`. Maximum value of the parameter.
	pub max_value: Option<i64>,
}


