// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connector_profile::_update_connector_profile_output::UpdateConnectorProfileOutputBuilder;

pub use crate::operation::update_connector_profile::_update_connector_profile_input::UpdateConnectorProfileInputBuilder;

impl UpdateConnectorProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connector_profile::UpdateConnectorProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_connector_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConnectorProfile`.
///
/// <p> Updates a given connector profile associated with your account. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConnectorProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connector_profile::builders::UpdateConnectorProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
        crate::operation::update_connector_profile::UpdateConnectorProfileError,
    > for UpdateConnectorProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
            crate::operation::update_connector_profile::UpdateConnectorProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConnectorProfileFluentBuilder {
    /// Creates a new `UpdateConnectorProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConnectorProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::update_connector_profile::builders::UpdateConnectorProfileInputBuilder {
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
        crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connector_profile::UpdateConnectorProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_connector_profile::UpdateConnectorProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_connector_profile::UpdateConnectorProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
        crate::operation::update_connector_profile::UpdateConnectorProfileError,
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
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn connector_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_profile_name(input.into());
        self
    }
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn set_connector_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_profile_name(input);
        self
    }
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn get_connector_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_profile_name()
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn connection_mode(mut self, input: crate::types::ConnectionMode) -> Self {
        self.inner = self.inner.connection_mode(input);
        self
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn set_connection_mode(mut self, input: ::std::option::Option<crate::types::ConnectionMode>) -> Self {
        self.inner = self.inner.set_connection_mode(input);
        self
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn get_connection_mode(&self) -> &::std::option::Option<crate::types::ConnectionMode> {
        self.inner.get_connection_mode()
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn connector_profile_config(mut self, input: crate::types::ConnectorProfileConfig) -> Self {
        self.inner = self.inner.connector_profile_config(input);
        self
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn set_connector_profile_config(mut self, input: ::std::option::Option<crate::types::ConnectorProfileConfig>) -> Self {
        self.inner = self.inner.set_connector_profile_config(input);
        self
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn get_connector_profile_config(&self) -> &::std::option::Option<crate::types::ConnectorProfileConfig> {
        self.inner.get_connector_profile_config()
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>UpdateConnectorProfile</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs. If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>UpdateConnectorProfile</code>. The token is active for 8 hours.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>UpdateConnectorProfile</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs. If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>UpdateConnectorProfile</code>. The token is active for 8 hours.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>UpdateConnectorProfile</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs. If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>UpdateConnectorProfile</code>. The token is active for 8 hours.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
