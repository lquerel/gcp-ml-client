//! An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__SampledShapleyAttribution {
	/// The number of feature permutations to consider when approximating the Shapley values.
	pub num_paths: Option<i32>,
}


