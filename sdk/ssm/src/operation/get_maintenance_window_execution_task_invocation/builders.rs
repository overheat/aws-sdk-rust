// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_maintenance_window_execution_task_invocation::_get_maintenance_window_execution_task_invocation_output::GetMaintenanceWindowExecutionTaskInvocationOutputBuilder;

pub use crate::operation::get_maintenance_window_execution_task_invocation::_get_maintenance_window_execution_task_invocation_input::GetMaintenanceWindowExecutionTaskInvocationInputBuilder;

impl GetMaintenanceWindowExecutionTaskInvocationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_maintenance_window_execution_task_invocation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMaintenanceWindowExecutionTaskInvocation`.
///
/// <p>Retrieves information about a specific task running on a specific target.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMaintenanceWindowExecutionTaskInvocationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_maintenance_window_execution_task_invocation::builders::GetMaintenanceWindowExecutionTaskInvocationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationOutput,
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationError,
    > for GetMaintenanceWindowExecutionTaskInvocationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationOutput,
            crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMaintenanceWindowExecutionTaskInvocationFluentBuilder {
    /// Creates a new `GetMaintenanceWindowExecutionTaskInvocation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMaintenanceWindowExecutionTaskInvocation as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::get_maintenance_window_execution_task_invocation::builders::GetMaintenanceWindowExecutionTaskInvocationInputBuilder {
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
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocation::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocation::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationOutput,
        crate::operation::get_maintenance_window_execution_task_invocation::GetMaintenanceWindowExecutionTaskInvocationError,
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
    /// <p>The ID of the maintenance window execution for which the task is a part.</p>
    pub fn window_execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.window_execution_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window execution for which the task is a part.</p>
    pub fn set_window_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_window_execution_id(input);
        self
    }
    /// <p>The ID of the maintenance window execution for which the task is a part.</p>
    pub fn get_window_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_window_execution_id()
    }
    /// <p>The ID of the specific task in the maintenance window task that should be retrieved. </p>
    pub fn task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_id(input.into());
        self
    }
    /// <p>The ID of the specific task in the maintenance window task that should be retrieved. </p>
    pub fn set_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task_id(input);
        self
    }
    /// <p>The ID of the specific task in the maintenance window task that should be retrieved. </p>
    pub fn get_task_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_task_id()
    }
    /// <p>The invocation ID to retrieve.</p>
    pub fn invocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.invocation_id(input.into());
        self
    }
    /// <p>The invocation ID to retrieve.</p>
    pub fn set_invocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_invocation_id(input);
        self
    }
    /// <p>The invocation ID to retrieve.</p>
    pub fn get_invocation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_invocation_id()
    }
}
