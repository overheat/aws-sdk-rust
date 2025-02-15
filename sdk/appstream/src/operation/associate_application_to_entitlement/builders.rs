// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_application_to_entitlement::_associate_application_to_entitlement_output::AssociateApplicationToEntitlementOutputBuilder;

pub use crate::operation::associate_application_to_entitlement::_associate_application_to_entitlement_input::AssociateApplicationToEntitlementInputBuilder;

impl AssociateApplicationToEntitlementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_application_to_entitlement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateApplicationToEntitlement`.
///
/// <p>Associates an application to entitle.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateApplicationToEntitlementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_application_to_entitlement::builders::AssociateApplicationToEntitlementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementOutput,
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementError,
    > for AssociateApplicationToEntitlementFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementOutput,
            crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateApplicationToEntitlementFluentBuilder {
    /// Creates a new `AssociateApplicationToEntitlement`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateApplicationToEntitlement as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_application_to_entitlement::builders::AssociateApplicationToEntitlementInputBuilder {
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
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementOutput,
        crate::operation::associate_application_to_entitlement::AssociateApplicationToEntitlementError,
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
    /// <p>The name of the stack.</p>
    pub fn stack_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_name(input.into());
        self
    }
    /// <p>The name of the stack.</p>
    pub fn set_stack_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_name(input);
        self
    }
    /// <p>The name of the stack.</p>
    pub fn get_stack_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stack_name()
    }
    /// <p>The name of the entitlement.</p>
    pub fn entitlement_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entitlement_name(input.into());
        self
    }
    /// <p>The name of the entitlement.</p>
    pub fn set_entitlement_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entitlement_name(input);
        self
    }
    /// <p>The name of the entitlement.</p>
    pub fn get_entitlement_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_entitlement_name()
    }
    /// <p>The identifier of the application.</p>
    pub fn application_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_identifier(input.into());
        self
    }
    /// <p>The identifier of the application.</p>
    pub fn set_application_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_identifier(input);
        self
    }
    /// <p>The identifier of the application.</p>
    pub fn get_application_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_identifier()
    }
}
