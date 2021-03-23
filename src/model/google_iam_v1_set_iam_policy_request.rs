//! Request message for `SetIamPolicy` method.
use crate::model::google_iam_v1_policy::GoogleIamV1__Policy;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleIamV1__SetIamPolicyRequest {
	/// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them.
	pub policy: GoogleIamV1__Policy,
	/// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
	pub update_mask: Option<String>,
}


