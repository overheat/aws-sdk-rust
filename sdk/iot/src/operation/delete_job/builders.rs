// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_job::_delete_job_output::DeleteJobOutputBuilder;

pub use crate::operation::delete_job::_delete_job_input::DeleteJobInputBuilder;

impl DeleteJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_job::DeleteJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_job::DeleteJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteJob`.
///
/// <p>Deletes a job and its related job executions.</p>
/// <p>Deleting a job may take time, depending on the number of job executions created for the job and various other factors. While the job is being deleted, the status of the job will be shown as "DELETION_IN_PROGRESS". Attempting to delete or cancel a job whose status is already "DELETION_IN_PROGRESS" will result in an error.</p>
/// <p>Only 10 jobs may have status "DELETION_IN_PROGRESS" at the same time, or a LimitExceededException will occur.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeleteJob</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_job::builders::DeleteJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::delete_job::DeleteJobOutput, crate::operation::delete_job::DeleteJobError>
    for DeleteJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::delete_job::DeleteJobOutput, crate::operation::delete_job::DeleteJobError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteJobFluentBuilder {
    /// Creates a new `DeleteJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteJob as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_job::builders::DeleteJobInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_job::DeleteJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_job::DeleteJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_job::DeleteJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_job::DeleteJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_job::DeleteJobOutput,
        crate::operation::delete_job::DeleteJobError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the job to be deleted.</p>
    /// <p>After a job deletion is completed, you may reuse this jobId when you create a new job. However, this is not recommended, and you must ensure that your devices are not using the jobId to refer to the deleted job.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The ID of the job to be deleted.</p>
    /// <p>After a job deletion is completed, you may reuse this jobId when you create a new job. However, this is not recommended, and you must ensure that your devices are not using the jobId to refer to the deleted job.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The ID of the job to be deleted.</p>
    /// <p>After a job deletion is completed, you may reuse this jobId when you create a new job. However, this is not recommended, and you must ensure that your devices are not using the jobId to refer to the deleted job.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
    /// <p>(Optional) When true, you can delete a job which is "IN_PROGRESS". Otherwise, you can only delete a job which is in a terminal state ("COMPLETED" or "CANCELED") or an exception will occur. The default is false.</p> <note>
    /// <p>Deleting a job which is "IN_PROGRESS", will cause a device which is executing the job to be unable to access job information or update the job execution status. Use caution and ensure that each device executing a job which is deleted is able to recover to a valid state.</p>
    /// </note>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>(Optional) When true, you can delete a job which is "IN_PROGRESS". Otherwise, you can only delete a job which is in a terminal state ("COMPLETED" or "CANCELED") or an exception will occur. The default is false.</p> <note>
    /// <p>Deleting a job which is "IN_PROGRESS", will cause a device which is executing the job to be unable to access job information or update the job execution status. Use caution and ensure that each device executing a job which is deleted is able to recover to a valid state.</p>
    /// </note>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>(Optional) When true, you can delete a job which is "IN_PROGRESS". Otherwise, you can only delete a job which is in a terminal state ("COMPLETED" or "CANCELED") or an exception will occur. The default is false.</p> <note>
    /// <p>Deleting a job which is "IN_PROGRESS", will cause a device which is executing the job to be unable to access job information or update the job execution status. Use caution and ensure that each device executing a job which is deleted is able to recover to a valid state.</p>
    /// </note>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        self.inner.get_force()
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p> <code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code> </p> <note>
    /// <p>The <code>namespaceId</code> feature is in public preview.</p>
    /// </note>
    pub fn namespace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace_id(input.into());
        self
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p> <code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code> </p> <note>
    /// <p>The <code>namespaceId</code> feature is in public preview.</p>
    /// </note>
    pub fn set_namespace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_id(input);
        self
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p> <code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code> </p> <note>
    /// <p>The <code>namespaceId</code> feature is in public preview.</p>
    /// </note>
    pub fn get_namespace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace_id()
    }
}
