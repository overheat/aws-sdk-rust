// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetJob`](crate::operation::get_job::builders::GetJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_arn(impl Into<String>)`](crate::operation::get_job::builders::GetJobFluentBuilder::job_arn) / [`set_job_arn(Option<String>)`](crate::operation::get_job::builders::GetJobFluentBuilder::set_job_arn):<br>required: **true**<br><p>The ARN of the job to retrieve.</p><br>
    ///   - [`additional_attribute_names(HybridJobAdditionalAttributeName)`](crate::operation::get_job::builders::GetJobFluentBuilder::additional_attribute_names) / [`set_additional_attribute_names(Option<Vec::<HybridJobAdditionalAttributeName>>)`](crate::operation::get_job::builders::GetJobFluentBuilder::set_additional_attribute_names):<br>required: **false**<br><p>A list of attributes to return information for.</p><br>
    /// - On success, responds with [`GetJobOutput`](crate::operation::get_job::GetJobOutput) with field(s):
    ///   - [`status(JobPrimaryStatus)`](crate::operation::get_job::GetJobOutput::status): <p>The status of the Amazon Braket job.</p>
    ///   - [`job_arn(String)`](crate::operation::get_job::GetJobOutput::job_arn): <p>The ARN of the Amazon Braket job.</p>
    ///   - [`role_arn(String)`](crate::operation::get_job::GetJobOutput::role_arn): <p>The Amazon Resource Name (ARN) of an IAM role that Amazon Braket can assume to perform tasks on behalf of a user. It can access user resources, run an Amazon Braket job container on behalf of user, and output resources to the s3 buckets of a user.</p>
    ///   - [`failure_reason(Option<String>)`](crate::operation::get_job::GetJobOutput::failure_reason): <p>A description of the reason why an Amazon Braket job failed, if it failed.</p>
    ///   - [`job_name(String)`](crate::operation::get_job::GetJobOutput::job_name): <p>The name of the Amazon Braket job.</p>
    ///   - [`hyper_parameters(Option<HashMap::<String, String>>)`](crate::operation::get_job::GetJobOutput::hyper_parameters): <p>Algorithm-specific parameters used by an Amazon Braket job that influence the quality of the traiing job. The values are set with a string of JSON key:value pairs, where the key is the name of the hyperparameter and the value is the value of th hyperparameter.</p>
    ///   - [`input_data_config(Option<Vec::<InputFileConfig>>)`](crate::operation::get_job::GetJobOutput::input_data_config): <p>A list of parameters that specify the name and type of input data and where it is located.</p>
    ///   - [`output_data_config(Option<JobOutputDataConfig>)`](crate::operation::get_job::GetJobOutput::output_data_config): <p>The path to the S3 location where job artifacts are stored and the encryption key used to store them there.</p>
    ///   - [`stopping_condition(Option<JobStoppingCondition>)`](crate::operation::get_job::GetJobOutput::stopping_condition): <p>The user-defined criteria that specifies when to stop a job running.</p>
    ///   - [`checkpoint_config(Option<JobCheckpointConfig>)`](crate::operation::get_job::GetJobOutput::checkpoint_config): <p>Information about the output locations for job checkpoint data.</p>
    ///   - [`algorithm_specification(Option<AlgorithmSpecification>)`](crate::operation::get_job::GetJobOutput::algorithm_specification): <p>Definition of the Amazon Braket job created. Specifies the container image the job uses, information about the Python scripts used for entry and training, and the user-defined metrics used to evaluation the job.</p>
    ///   - [`instance_config(Option<InstanceConfig>)`](crate::operation::get_job::GetJobOutput::instance_config): <p>The resource instances to use while running the hybrid job on Amazon Braket.</p>
    ///   - [`created_at(DateTime)`](crate::operation::get_job::GetJobOutput::created_at): <p>The date and time that the Amazon Braket job was created.</p>
    ///   - [`started_at(Option<DateTime>)`](crate::operation::get_job::GetJobOutput::started_at): <p>The date and time that the Amazon Braket job was started.</p>
    ///   - [`ended_at(Option<DateTime>)`](crate::operation::get_job::GetJobOutput::ended_at): <p>The date and time that the Amazon Braket job ended.</p>
    ///   - [`billable_duration(Option<i32>)`](crate::operation::get_job::GetJobOutput::billable_duration): <p>The billable time the Amazon Braket job used to complete.</p>
    ///   - [`device_config(Option<DeviceConfig>)`](crate::operation::get_job::GetJobOutput::device_config): <p>The quantum processing unit (QPU) or simulator used to run the Amazon Braket job.</p>
    ///   - [`events(Option<Vec::<JobEventDetails>>)`](crate::operation::get_job::GetJobOutput::events): <p>Details about the type and time events occurred related to the Amazon Braket job.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_job::GetJobOutput::tags): <p>A tag object that consists of a key and an optional value, used to manage metadata for Amazon Braket resources.</p>
    ///   - [`queue_info(Option<HybridJobQueueInfo>)`](crate::operation::get_job::GetJobOutput::queue_info): <p>Queue information for the requested job. Only returned if <code>QueueInfo</code> is specified in the <code>additionalAttributeNames"</code> field in the <code>GetJob</code> API request.</p>
    /// - On failure, responds with [`SdkError<GetJobError>`](crate::operation::get_job::GetJobError)
    pub fn get_job(&self) -> crate::operation::get_job::builders::GetJobFluentBuilder {
        crate::operation::get_job::builders::GetJobFluentBuilder::new(self.handle.clone())
    }
}
