//! Represents a network port in a single container. This message is a subset of the [Kubernetes ContainerPort v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#containerport-v1-core).

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__ContainerPort {
	/// Number of the port to expose on the container. This must be a valid port number: 0 < PORT_NUMBER < 65536.
	pub container_port: Option<i32>,
}


