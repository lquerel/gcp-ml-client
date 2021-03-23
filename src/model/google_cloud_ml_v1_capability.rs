
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__Capability {
	/// Available accelerators for the capability.
	pub available_accelerators: Option<Vec<AvailableAccelerators>>,
	pub r#type: Option<Type>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvailableAccelerators {
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


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
	/// 
	TypeUnspecified,
	/// 
	Training,
	/// 
	BatchPrediction,
	/// 
	OnlinePrediction,
}


