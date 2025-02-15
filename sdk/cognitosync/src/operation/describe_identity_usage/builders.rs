// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_identity_usage::_describe_identity_usage_output::DescribeIdentityUsageOutputBuilder;

pub use crate::operation::describe_identity_usage::_describe_identity_usage_input::DescribeIdentityUsageInputBuilder;

impl DescribeIdentityUsageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_identity_usage::DescribeIdentityUsageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_identity_usage::DescribeIdentityUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_identity_usage();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeIdentityUsage`.
///
/// <p>Gets usage information for an identity, including number of datasets and data usage.</p>
/// <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p> <examples>
/// <example>
/// <name>
/// DescribeIdentityUsage
/// </name>
/// <description>
/// The following examples have been edited for readability.
/// </description>
/// <request>
/// POST / HTTP/1.1 CONTENT-TYPE: application/json X-AMZN-REQUESTID: 33f9b4e4-a177-4aad-a3bb-6edb7980b283 X-AMZ-TARGET: com.amazonaws.cognito.sync.model.AWSCognitoSyncService.DescribeIdentityUsage HOST: cognito-sync.us-east-1.amazonaws.com:443 X-AMZ-DATE: 20141111T215129Z AUTHORIZATION: AWS4-HMAC-SHA256 Credential=
/// <credential>
/// , SignedHeaders=content-type;host;x-amz-date;x-amz-target;x-amzn-requestid, Signature=
/// <signature>
/// { "Operation": "com.amazonaws.cognito.sync.model#DescribeIdentityUsage", "Service": "com.amazonaws.cognito.sync.model#AWSCognitoSyncService", "Input": { "IdentityPoolId": "IDENTITY_POOL_ID", "IdentityId": "IDENTITY_ID" } }
/// </signature>
/// </credential>
/// </request>
/// <response>
/// 1.1 200 OK x-amzn-requestid: 33f9b4e4-a177-4aad-a3bb-6edb7980b283 content-type: application/json content-length: 318 date: Tue, 11 Nov 2014 21:51:29 GMT { "Output": { "__type": "com.amazonaws.cognito.sync.model#DescribeIdentityUsageResponse", "IdentityUsage": { "DataStorage": 16, "DatasetCount": 1, "IdentityId": "IDENTITY_ID", "IdentityPoolId": "IDENTITY_POOL_ID", "LastModifiedDate": 1.412974081336E9 } }, "Version": "1.0" }
/// </response>
/// </example>
/// </examples>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeIdentityUsageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_identity_usage::builders::DescribeIdentityUsageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_identity_usage::DescribeIdentityUsageOutput,
        crate::operation::describe_identity_usage::DescribeIdentityUsageError,
    > for DescribeIdentityUsageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_identity_usage::DescribeIdentityUsageOutput,
            crate::operation::describe_identity_usage::DescribeIdentityUsageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeIdentityUsageFluentBuilder {
    /// Creates a new `DescribeIdentityUsage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeIdentityUsage as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_identity_usage::builders::DescribeIdentityUsageInputBuilder {
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
        crate::operation::describe_identity_usage::DescribeIdentityUsageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_identity_usage::DescribeIdentityUsageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_identity_usage::DescribeIdentityUsage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_identity_usage::DescribeIdentityUsage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_identity_usage::DescribeIdentityUsageOutput,
        crate::operation::describe_identity_usage::DescribeIdentityUsageError,
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
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn identity_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_pool_id(input.into());
        self
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn set_identity_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_pool_id(input);
        self
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn get_identity_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_pool_id()
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn identity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_id(input.into());
        self
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn set_identity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_id(input);
        self
    }
    /// A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    pub fn get_identity_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_id()
    }
}
