// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_time_based_auto_scaling::_describe_time_based_auto_scaling_output::DescribeTimeBasedAutoScalingOutputBuilder;

pub use crate::operation::describe_time_based_auto_scaling::_describe_time_based_auto_scaling_input::DescribeTimeBasedAutoScalingInputBuilder;

impl DescribeTimeBasedAutoScalingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_time_based_auto_scaling();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTimeBasedAutoScaling`.
///
/// <p>Describes time-based auto scaling configurations for specified instances.</p> <note>
/// <p>You must specify at least one of the parameters.</p>
/// </note>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTimeBasedAutoScalingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_time_based_auto_scaling::builders::DescribeTimeBasedAutoScalingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingOutput,
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingError,
    > for DescribeTimeBasedAutoScalingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingOutput,
            crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTimeBasedAutoScalingFluentBuilder {
    /// Creates a new `DescribeTimeBasedAutoScaling`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTimeBasedAutoScaling as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_time_based_auto_scaling::builders::DescribeTimeBasedAutoScalingInputBuilder {
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
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScaling::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScaling::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingOutput,
        crate::operation::describe_time_based_auto_scaling::DescribeTimeBasedAutoScalingError,
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
    /// Appends an item to `InstanceIds`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>An array of instance IDs.</p>
    pub fn instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_ids(input.into());
        self
    }
    /// <p>An array of instance IDs.</p>
    pub fn set_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_instance_ids(input);
        self
    }
    /// <p>An array of instance IDs.</p>
    pub fn get_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_instance_ids()
    }
}
