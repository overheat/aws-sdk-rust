// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_monitoring_schedule::_start_monitoring_schedule_output::StartMonitoringScheduleOutputBuilder;

pub use crate::operation::start_monitoring_schedule::_start_monitoring_schedule_input::StartMonitoringScheduleInputBuilder;

impl StartMonitoringScheduleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_monitoring_schedule::StartMonitoringScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_monitoring_schedule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartMonitoringSchedule`.
///
/// <p>Starts a previously stopped monitoring schedule.</p> <note>
/// <p>By default, when you successfully create a new schedule, the status of a monitoring schedule is <code>scheduled</code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartMonitoringScheduleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_monitoring_schedule::builders::StartMonitoringScheduleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleOutput,
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleError,
    > for StartMonitoringScheduleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_monitoring_schedule::StartMonitoringScheduleOutput,
            crate::operation::start_monitoring_schedule::StartMonitoringScheduleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartMonitoringScheduleFluentBuilder {
    /// Creates a new `StartMonitoringSchedule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartMonitoringSchedule as a reference.
    pub fn as_input(&self) -> &crate::operation::start_monitoring_schedule::builders::StartMonitoringScheduleInputBuilder {
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
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_monitoring_schedule::StartMonitoringScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_monitoring_schedule::StartMonitoringSchedule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_monitoring_schedule::StartMonitoringSchedule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleOutput,
        crate::operation::start_monitoring_schedule::StartMonitoringScheduleError,
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
    /// <p>The name of the schedule to start.</p>
    pub fn monitoring_schedule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.monitoring_schedule_name(input.into());
        self
    }
    /// <p>The name of the schedule to start.</p>
    pub fn set_monitoring_schedule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_monitoring_schedule_name(input);
        self
    }
    /// <p>The name of the schedule to start.</p>
    pub fn get_monitoring_schedule_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_monitoring_schedule_name()
    }
}
