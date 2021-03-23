//! Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleIamV1__AuditLogConfig {
	/// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
	pub exempted_members: Option<Vec<String>>,
	/// The log type that this config enables.
	pub log_type: Option<LogType>,
}


/// The log type that this config enables.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogType {
	/// Default case. Should never be this.
	LogTypeUnspecified,
	/// Admin reads. Example: CloudIAM getIamPolicy
	AdminRead,
	/// Data writes. Example: CloudSQL Users create
	DataWrite,
	/// Data reads. Example: CloudSQL Users list
	DataRead,
}


