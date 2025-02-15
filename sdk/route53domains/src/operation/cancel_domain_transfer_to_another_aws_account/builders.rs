// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_domain_transfer_to_another_aws_account::_cancel_domain_transfer_to_another_aws_account_output::CancelDomainTransferToAnotherAwsAccountOutputBuilder;

pub use crate::operation::cancel_domain_transfer_to_another_aws_account::_cancel_domain_transfer_to_another_aws_account_input::CancelDomainTransferToAnotherAwsAccountInputBuilder;

impl CancelDomainTransferToAnotherAwsAccountInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_domain_transfer_to_another_aws_account();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelDomainTransferToAnotherAwsAccount`.
///
/// <p>Cancels the transfer of a domain from the current Amazon Web Services account to another Amazon Web Services account. You initiate a transfer betweenAmazon Web Services accounts using <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_TransferDomainToAnotherAwsAccount.html">TransferDomainToAnotherAwsAccount</a>. </p> <important>
/// <p>You must cancel the transfer before the other Amazon Web Services account accepts the transfer using <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_AcceptDomainTransferFromAnotherAwsAccount.html">AcceptDomainTransferFromAnotherAwsAccount</a>.</p>
/// </important>
/// <p>Use either <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ListOperations.html">ListOperations</a> or <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_GetOperationDetail.html">GetOperationDetail</a> to determine whether the operation succeeded. <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_GetOperationDetail.html">GetOperationDetail</a> provides additional information, for example, <code>Domain Transfer from Aws Account 111122223333 has been cancelled</code>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelDomainTransferToAnotherAwsAccountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_domain_transfer_to_another_aws_account::builders::CancelDomainTransferToAnotherAwsAccountInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountOutput,
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountError,
    > for CancelDomainTransferToAnotherAwsAccountFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountOutput,
            crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelDomainTransferToAnotherAwsAccountFluentBuilder {
    /// Creates a new `CancelDomainTransferToAnotherAwsAccount`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelDomainTransferToAnotherAwsAccount as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::cancel_domain_transfer_to_another_aws_account::builders::CancelDomainTransferToAnotherAwsAccountInputBuilder {
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
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccount::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccount::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountOutput,
        crate::operation::cancel_domain_transfer_to_another_aws_account::CancelDomainTransferToAnotherAwsAccountError,
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
    /// <p>The name of the domain for which you want to cancel the transfer to another Amazon Web Services account.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain for which you want to cancel the transfer to another Amazon Web Services account.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain for which you want to cancel the transfer to another Amazon Web Services account.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
}
