//! Represents a metric to optimize.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1_StudyConfig_MetricSpec {
	/// Required. The optimization goal of the metric.
	pub goal: Goal,
	/// Required. The name of the metric.
	pub metric: String,
}


/// Required. The optimization goal of the metric.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Goal {
	/// Goal Type will default to maximize.
	GoalTypeUnspecified,
	/// Maximize the goal metric.
	Maximize,
	/// Minimize the goal metric.
	Minimize,
}


