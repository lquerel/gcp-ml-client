//! Represents the spec to match integer values from parent parameter.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec {
	/// Matches values of the parent parameter with type 'INTEGER'. All values must lie in `integer_value_spec` of parent parameter.
	pub values: Option<Vec<i64>>,
}


