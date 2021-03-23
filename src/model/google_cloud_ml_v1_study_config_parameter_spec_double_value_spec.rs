
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec {
	/// Must be specified if type is `DOUBLE`. Minimum value of the parameter.
	pub min_value: Option<f64>,
	/// Must be specified if type is `DOUBLE`. Maximum value of the parameter.
	pub max_value: Option<f64>,
}


