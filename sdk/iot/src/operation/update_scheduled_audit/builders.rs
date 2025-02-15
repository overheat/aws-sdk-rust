// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_scheduled_audit::_update_scheduled_audit_output::UpdateScheduledAuditOutputBuilder;

pub use crate::operation::update_scheduled_audit::_update_scheduled_audit_input::UpdateScheduledAuditInputBuilder;

impl UpdateScheduledAuditInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_scheduled_audit();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateScheduledAudit`.
///
/// <p>Updates a scheduled audit, including which checks are performed and how often the audit takes place.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateScheduledAudit</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateScheduledAuditFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_scheduled_audit::builders::UpdateScheduledAuditInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
    > for UpdateScheduledAuditFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateScheduledAuditFluentBuilder {
    /// Creates a new `UpdateScheduledAudit`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateScheduledAudit as a reference.
    pub fn as_input(&self) -> &crate::operation::update_scheduled_audit::builders::UpdateScheduledAuditInputBuilder {
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
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_scheduled_audit::UpdateScheduledAudit::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_scheduled_audit::UpdateScheduledAudit::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_scheduled_audit::UpdateScheduledAuditOutput,
        crate::operation::update_scheduled_audit::UpdateScheduledAuditError,
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
    /// <p>How often the scheduled audit takes place, either <code>DAILY</code>, <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the system.</p>
    pub fn frequency(mut self, input: crate::types::AuditFrequency) -> Self {
        self.inner = self.inner.frequency(input);
        self
    }
    /// <p>How often the scheduled audit takes place, either <code>DAILY</code>, <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the system.</p>
    pub fn set_frequency(mut self, input: ::std::option::Option<crate::types::AuditFrequency>) -> Self {
        self.inner = self.inner.set_frequency(input);
        self
    }
    /// <p>How often the scheduled audit takes place, either <code>DAILY</code>, <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the system.</p>
    pub fn get_frequency(&self) -> &::std::option::Option<crate::types::AuditFrequency> {
        self.inner.get_frequency()
    }
    /// <p>The day of the month on which the scheduled audit takes place. This can be <code>1</code> through <code>31</code> or <code>LAST</code>. This field is required if the <code>frequency</code> parameter is set to <code>MONTHLY</code>. If days 29-31 are specified, and the month does not have that many days, the audit takes place on the "LAST" day of the month.</p>
    pub fn day_of_month(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.day_of_month(input.into());
        self
    }
    /// <p>The day of the month on which the scheduled audit takes place. This can be <code>1</code> through <code>31</code> or <code>LAST</code>. This field is required if the <code>frequency</code> parameter is set to <code>MONTHLY</code>. If days 29-31 are specified, and the month does not have that many days, the audit takes place on the "LAST" day of the month.</p>
    pub fn set_day_of_month(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_day_of_month(input);
        self
    }
    /// <p>The day of the month on which the scheduled audit takes place. This can be <code>1</code> through <code>31</code> or <code>LAST</code>. This field is required if the <code>frequency</code> parameter is set to <code>MONTHLY</code>. If days 29-31 are specified, and the month does not have that many days, the audit takes place on the "LAST" day of the month.</p>
    pub fn get_day_of_month(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_day_of_month()
    }
    /// <p>The day of the week on which the scheduled audit takes place. This can be one of <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the "frequency" parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p>
    pub fn day_of_week(mut self, input: crate::types::DayOfWeek) -> Self {
        self.inner = self.inner.day_of_week(input);
        self
    }
    /// <p>The day of the week on which the scheduled audit takes place. This can be one of <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the "frequency" parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p>
    pub fn set_day_of_week(mut self, input: ::std::option::Option<crate::types::DayOfWeek>) -> Self {
        self.inner = self.inner.set_day_of_week(input);
        self
    }
    /// <p>The day of the week on which the scheduled audit takes place. This can be one of <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the "frequency" parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p>
    pub fn get_day_of_week(&self) -> &::std::option::Option<crate::types::DayOfWeek> {
        self.inner.get_day_of_week()
    }
    /// Appends an item to `targetCheckNames`.
    ///
    /// To override the contents of this collection use [`set_target_check_names`](Self::set_target_check_names).
    ///
    /// <p>Which checks are performed during the scheduled audit. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn target_check_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_check_names(input.into());
        self
    }
    /// <p>Which checks are performed during the scheduled audit. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn set_target_check_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_target_check_names(input);
        self
    }
    /// <p>Which checks are performed during the scheduled audit. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn get_target_check_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_target_check_names()
    }
    /// <p>The name of the scheduled audit. (Max. 128 chars)</p>
    pub fn scheduled_audit_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scheduled_audit_name(input.into());
        self
    }
    /// <p>The name of the scheduled audit. (Max. 128 chars)</p>
    pub fn set_scheduled_audit_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_scheduled_audit_name(input);
        self
    }
    /// <p>The name of the scheduled audit. (Max. 128 chars)</p>
    pub fn get_scheduled_audit_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_scheduled_audit_name()
    }
}
