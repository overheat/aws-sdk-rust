// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_data_retrieval_policy::_set_data_retrieval_policy_output::SetDataRetrievalPolicyOutputBuilder;

pub use crate::operation::set_data_retrieval_policy::_set_data_retrieval_policy_input::SetDataRetrievalPolicyInputBuilder;

impl SetDataRetrievalPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_data_retrieval_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetDataRetrievalPolicy`.
///
/// <p>This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful PUT operation.</p>
/// <p>The set policy operation does not affect retrieval jobs that were in progress before the policy was enacted. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetDataRetrievalPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_data_retrieval_policy::builders::SetDataRetrievalPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyOutput,
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyError,
    > for SetDataRetrievalPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyOutput,
            crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetDataRetrievalPolicyFluentBuilder {
    /// Creates a new `SetDataRetrievalPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetDataRetrievalPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::set_data_retrieval_policy::builders::SetDataRetrievalPolicyInputBuilder {
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
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyOutput,
        crate::operation::set_data_retrieval_policy::SetDataRetrievalPolicyError,
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
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The data retrieval policy in JSON format.</p>
    pub fn policy(mut self, input: crate::types::DataRetrievalPolicy) -> Self {
        self.inner = self.inner.policy(input);
        self
    }
    /// <p>The data retrieval policy in JSON format.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<crate::types::DataRetrievalPolicy>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>The data retrieval policy in JSON format.</p>
    pub fn get_policy(&self) -> &::std::option::Option<crate::types::DataRetrievalPolicy> {
        self.inner.get_policy()
    }
}
