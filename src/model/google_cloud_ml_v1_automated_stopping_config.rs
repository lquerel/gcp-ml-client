//! Configuration for Automated Early Stopping of Trials. If no implementation_config is set, automated early stopping will not be run.
use crate::model::google_cloud_ml_v1_automated_stopping_config_median_automated_stopping_config::GoogleCloudMlV1_AutomatedStoppingConfig_MedianAutomatedStoppingConfig;
use crate::model::google_cloud_ml_v1_automated_stopping_config_decay_curve_automated_stopping_config::GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__AutomatedStoppingConfig {
	pub median_automated_stopping_config: Option<GoogleCloudMlV1_AutomatedStoppingConfig_MedianAutomatedStoppingConfig>,
	pub decay_curve_stopping_config: Option<GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig>,
}


