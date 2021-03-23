//! Represents input parameters for a prediction job.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudMlV1__PredictionInput {
	/// Use this field if you want to specify a version of the model to use. The string is formatted the same way as `model_version`, with the addition of the version information: `"projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION"`
	pub version_name: Option<String>,
	/// Required. The output Google Cloud Storage location.
	pub output_path: String,
	/// Required. The Cloud Storage location of the input data files. May contain wildcards.
	pub input_paths: Vec<String>,
	/// Optional. The name of the signature defined in the SavedModel to use for this job. Please refer to [SavedModel](https://tensorflow.github.io/serving/serving_basic.html) for information about how to use signatures. Defaults to [DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants) , which is "serving_default".
	pub signature_name: Option<String>,
	/// Optional. Format of the output data files, defaults to JSON.
	pub output_data_format: Option<OutputDataFormat>,
	/// Required. The format of the input data files.
	pub data_format: DataFormat,
	/// Optional. The AI Platform runtime version to use for this batch prediction. If not set, AI Platform will pick the runtime version used during the CreateVersion request for this model version, or choose the latest stable version when model version information is not available such as when the model is specified by uri.
	pub runtime_version: Option<String>,
	/// Use this field if you want to use the default version for the specified model. The string must use the following format: `"projects/YOUR_PROJECT/models/YOUR_MODEL"`
	pub model_name: Option<String>,
	/// Optional. The maximum number of workers to be used for parallel processing. Defaults to 10 if not specified.
	pub max_worker_count: Option<i64>,
	/// Use this field if you want to specify a Google Cloud Storage path for the model to use.
	pub uri: Option<String>,
	/// Required. The Google Compute Engine region to run the prediction job in. See the available regions for AI Platform services.
	pub region: String,
	/// Optional. Number of records per batch, defaults to 64. The service will buffer batch_size number of records in memory before invoking one Tensorflow prediction call internally. So take the record size and memory available into consideration when setting this parameter.
	pub batch_size: Option<i64>,
}


/// Optional. Format of the output data files, defaults to JSON.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutputDataFormat {
	/// Unspecified format.
	DataFormatUnspecified,
	/// Each line of the file is a JSON dictionary representing one record.
	Json,
	/// Deprecated. Use JSON instead.
	Text,
	/// The source file is a TFRecord file. Currently available only for input data.
	TfRecord,
	/// The source file is a GZIP-compressed TFRecord file. Currently available only for input data.
	TfRecordGzip,
	/// Values are comma-separated rows, with keys in a separate file. Currently available only for output data.
	Csv,
}


/// Required. The format of the input data files.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataFormat {
	/// Unspecified format.
	DataFormatUnspecified,
	/// Each line of the file is a JSON dictionary representing one record.
	Json,
	/// Deprecated. Use JSON instead.
	Text,
	/// The source file is a TFRecord file. Currently available only for input data.
	TfRecord,
	/// The source file is a GZIP-compressed TFRecord file. Currently available only for input data.
	TfRecordGzip,
	/// Values are comma-separated rows, with keys in a separate file. Currently available only for output data.
	Csv,
}


