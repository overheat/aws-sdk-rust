// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_identity_provider_config::_disassociate_identity_provider_config_output::DisassociateIdentityProviderConfigOutputBuilder;

pub use crate::operation::disassociate_identity_provider_config::_disassociate_identity_provider_config_input::DisassociateIdentityProviderConfigInputBuilder;

impl DisassociateIdentityProviderConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_identity_provider_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateIdentityProviderConfig`.
///
/// <p>Disassociates an identity provider configuration from a cluster. If you disassociate an identity provider from your cluster, users included in the provider can no longer access the cluster. However, you can still access the cluster with Amazon Web Services IAM users.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateIdentityProviderConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput,
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError,
    > for DisassociateIdentityProviderConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput,
            crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateIdentityProviderConfigFluentBuilder {
    /// Creates a new `DisassociateIdentityProviderConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateIdentityProviderConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigInputBuilder {
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
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput,
        crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError,
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
    /// <p>The name of the cluster to disassociate an identity provider from.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_name(input.into());
        self
    }
    /// <p>The name of the cluster to disassociate an identity provider from.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_name(input);
        self
    }
    /// <p>The name of the cluster to disassociate an identity provider from.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_name()
    }
    /// <p>An object representing an identity provider configuration.</p>
    pub fn identity_provider_config(mut self, input: crate::types::IdentityProviderConfig) -> Self {
        self.inner = self.inner.identity_provider_config(input);
        self
    }
    /// <p>An object representing an identity provider configuration.</p>
    pub fn set_identity_provider_config(mut self, input: ::std::option::Option<crate::types::IdentityProviderConfig>) -> Self {
        self.inner = self.inner.set_identity_provider_config(input);
        self
    }
    /// <p>An object representing an identity provider configuration.</p>
    pub fn get_identity_provider_config(&self) -> &::std::option::Option<crate::types::IdentityProviderConfig> {
        self.inner.get_identity_provider_config()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
}
