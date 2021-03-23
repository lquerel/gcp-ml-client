//! MetricSpec contains the specifications to use to calculate the desired nodes count when autoscaling is enabled.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__MetricSpec {
	/// metric name.
	pub name: Option<Name>,
	/// Target specifies the target value for the given metric; once real metric deviates from the threshold by a certain percentage, the node count changes.
	pub target: Option<i32>,
}


/// metric name.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Name {
	/// Unspecified MetricName.
	MetricNameUnspecified,
	/// CPU usage.
	CpuUsage,
	/// GPU duty cycle.
	GpuDutyCycle,
}


