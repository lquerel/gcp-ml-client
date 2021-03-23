//! Represents a hardware accelerator request config. Note that the AcceleratorConfig can be used in both Jobs and Versions. Learn more about [accelerators for training](/ml-engine/docs/using-gpus) and [accelerators for online prediction](/ml-engine/docs/machine-types-online-prediction#gpus).

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__AcceleratorConfig {
	/// The type of accelerator to use.
	pub r#type: Option<Type>,
	/// The number of accelerators to attach to each machine running the job.
	pub count: Option<i64>,
}


/// The type of accelerator to use.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
	/// Unspecified accelerator type. Default to no GPU.
	AcceleratorTypeUnspecified,
	/// Nvidia Tesla K80 GPU.
	NvidiaTeslaK80,
	/// Nvidia Tesla P100 GPU.
	NvidiaTeslaP100,
	/// Nvidia V100 GPU.
	NvidiaTeslaV100,
	/// Nvidia Tesla P4 GPU.
	NvidiaTeslaP4,
	/// Nvidia T4 GPU.
	NvidiaTeslaT4,
	/// Nvidia A100 GPU.
	NvidiaTeslaA100,
	/// TPU v2.
	TpuV2,
	/// TPU v3.
	TpuV3,
}


