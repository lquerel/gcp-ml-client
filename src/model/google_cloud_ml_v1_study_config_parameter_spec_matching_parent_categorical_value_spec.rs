//! Represents the spec to match categorical values from parent parameter.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec {
	/// Matches values of the parent parameter with type 'CATEGORICAL'. All values must exist in `categorical_value_spec` of parent parameter.
	pub values: Option<Vec<String>>,
}


