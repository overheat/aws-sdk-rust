// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::execute_policy::_execute_policy_output::ExecutePolicyOutputBuilder;

pub use crate::operation::execute_policy::_execute_policy_input::ExecutePolicyInputBuilder;

impl ExecutePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::execute_policy::ExecutePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_policy::ExecutePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.execute_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExecutePolicy`.
///
/// <p>Executes the specified policy. This can be useful for testing the design of your scaling policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecutePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::execute_policy::builders::ExecutePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::execute_policy::ExecutePolicyOutput,
        crate::operation::execute_policy::ExecutePolicyError,
    > for ExecutePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::execute_policy::ExecutePolicyOutput,
            crate::operation::execute_policy::ExecutePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ExecutePolicyFluentBuilder {
    /// Creates a new `ExecutePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ExecutePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::execute_policy::builders::ExecutePolicyInputBuilder {
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
        crate::operation::execute_policy::ExecutePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_policy::ExecutePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::execute_policy::ExecutePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::execute_policy::ExecutePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::execute_policy::ExecutePolicyOutput,
        crate::operation::execute_policy::ExecutePolicyError,
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
    /// <p>The name of the Auto Scaling group.</p>
    pub fn auto_scaling_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.auto_scaling_group_name(input.into());
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn set_auto_scaling_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_auto_scaling_group_name(input);
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn get_auto_scaling_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_auto_scaling_group_name()
    }
    /// <p>The name or ARN of the policy.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>The name or ARN of the policy.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The name or ARN of the policy.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before executing the policy.</p>
    /// <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub fn honor_cooldown(mut self, input: bool) -> Self {
        self.inner = self.inner.honor_cooldown(input);
        self
    }
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before executing the policy.</p>
    /// <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub fn set_honor_cooldown(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_honor_cooldown(input);
        self
    }
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before executing the policy.</p>
    /// <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/Cooldown.html">Scaling cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
    pub fn get_honor_cooldown(&self) -> &::std::option::Option<bool> {
        self.inner.get_honor_cooldown()
    }
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p>
    /// <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn metric_value(mut self, input: f64) -> Self {
        self.inner = self.inner.metric_value(input);
        self
    }
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p>
    /// <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn set_metric_value(mut self, input: ::std::option::Option<f64>) -> Self {
        self.inner = self.inner.set_metric_value(input);
        self
    }
    /// <p>The metric value to compare to <code>BreachThreshold</code>. This enables you to execute a policy of type <code>StepScaling</code> and determine which step adjustment to use. For example, if the breach threshold is 50 and you want to use a step adjustment with a lower bound of 0 and an upper bound of 10, you can set the metric value to 59.</p>
    /// <p>If you specify a metric value that doesn't correspond to a step adjustment for the policy, the call returns an error.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn get_metric_value(&self) -> &::std::option::Option<f64> {
        self.inner.get_metric_value()
    }
    /// <p>The breach threshold for the alarm.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn breach_threshold(mut self, input: f64) -> Self {
        self.inner = self.inner.breach_threshold(input);
        self
    }
    /// <p>The breach threshold for the alarm.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn set_breach_threshold(mut self, input: ::std::option::Option<f64>) -> Self {
        self.inner = self.inner.set_breach_threshold(input);
        self
    }
    /// <p>The breach threshold for the alarm.</p>
    /// <p>Required if the policy type is <code>StepScaling</code> and not supported otherwise.</p>
    pub fn get_breach_threshold(&self) -> &::std::option::Option<f64> {
        self.inner.get_breach_threshold()
    }
}
