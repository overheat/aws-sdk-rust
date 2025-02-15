// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_observability_configuration::_describe_observability_configuration_output::DescribeObservabilityConfigurationOutputBuilder;

pub use crate::operation::describe_observability_configuration::_describe_observability_configuration_input::DescribeObservabilityConfigurationInputBuilder;

impl DescribeObservabilityConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_observability_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeObservabilityConfiguration`.
///
/// <p>Return a full description of an App Runner observability configuration resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeObservabilityConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_observability_configuration::builders::DescribeObservabilityConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationOutput,
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationError,
    > for DescribeObservabilityConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationOutput,
            crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeObservabilityConfigurationFluentBuilder {
    /// Creates a new `DescribeObservabilityConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeObservabilityConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_observability_configuration::builders::DescribeObservabilityConfigurationInputBuilder {
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
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_observability_configuration::DescribeObservabilityConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_observability_configuration::DescribeObservabilityConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationOutput,
        crate::operation::describe_observability_configuration::DescribeObservabilityConfigurationError,
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
    /// <p>The Amazon Resource Name (ARN) of the App Runner observability configuration that you want a description for.</p>
    /// <p>The ARN can be a full observability configuration ARN, or a partial ARN ending with either <code>.../<i>name</i> </code> or <code>.../<i>name</i>/<i>revision</i> </code>. If a revision isn't specified, the latest active revision is described.</p>
    pub fn observability_configuration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.observability_configuration_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the App Runner observability configuration that you want a description for.</p>
    /// <p>The ARN can be a full observability configuration ARN, or a partial ARN ending with either <code>.../<i>name</i> </code> or <code>.../<i>name</i>/<i>revision</i> </code>. If a revision isn't specified, the latest active revision is described.</p>
    pub fn set_observability_configuration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_observability_configuration_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the App Runner observability configuration that you want a description for.</p>
    /// <p>The ARN can be a full observability configuration ARN, or a partial ARN ending with either <code>.../<i>name</i> </code> or <code>.../<i>name</i>/<i>revision</i> </code>. If a revision isn't specified, the latest active revision is described.</p>
    pub fn get_observability_configuration_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_observability_configuration_arn()
    }
}
