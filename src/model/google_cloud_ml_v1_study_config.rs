//! Represents configuration of a study.
use crate::model::google_cloud_ml_v1_study_config_metric_spec::GoogleCloudMlV1_StudyConfig_MetricSpec;
use crate::model::google_cloud_ml_v1_study_config_parameter_spec::GoogleCloudMlV1_StudyConfig_ParameterSpec;
use crate::model::google_cloud_ml_v1_automated_stopping_config::GoogleCloudMlV1__AutomatedStoppingConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__StudyConfig {
	/// The search algorithm specified for the study.
	pub algorithm: Option<Algorithm>,
	/// Configuration for automated stopping of unpromising Trials.
	pub automated_stopping_config: Option<GoogleCloudMlV1__AutomatedStoppingConfig>,
	/// Metric specs for the study.
	pub metrics: Option<Vec<GoogleCloudMlV1_StudyConfig_MetricSpec>>,
	/// Required. The set of parameters to tune.
	pub parameters: Vec<GoogleCloudMlV1_StudyConfig_ParameterSpec>,
}


/// The search algorithm specified for the study.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Algorithm {
	/// The default algorithm used by the Cloud AI Platform Vizier service.
	AlgorithmUnspecified,
	/// Gaussian Process Bandit.
	GaussianProcessBandit,
	/// Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`.
	GridSearch,
	/// Simple random search within the feasible space.
	RandomSearch,
}


