// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_component_configuration_recommendation::_describe_component_configuration_recommendation_output::DescribeComponentConfigurationRecommendationOutputBuilder;

pub use crate::operation::describe_component_configuration_recommendation::_describe_component_configuration_recommendation_input::DescribeComponentConfigurationRecommendationInputBuilder;

impl DescribeComponentConfigurationRecommendationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_component_configuration_recommendation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeComponentConfigurationRecommendation`.
///
/// <p>Describes the recommended monitoring configuration of the component.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeComponentConfigurationRecommendationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_component_configuration_recommendation::builders::DescribeComponentConfigurationRecommendationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationOutput,
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationError,
    > for DescribeComponentConfigurationRecommendationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationOutput,
            crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeComponentConfigurationRecommendationFluentBuilder {
    /// Creates a new `DescribeComponentConfigurationRecommendation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeComponentConfigurationRecommendation as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_component_configuration_recommendation::builders::DescribeComponentConfigurationRecommendationInputBuilder {
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
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendation::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendation::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationOutput,
        crate::operation::describe_component_configuration_recommendation::DescribeComponentConfigurationRecommendationError,
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
    /// <p>The recommended configuration type.</p>
    pub fn recommendation_type(mut self, input: crate::types::RecommendationType) -> Self {
        self.inner = self.inner.recommendation_type(input);
        self
    }
    /// <p>The recommended configuration type.</p>
    pub fn set_recommendation_type(mut self, input: ::std::option::Option<crate::types::RecommendationType>) -> Self {
        self.inner = self.inner.set_recommendation_type(input);
        self
    }
    /// <p>The recommended configuration type.</p>
    pub fn get_recommendation_type(&self) -> &::std::option::Option<crate::types::RecommendationType> {
        self.inner.get_recommendation_type()
    }
}
