//! Message holding configuration options for explaining model predictions. There are three feature attribution methods supported for TensorFlow models: integrated gradients, sampled Shapley, and XRAI. [Learn more about feature attributions.](/ai-platform/prediction/docs/ai-explanations/overview)
use crate::model::google_cloud_ml_v1_xrai_attribution::GoogleCloudMlV1__XraiAttribution;
use crate::model::google_cloud_ml_v1_sampled_shapley_attribution::GoogleCloudMlV1__SampledShapleyAttribution;
use crate::model::google_cloud_ml_v1_integrated_gradients_attribution::GoogleCloudMlV1__IntegratedGradientsAttribution;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ExplanationConfig {
	/// Attributes credit by computing the XRAI taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1906.02825 Currently only implemented for models with natural image inputs.
	pub xrai_attribution: Option<GoogleCloudMlV1__XraiAttribution>,
	/// An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features.
	pub sampled_shapley_attribution: Option<GoogleCloudMlV1__SampledShapleyAttribution>,
	/// Attributes credit by computing the Aumann-Shapley value taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1703.01365
	pub integrated_gradients_attribution: Option<GoogleCloudMlV1__IntegratedGradientsAttribution>,
}


