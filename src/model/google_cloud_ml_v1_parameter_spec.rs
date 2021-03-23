//! Represents a single hyperparameter to optimize.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ParameterSpec {
	/// Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`).
	pub scale_type: Option<ScaleType>,
	/// Required if type is `CATEGORICAL`. The list of possible categories.
	pub categorical_values: Option<Vec<String>>,
	/// Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is `INTEGER`.
	pub max_value: Option<f64>,
	/// Required if type is `DISCRETE`. A list of feasible points. The list should be in strictly increasing order. For instance, this parameter might have possible settings of 1.5, 2.5, and 4.0. This list should not contain more than 1,000 values.
	pub discrete_values: Option<Vec<f64>>,
	/// Required. The parameter name must be unique amongst all ParameterConfigs in a HyperparameterSpec message. E.g., "learning_rate".
	pub parameter_name: String,
	/// Required. The type of the parameter.
	pub r#type: Type,
	/// Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is INTEGER.
	pub min_value: Option<f64>,
}


/// Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScaleType {
	/// By default, no scaling is applied.
	None,
	/// Scales the feasible space to (0, 1) linearly.
	UnitLinearScale,
	/// Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive.
	UnitLogScale,
	/// Scales the feasible space "reverse" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive.
	UnitReverseLogScale,
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


