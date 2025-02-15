// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_core_definition::_update_core_definition_output::UpdateCoreDefinitionOutputBuilder;

pub use crate::operation::update_core_definition::_update_core_definition_input::UpdateCoreDefinitionInputBuilder;

impl UpdateCoreDefinitionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_core_definition::UpdateCoreDefinitionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_core_definition::UpdateCoreDefinitionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_core_definition();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateCoreDefinition`.
///
/// Updates a core definition.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCoreDefinitionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_core_definition::builders::UpdateCoreDefinitionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_core_definition::UpdateCoreDefinitionOutput,
        crate::operation::update_core_definition::UpdateCoreDefinitionError,
    > for UpdateCoreDefinitionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_core_definition::UpdateCoreDefinitionOutput,
            crate::operation::update_core_definition::UpdateCoreDefinitionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateCoreDefinitionFluentBuilder {
    /// Creates a new `UpdateCoreDefinition`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateCoreDefinition as a reference.
    pub fn as_input(&self) -> &crate::operation::update_core_definition::builders::UpdateCoreDefinitionInputBuilder {
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
        crate::operation::update_core_definition::UpdateCoreDefinitionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_core_definition::UpdateCoreDefinitionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_core_definition::UpdateCoreDefinition::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_core_definition::UpdateCoreDefinition::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_core_definition::UpdateCoreDefinitionOutput,
        crate::operation::update_core_definition::UpdateCoreDefinitionError,
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
    /// The ID of the core definition.
    pub fn core_definition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.core_definition_id(input.into());
        self
    }
    /// The ID of the core definition.
    pub fn set_core_definition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_core_definition_id(input);
        self
    }
    /// The ID of the core definition.
    pub fn get_core_definition_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_core_definition_id()
    }
    /// The name of the definition.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// The name of the definition.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// The name of the definition.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
