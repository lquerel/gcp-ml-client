//! Represents the config of disk options.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__DiskConfig {
	/// Size in GB of the boot disk (default is 100GB).
	pub boot_disk_size_gb: Option<i32>,
	/// Type of the boot disk (default is "pd-ssd"). Valid values: "pd-ssd" (Persistent Disk Solid State Drive) or "pd-standard" (Persistent Disk Hard Disk Drive).
	pub boot_disk_type: Option<String>,
}


