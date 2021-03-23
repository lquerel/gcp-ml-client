//! Represents a single parameter to optimize.
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_matching_parent_int_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_discrete_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_DiscreteValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_integer_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_categorical_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_double_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_matching_parent_categorical_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec_matching_parent_discrete_value_spec::GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec::GoogleCloudMlV1_StudyConfig_ParameterSpec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfig_ParameterSpec {
	/// A child node is active if the parameter's value matches the child node's matching_parent_values. If two items in child_parameter_specs have the same name, they must have disjoint matching_parent_values.
	pub child_parameter_specs: Option<Vec<GoogleCloudMlV1_StudyConfig_ParameterSpec>>,
	/// The value spec for a 'CATEGORICAL' parameter.
	pub categorical_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec>,
	/// Required. The type of the parameter.
	pub r#type: Type,
	pub parent_int_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec>,
	/// The value spec for a 'DOUBLE' parameter.
	pub double_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec>,
	/// Required. The parameter name must be unique amongst all ParameterSpecs.
	pub parameter: String,
	pub parent_discrete_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec>,
	/// How the parameter should be scaled. Leave unset for categorical parameters.
	pub scale_type: Option<ScaleType>,
	/// The value spec for an 'INTEGER' parameter.
	pub integer_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec>,
	/// The value spec for a 'DISCRETE' parameter.
	pub discrete_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_DiscreteValueSpec>,
	pub parent_categorical_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec>,
}


/// Required. The type of the parameter.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
	/// You must specify a valid type. Using this unspecified type will result in an error.
	ParameterTypeUnspecified,
	/// Type for real-valued parameters.
	Double,
	/// Type for integral parameters.
	Integer,
	/// The parameter is categorical, with a value chosen from the categories field.
	Categorical,
	/// The parameter is real valued, with a fixed set of feasible points. If `type==DISCRETE`, feasible_points must be provided, and {`min_value`, `max_value`} will be ignored.
	Discrete,
}


/// How the parameter should be scaled. Leave unset for categorical parameters.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScaleType {
	/// By default, no scaling is applied.
	ScaleTypeUnspecified,
	/// Scales the feasible space to (0, 1) linearly.
	UnitLinearScale,
	/// Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive.
	UnitLogScale,
	/// Scales the feasible space "reverse" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive.
	UnitReverseLogScale,
}


