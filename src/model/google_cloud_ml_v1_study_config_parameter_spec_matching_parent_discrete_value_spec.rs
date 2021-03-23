//! Represents the spec to match discrete values from parent parameter.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec {
	/// Matches values of the parent parameter with type 'DISCRETE'. All values must exist in `discrete_value_spec` of parent parameter.
	pub values: Option<Vec<f64>>,
}


