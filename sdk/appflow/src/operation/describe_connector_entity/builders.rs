// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_connector_entity::_describe_connector_entity_output::DescribeConnectorEntityOutputBuilder;

pub use crate::operation::describe_connector_entity::_describe_connector_entity_input::DescribeConnectorEntityInputBuilder;

impl DescribeConnectorEntityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_connector_entity::DescribeConnectorEntityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_connector_entity::DescribeConnectorEntityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_connector_entity();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeConnectorEntity`.
///
/// <p> Provides details regarding the entity used with the connector, with a description of the data model for each field in that entity. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeConnectorEntityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_connector_entity::builders::DescribeConnectorEntityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_connector_entity::DescribeConnectorEntityOutput,
        crate::operation::describe_connector_entity::DescribeConnectorEntityError,
    > for DescribeConnectorEntityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_connector_entity::DescribeConnectorEntityOutput,
            crate::operation::describe_connector_entity::DescribeConnectorEntityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeConnectorEntityFluentBuilder {
    /// Creates a new `DescribeConnectorEntity`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeConnectorEntity as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_connector_entity::builders::DescribeConnectorEntityInputBuilder {
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
        crate::operation::describe_connector_entity::DescribeConnectorEntityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_connector_entity::DescribeConnectorEntityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_connector_entity::DescribeConnectorEntity::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_connector_entity::DescribeConnectorEntity::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_connector_entity::DescribeConnectorEntityOutput,
        crate::operation::describe_connector_entity::DescribeConnectorEntityError,
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
    /// <p> The entity name for that connector. </p>
    pub fn connector_entity_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_entity_name(input.into());
        self
    }
    /// <p> The entity name for that connector. </p>
    pub fn set_connector_entity_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_entity_name(input);
        self
    }
    /// <p> The entity name for that connector. </p>
    pub fn get_connector_entity_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_entity_name()
    }
    /// <p> The type of connector application, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(mut self, input: crate::types::ConnectorType) -> Self {
        self.inner = self.inner.connector_type(input);
        self
    }
    /// <p> The type of connector application, such as Salesforce, Amplitude, and so on. </p>
    pub fn set_connector_type(mut self, input: ::std::option::Option<crate::types::ConnectorType>) -> Self {
        self.inner = self.inner.set_connector_type(input);
        self
    }
    /// <p> The type of connector application, such as Salesforce, Amplitude, and so on. </p>
    pub fn get_connector_type(&self) -> &::std::option::Option<crate::types::ConnectorType> {
        self.inner.get_connector_type()
    }
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn connector_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_profile_name(input.into());
        self
    }
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn set_connector_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_profile_name(input);
        self
    }
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn get_connector_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_profile_name()
    }
    /// <p>The version of the API that's used by the connector.</p>
    pub fn api_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_version(input.into());
        self
    }
    /// <p>The version of the API that's used by the connector.</p>
    pub fn set_api_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_version(input);
        self
    }
    /// <p>The version of the API that's used by the connector.</p>
    pub fn get_api_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_version()
    }
}
