// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_availability_configuration::_update_availability_configuration_output::UpdateAvailabilityConfigurationOutputBuilder;

pub use crate::operation::update_availability_configuration::_update_availability_configuration_input::UpdateAvailabilityConfigurationInputBuilder;

impl UpdateAvailabilityConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_availability_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAvailabilityConfiguration`.
///
/// <p>Updates an existing <code>AvailabilityConfiguration</code> for the given WorkMail organization and domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAvailabilityConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_availability_configuration::builders::UpdateAvailabilityConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationOutput,
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationError,
    > for UpdateAvailabilityConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationOutput,
            crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAvailabilityConfigurationFluentBuilder {
    /// Creates a new `UpdateAvailabilityConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAvailabilityConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_availability_configuration::builders::UpdateAvailabilityConfigurationInputBuilder {
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
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_availability_configuration::UpdateAvailabilityConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_availability_configuration::UpdateAvailabilityConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationOutput,
        crate::operation::update_availability_configuration::UpdateAvailabilityConfigurationError,
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
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be updated.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be updated.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be updated.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_id()
    }
    /// <p>The domain to which the provider applies the availability configuration.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The domain to which the provider applies the availability configuration.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The domain to which the provider applies the availability configuration.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>The EWS availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn ews_provider(mut self, input: crate::types::EwsAvailabilityProvider) -> Self {
        self.inner = self.inner.ews_provider(input);
        self
    }
    /// <p>The EWS availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn set_ews_provider(mut self, input: ::std::option::Option<crate::types::EwsAvailabilityProvider>) -> Self {
        self.inner = self.inner.set_ews_provider(input);
        self
    }
    /// <p>The EWS availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn get_ews_provider(&self) -> &::std::option::Option<crate::types::EwsAvailabilityProvider> {
        self.inner.get_ews_provider()
    }
    /// <p>The Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn lambda_provider(mut self, input: crate::types::LambdaAvailabilityProvider) -> Self {
        self.inner = self.inner.lambda_provider(input);
        self
    }
    /// <p>The Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn set_lambda_provider(mut self, input: ::std::option::Option<crate::types::LambdaAvailabilityProvider>) -> Self {
        self.inner = self.inner.set_lambda_provider(input);
        self
    }
    /// <p>The Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>. The previously stored provider will be overridden by the one provided.</p>
    pub fn get_lambda_provider(&self) -> &::std::option::Option<crate::types::LambdaAvailabilityProvider> {
        self.inner.get_lambda_provider()
    }
}
