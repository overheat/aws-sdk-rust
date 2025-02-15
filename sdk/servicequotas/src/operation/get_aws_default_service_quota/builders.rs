// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_aws_default_service_quota::_get_aws_default_service_quota_output::GetAwsDefaultServiceQuotaOutputBuilder;

pub use crate::operation::get_aws_default_service_quota::_get_aws_default_service_quota_input::GetAwsDefaultServiceQuotaInputBuilder;

impl GetAwsDefaultServiceQuotaInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_aws_default_service_quota::GetAwsDefaultServiceQuotaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuotaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_aws_default_service_quota();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAWSDefaultServiceQuota`.
///
/// <p>Retrieves the default value for the specified quota. The default value does not reflect any quota increases.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAWSDefaultServiceQuotaFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_aws_default_service_quota::builders::GetAwsDefaultServiceQuotaInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_aws_default_service_quota::GetAwsDefaultServiceQuotaOutput,
        crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuotaError,
    > for GetAWSDefaultServiceQuotaFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_aws_default_service_quota::GetAwsDefaultServiceQuotaOutput,
            crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuotaError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAWSDefaultServiceQuotaFluentBuilder {
    /// Creates a new `GetAWSDefaultServiceQuota`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAWSDefaultServiceQuota as a reference.
    pub fn as_input(&self) -> &crate::operation::get_aws_default_service_quota::builders::GetAwsDefaultServiceQuotaInputBuilder {
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
        crate::operation::get_aws_default_service_quota::GetAwsDefaultServiceQuotaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuotaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuota::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuota::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_aws_default_service_quota::GetAwsDefaultServiceQuotaOutput,
        crate::operation::get_aws_default_service_quota::GetAWSDefaultServiceQuotaError,
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
    /// <p>Specifies the service identifier. To find the service code value for an Amazon Web Services service, use the <code>ListServices</code> operation.</p>
    pub fn service_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_code(input.into());
        self
    }
    /// <p>Specifies the service identifier. To find the service code value for an Amazon Web Services service, use the <code>ListServices</code> operation.</p>
    pub fn set_service_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_code(input);
        self
    }
    /// <p>Specifies the service identifier. To find the service code value for an Amazon Web Services service, use the <code>ListServices</code> operation.</p>
    pub fn get_service_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_code()
    }
    /// <p>Specifies the quota identifier. To find the quota code for a specific quota, use the <code>ListServiceQuotas</code> operation, and look for the <code>QuotaCode</code> response in the output for the quota you want.</p>
    pub fn quota_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.quota_code(input.into());
        self
    }
    /// <p>Specifies the quota identifier. To find the quota code for a specific quota, use the <code>ListServiceQuotas</code> operation, and look for the <code>QuotaCode</code> response in the output for the quota you want.</p>
    pub fn set_quota_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_quota_code(input);
        self
    }
    /// <p>Specifies the quota identifier. To find the quota code for a specific quota, use the <code>ListServiceQuotas</code> operation, and look for the <code>QuotaCode</code> response in the output for the quota you want.</p>
    pub fn get_quota_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_quota_code()
    }
}
