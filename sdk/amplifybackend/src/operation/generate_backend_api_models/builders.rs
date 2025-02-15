// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::generate_backend_api_models::_generate_backend_api_models_output::GenerateBackendApiModelsOutputBuilder;

pub use crate::operation::generate_backend_api_models::_generate_backend_api_models_input::GenerateBackendApiModelsInputBuilder;

impl GenerateBackendApiModelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_backend_api_models::GenerateBackendApiModelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::generate_backend_api_models::GenerateBackendAPIModelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.generate_backend_api_models();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateBackendAPIModels`.
///
/// <p>Generates a model schema for an existing backend API resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateBackendAPIModelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::generate_backend_api_models::builders::GenerateBackendApiModelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::generate_backend_api_models::GenerateBackendApiModelsOutput,
        crate::operation::generate_backend_api_models::GenerateBackendAPIModelsError,
    > for GenerateBackendAPIModelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::generate_backend_api_models::GenerateBackendApiModelsOutput,
            crate::operation::generate_backend_api_models::GenerateBackendAPIModelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GenerateBackendAPIModelsFluentBuilder {
    /// Creates a new `GenerateBackendAPIModels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GenerateBackendAPIModels as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_backend_api_models::builders::GenerateBackendApiModelsInputBuilder {
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
        crate::operation::generate_backend_api_models::GenerateBackendApiModelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::generate_backend_api_models::GenerateBackendAPIModelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::generate_backend_api_models::GenerateBackendAPIModels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::generate_backend_api_models::GenerateBackendAPIModels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::generate_backend_api_models::GenerateBackendApiModelsOutput,
        crate::operation::generate_backend_api_models::GenerateBackendAPIModelsError,
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
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The app ID.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// <p>The name of the backend environment.</p>
    pub fn backend_environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backend_environment_name(input.into());
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn set_backend_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backend_environment_name(input);
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn get_backend_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backend_environment_name()
    }
    /// <p>The name of this resource.</p>
    pub fn resource_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_name(input.into());
        self
    }
    /// <p>The name of this resource.</p>
    pub fn set_resource_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_name(input);
        self
    }
    /// <p>The name of this resource.</p>
    pub fn get_resource_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_name()
    }
}
