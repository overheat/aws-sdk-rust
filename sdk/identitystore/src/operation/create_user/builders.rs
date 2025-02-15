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
/// <p>Creates a user within the specified identity store.</p>
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
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn identity_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_store_id(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn set_identity_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_store_id(input);
        self
    }
    /// <p>The globally unique identifier for the identity store.</p>
    pub fn get_identity_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_store_id()
    }
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. <code>Administrator</code> and <code>AWSAdministrators</code> are reserved names and can't be used for users or groups.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. <code>Administrator</code> and <code>AWSAdministrators</code> are reserved names and can't be used for users or groups.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>A unique string used to identify the user. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. <code>Administrator</code> and <code>AWSAdministrators</code> are reserved names and can't be used for users or groups.</p>
    pub fn get_user_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_name()
    }
    /// <p>An object containing the name of the user.</p>
    pub fn name(mut self, input: crate::types::Name) -> Self {
        self.inner = self.inner.name(input);
        self
    }
    /// <p>An object containing the name of the user.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::Name>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>An object containing the name of the user.</p>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::Name> {
        self.inner.get_name()
    }
    /// <p>A string containing the name of the user. This value is typically formatted for display when the user is referenced. For example, "John Doe." </p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>A string containing the name of the user. This value is typically formatted for display when the user is referenced. For example, "John Doe." </p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>A string containing the name of the user. This value is typically formatted for display when the user is referenced. For example, "John Doe." </p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn nick_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.nick_name(input.into());
        self
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn set_nick_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_nick_name(input);
        self
    }
    /// <p>A string containing an alternate name for the user.</p>
    pub fn get_nick_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_nick_name()
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn profile_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_url(input.into());
        self
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn set_profile_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_url(input);
        self
    }
    /// <p>A string containing a URL that might be associated with the user.</p>
    pub fn get_profile_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profile_url()
    }
    /// Appends an item to `Emails`.
    ///
    /// To override the contents of this collection use [`set_emails`](Self::set_emails).
    ///
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn emails(mut self, input: crate::types::Email) -> Self {
        self.inner = self.inner.emails(input);
        self
    }
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn set_emails(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Email>>) -> Self {
        self.inner = self.inner.set_emails(input);
        self
    }
    /// <p>A list of <code>Email</code> objects containing email addresses associated with the user.</p>
    pub fn get_emails(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Email>> {
        self.inner.get_emails()
    }
    /// Appends an item to `Addresses`.
    ///
    /// To override the contents of this collection use [`set_addresses`](Self::set_addresses).
    ///
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn addresses(mut self, input: crate::types::Address) -> Self {
        self.inner = self.inner.addresses(input);
        self
    }
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn set_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Address>>) -> Self {
        self.inner = self.inner.set_addresses(input);
        self
    }
    /// <p>A list of <code>Address</code> objects containing addresses associated with the user.</p>
    pub fn get_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Address>> {
        self.inner.get_addresses()
    }
    /// Appends an item to `PhoneNumbers`.
    ///
    /// To override the contents of this collection use [`set_phone_numbers`](Self::set_phone_numbers).
    ///
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn phone_numbers(mut self, input: crate::types::PhoneNumber) -> Self {
        self.inner = self.inner.phone_numbers(input);
        self
    }
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn set_phone_numbers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumber>>) -> Self {
        self.inner = self.inner.set_phone_numbers(input);
        self
    }
    /// <p>A list of <code>PhoneNumber</code> objects containing phone numbers associated with the user.</p>
    pub fn get_phone_numbers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PhoneNumber>> {
        self.inner.get_phone_numbers()
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn user_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_type(input.into());
        self
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn set_user_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_type(input);
        self
    }
    /// <p>A string indicating the type of user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn get_user_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_type()
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.title(input.into());
        self
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_title(input);
        self
    }
    /// <p>A string containing the title of the user. Possible values are left unspecified. The value can vary based on your specific use case.</p>
    pub fn get_title(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_title()
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn preferred_language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.preferred_language(input.into());
        self
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn set_preferred_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_preferred_language(input);
        self
    }
    /// <p>A string containing the preferred language of the user. For example, "American English" or "en-us."</p>
    pub fn get_preferred_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_preferred_language()
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale(input.into());
        self
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale(input);
        self
    }
    /// <p>A string containing the geographical region or location of the user.</p>
    pub fn get_locale(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale()
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.timezone(input.into());
        self
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_timezone(input);
        self
    }
    /// <p>A string containing the time zone of the user.</p>
    pub fn get_timezone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_timezone()
    }
}
