// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_session::_delete_session_output::DeleteSessionOutputBuilder;

pub use crate::operation::delete_session::_delete_session_input::DeleteSessionInputBuilder;

impl DeleteSessionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_session::DeleteSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_session::DeleteSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_session();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteSession`.
///
/// <p>Removes session information for a specified bot, alias, and user ID. </p>
/// <p>You can use this operation to restart a conversation with a bot. When you remove a session, the entire history of the session is removed so that you can start again.</p>
/// <p>You don't need to delete a session. Sessions have a time limit and will expire. Set the session time limit when you create the bot. The default is 5 minutes, but you can specify anything between 1 minute and 24 hours.</p>
/// <p>If you specify a bot or alias ID that doesn't exist, you receive a <code>BadRequestException.</code> </p>
/// <p>If the locale doesn't exist in the bot, or if the locale hasn't been enables for the alias, you receive a <code>BadRequestException</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSessionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_session::builders::DeleteSessionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_session::DeleteSessionOutput,
        crate::operation::delete_session::DeleteSessionError,
    > for DeleteSessionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_session::DeleteSessionOutput,
            crate::operation::delete_session::DeleteSessionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteSessionFluentBuilder {
    /// Creates a new `DeleteSession`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteSession as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_session::builders::DeleteSessionInputBuilder {
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
        crate::operation::delete_session::DeleteSessionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_session::DeleteSessionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_session::DeleteSession::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_session::DeleteSession::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_session::DeleteSessionOutput,
        crate::operation::delete_session::DeleteSessionError,
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
    /// <p>The identifier of the bot that contains the session data.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot that contains the session data.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The identifier of the bot that contains the session data.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The alias identifier in use for the bot that contains the session data.</p>
    pub fn bot_alias_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_alias_id(input.into());
        self
    }
    /// <p>The alias identifier in use for the bot that contains the session data.</p>
    pub fn set_bot_alias_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_alias_id(input);
        self
    }
    /// <p>The alias identifier in use for the bot that contains the session data.</p>
    pub fn get_bot_alias_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_alias_id()
    }
    /// <p>The locale where the session is in use.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The identifier of the session to delete.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The identifier of the session to delete.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The identifier of the session to delete.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
}
