//! A message representing a parameter to be tuned. Contains the name of the parameter and the suggested value to use for this trial.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_Trial_Parameter {
	/// Must be set if ParameterTypeis CATEGORICAL
	pub string_value: Option<String>,
	/// The name of the parameter.
	pub parameter: Option<String>,
	/// Must be set if ParameterType is INTEGER
	pub int_value: Option<i64>,
	/// Must be set if ParameterType is DOUBLE or DISCRETE.
	pub float_value: Option<f64>,
}


