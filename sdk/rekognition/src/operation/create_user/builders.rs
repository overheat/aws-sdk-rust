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
/// <p>Creates a new User within a collection specified by <code>CollectionId</code>. Takes <code>UserId</code> as a parameter, which is a user provided ID which should be unique within the collection. The provided <code>UserId</code> will alias the system generated UUID to make the <code>UserId</code> more user friendly. </p>
/// <p>Uses a <code>ClientToken</code>, an idempotency token that ensures a call to <code>CreateUser</code> completes only once. If the value is not supplied, the AWS SDK generates an idempotency token for the requests. This prevents retries after a network error results from making multiple <code>CreateUser</code> calls. </p>
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
    /// <p>The ID of an existing collection to which the new UserID needs to be created.</p>
    pub fn collection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.collection_id(input.into());
        self
    }
    /// <p>The ID of an existing collection to which the new UserID needs to be created.</p>
    pub fn set_collection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_collection_id(input);
        self
    }
    /// <p>The ID of an existing collection to which the new UserID needs to be created.</p>
    pub fn get_collection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_collection_id()
    }
    /// <p>ID for the UserID to be created. This ID needs to be unique within the collection.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>ID for the UserID to be created. This ID needs to be unique within the collection.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>ID for the UserID to be created. This ID needs to be unique within the collection.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>Idempotent token used to identify the request to <code>CreateUser</code>. If you use the same token with multiple <code>CreateUser</code> requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Idempotent token used to identify the request to <code>CreateUser</code>. If you use the same token with multiple <code>CreateUser</code> requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Idempotent token used to identify the request to <code>CreateUser</code>. If you use the same token with multiple <code>CreateUser</code> requests, the same response is returned. Use ClientRequestToken to prevent the same request from being processed more than once.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
}
