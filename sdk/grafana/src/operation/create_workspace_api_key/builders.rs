// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_workspace_api_key::_create_workspace_api_key_output::CreateWorkspaceApiKeyOutputBuilder;

pub use crate::operation::create_workspace_api_key::_create_workspace_api_key_input::CreateWorkspaceApiKeyInputBuilder;

impl CreateWorkspaceApiKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_workspace_api_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateWorkspaceApiKey`.
///
/// <p>Creates a Grafana API key for the workspace. This key can be used to authenticate requests sent to the workspace's HTTP API. See <a href="https://docs.aws.amazon.com/grafana/latest/userguide/Using-Grafana-APIs.html">https://docs.aws.amazon.com/grafana/latest/userguide/Using-Grafana-APIs.html</a> for available APIs and example requests.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateWorkspaceApiKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_workspace_api_key::builders::CreateWorkspaceApiKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyOutput,
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyError,
    > for CreateWorkspaceApiKeyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyOutput,
            crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateWorkspaceApiKeyFluentBuilder {
    /// Creates a new `CreateWorkspaceApiKey`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateWorkspaceApiKey as a reference.
    pub fn as_input(&self) -> &crate::operation::create_workspace_api_key::builders::CreateWorkspaceApiKeyInputBuilder {
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
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_workspace_api_key::CreateWorkspaceApiKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyOutput,
        crate::operation::create_workspace_api_key::CreateWorkspaceApiKeyError,
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
    /// <p>Specifies the name of the key. Keynames must be unique to the workspace.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>Specifies the name of the key. Keynames must be unique to the workspace.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// <p>Specifies the name of the key. Keynames must be unique to the workspace.</p>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_name()
    }
    /// <p>Specifies the permission level of the key.</p>
    /// <p> Valid values: <code>VIEWER</code>|<code>EDITOR</code>|<code>ADMIN</code> </p>
    pub fn key_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_role(input.into());
        self
    }
    /// <p>Specifies the permission level of the key.</p>
    /// <p> Valid values: <code>VIEWER</code>|<code>EDITOR</code>|<code>ADMIN</code> </p>
    pub fn set_key_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_role(input);
        self
    }
    /// <p>Specifies the permission level of the key.</p>
    /// <p> Valid values: <code>VIEWER</code>|<code>EDITOR</code>|<code>ADMIN</code> </p>
    pub fn get_key_role(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_role()
    }
    /// <p>Specifies the time in seconds until the key expires. Keys can be valid for up to 30 days.</p>
    pub fn seconds_to_live(mut self, input: i32) -> Self {
        self.inner = self.inner.seconds_to_live(input);
        self
    }
    /// <p>Specifies the time in seconds until the key expires. Keys can be valid for up to 30 days.</p>
    pub fn set_seconds_to_live(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_seconds_to_live(input);
        self
    }
    /// <p>Specifies the time in seconds until the key expires. Keys can be valid for up to 30 days.</p>
    pub fn get_seconds_to_live(&self) -> &::std::option::Option<i32> {
        self.inner.get_seconds_to_live()
    }
    /// <p>The ID of the workspace to create an API key.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace to create an API key.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// <p>The ID of the workspace to create an API key.</p>
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workspace_id()
    }
}
