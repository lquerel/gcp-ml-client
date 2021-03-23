
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig {
	/// If true, measurement.elapsed_time is used as the x-axis of each Trials Decay Curve. Otherwise, Measurement.steps will be used as the x-axis.
	pub use_elapsed_time: Option<bool>,
}


