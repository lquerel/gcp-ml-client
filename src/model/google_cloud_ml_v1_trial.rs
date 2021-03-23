//! A message representing a trial.
use crate::model::google_cloud_ml_v1_trial_parameter::GoogleCloudMlV1_Trial_Parameter;
use chrono::Utc;
use chrono::DateTime;
use crate::model::google_cloud_ml_v1_measurement::GoogleCloudMlV1__Measurement;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Trial {
	/// Output only. The identifier of the client that originally requested this trial.
	pub client_id: Option<String>,
	/// Output only. Name of the trial assigned by the service.
	pub name: Option<String>,
	/// Output only. Time at which the trial was started.
	pub start_time: Option<DateTime<Utc>>,
	/// A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_time). These are used for early stopping computations.
	pub measurements: Option<Vec<GoogleCloudMlV1__Measurement>>,
	/// The parameters of the trial.
	pub parameters: Option<Vec<GoogleCloudMlV1_Trial_Parameter>>,
	/// The detailed state of a trial.
	pub state: Option<State>,
	/// Output only. A human readable string describing why the trial is infeasible. This should only be set if trial_infeasible is true.
	pub infeasible_reason: Option<String>,
	/// The final measurement containing the objective value.
	pub final_measurement: Option<GoogleCloudMlV1__Measurement>,
	/// Output only. If true, the parameters in this trial are not attempted again.
	pub trial_infeasible: Option<bool>,
	/// Output only. Time at which the trial's status changed to COMPLETED.
	pub end_time: Option<DateTime<Utc>>,
}


/// The detailed state of a trial.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
	/// The trial state is unspecified.
	StateUnspecified,
	/// Indicates that a specific trial has been requested, but it has not yet been suggested by the service.
	Requested,
	/// Indicates that the trial has been suggested.
	Active,
	/// Indicates that the trial is done, and either has a final_measurement set, or is marked as trial_infeasible.
	Completed,
	/// Indicates that the trial should stop according to the service.
	Stopping,
}


