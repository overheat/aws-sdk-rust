// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_user::_create_user_output::CreateUserOutputBuilder;

pub use crate::operation::create_user::_create_user_input::CreateUserInputBuilder;

impl CreateUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_user::CreateUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_user::CreateUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateUser`.
///
/// <p>Creates a new user in the user pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_user::builders::CreateUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_user::CreateUserOutput,
        crate::operation::create_user::CreateUserError,
    > for CreateUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_user::CreateUserOutput,
            crate::operation::create_user::CreateUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateUserFluentBuilder {
    /// Creates a new `CreateUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateUser as a reference.
    pub fn as_input(&self) -> &crate::operation::create_user::builders::CreateUserInputBuilder {
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
        crate::operation::create_user::CreateUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_user::CreateUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_user::CreateUser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_user::CreateUser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_user::CreateUserOutput,
        crate::operation::create_user::CreateUserError,
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
    /// <p>The email address of the user.</p> <note>
    /// <p>Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.</p>
    /// </note>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>The email address of the user.</p> <note>
    /// <p>Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.</p>
    /// </note>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>The email address of the user.</p> <note>
    /// <p>Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.</p>
    /// </note>
    pub fn get_user_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_name()
    }
    /// <p>The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent. </p> <note>
    /// <p>The temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.</p>
    /// </note>
    pub fn message_action(mut self, input: crate::types::MessageAction) -> Self {
        self.inner = self.inner.message_action(input);
        self
    }
    /// <p>The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent. </p> <note>
    /// <p>The temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.</p>
    /// </note>
    pub fn set_message_action(mut self, input: ::std::option::Option<crate::types::MessageAction>) -> Self {
        self.inner = self.inner.set_message_action(input);
        self
    }
    /// <p>The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent. </p> <note>
    /// <p>The temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.</p>
    /// </note>
    pub fn get_message_action(&self) -> &::std::option::Option<crate::types::MessageAction> {
        self.inner.get_message_action()
    }
    /// <p>The first name, or given name, of the user.</p>
    pub fn first_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.first_name(input.into());
        self
    }
    /// <p>The first name, or given name, of the user.</p>
    pub fn set_first_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_first_name(input);
        self
    }
    /// <p>The first name, or given name, of the user.</p>
    pub fn get_first_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_first_name()
    }
    /// <p>The last name, or surname, of the user.</p>
    pub fn last_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.last_name(input.into());
        self
    }
    /// <p>The last name, or surname, of the user.</p>
    pub fn set_last_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_last_name(input);
        self
    }
    /// <p>The last name, or surname, of the user.</p>
    pub fn get_last_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_last_name()
    }
    /// <p>The authentication type for the user. You must specify USERPOOL. </p>
    pub fn authentication_type(mut self, input: crate::types::AuthenticationType) -> Self {
        self.inner = self.inner.authentication_type(input);
        self
    }
    /// <p>The authentication type for the user. You must specify USERPOOL. </p>
    pub fn set_authentication_type(mut self, input: ::std::option::Option<crate::types::AuthenticationType>) -> Self {
        self.inner = self.inner.set_authentication_type(input);
        self
    }
    /// <p>The authentication type for the user. You must specify USERPOOL. </p>
    pub fn get_authentication_type(&self) -> &::std::option::Option<crate::types::AuthenticationType> {
        self.inner.get_authentication_type()
    }
}
