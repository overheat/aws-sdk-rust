// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_component_configuration::_update_component_configuration_output::UpdateComponentConfigurationOutputBuilder;

pub use crate::operation::update_component_configuration::_update_component_configuration_input::UpdateComponentConfigurationInputBuilder;

impl UpdateComponentConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_component_configuration::UpdateComponentConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_component_configuration::UpdateComponentConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_component_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateComponentConfiguration`.
///
/// <p>Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by <code>DescribeComponentConfigurationRecommendation</code>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateComponentConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_component_configuration::builders::UpdateComponentConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_component_configuration::UpdateComponentConfigurationOutput,
        crate::operation::update_component_configuration::UpdateComponentConfigurationError,
    > for UpdateComponentConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_component_configuration::UpdateComponentConfigurationOutput,
            crate::operation::update_component_configuration::UpdateComponentConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateComponentConfigurationFluentBuilder {
    /// Creates a new `UpdateComponentConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateComponentConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_component_configuration::builders::UpdateComponentConfigurationInputBuilder {
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
        crate::operation::update_component_configuration::UpdateComponentConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_component_configuration::UpdateComponentConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_component_configuration::UpdateComponentConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_component_configuration::UpdateComponentConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_component_configuration::UpdateComponentConfigurationOutput,
        crate::operation::update_component_configuration::UpdateComponentConfigurationError,
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
    /// <p>The name of the resource group.</p>
    pub fn resource_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_group_name(input.into());
        self
    }
    /// <p>The name of the resource group.</p>
    pub fn set_resource_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_group_name(input);
        self
    }
    /// <p>The name of the resource group.</p>
    pub fn get_resource_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_group_name()
    }
    /// <p>The name of the component.</p>
    pub fn component_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.component_name(input.into());
        self
    }
    /// <p>The name of the component.</p>
    pub fn set_component_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_component_name(input);
        self
    }
    /// <p>The name of the component.</p>
    pub fn get_component_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_component_name()
    }
    /// <p>Indicates whether the application component is monitored.</p>
    pub fn monitor(mut self, input: bool) -> Self {
        self.inner = self.inner.monitor(input);
        self
    }
    /// <p>Indicates whether the application component is monitored.</p>
    pub fn set_monitor(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_monitor(input);
        self
    }
    /// <p>Indicates whether the application component is monitored.</p>
    pub fn get_monitor(&self) -> &::std::option::Option<bool> {
        self.inner.get_monitor()
    }
    /// <p>The tier of the application component.</p>
    pub fn tier(mut self, input: crate::types::Tier) -> Self {
        self.inner = self.inner.tier(input);
        self
    }
    /// <p>The tier of the application component.</p>
    pub fn set_tier(mut self, input: ::std::option::Option<crate::types::Tier>) -> Self {
        self.inner = self.inner.set_tier(input);
        self
    }
    /// <p>The tier of the application component.</p>
    pub fn get_tier(&self) -> &::std::option::Option<crate::types::Tier> {
        self.inner.get_tier()
    }
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to <code>DescribeComponentConfigurationRecommendation</code> to see the recommended configuration for a component. For the complete format of the component configuration file, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p>
    pub fn component_configuration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.component_configuration(input.into());
        self
    }
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to <code>DescribeComponentConfigurationRecommendation</code> to see the recommended configuration for a component. For the complete format of the component configuration file, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p>
    pub fn set_component_configuration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_component_configuration(input);
        self
    }
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to <code>DescribeComponentConfigurationRecommendation</code> to see the recommended configuration for a component. For the complete format of the component configuration file, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p>
    pub fn get_component_configuration(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_component_configuration()
    }
    /// <p> Automatically configures the component by applying the recommended configurations. </p>
    pub fn auto_config_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_config_enabled(input);
        self
    }
    /// <p> Automatically configures the component by applying the recommended configurations. </p>
    pub fn set_auto_config_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_config_enabled(input);
        self
    }
    /// <p> Automatically configures the component by applying the recommended configurations. </p>
    pub fn get_auto_config_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_auto_config_enabled()
    }
}
