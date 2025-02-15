// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::close_account::_close_account_output::CloseAccountOutputBuilder;

pub use crate::operation::close_account::_close_account_input::CloseAccountInputBuilder;

impl CloseAccountInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::close_account::CloseAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::close_account::CloseAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.close_account();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CloseAccount`.
///
/// <p>Closes an Amazon Web Services member account within an organization. You can close an account when <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features are enabled </a>. You can't close the management account with this API. This is an asynchronous request that Amazon Web Services performs in the background. Because <code>CloseAccount</code> operates asynchronously, it can return a successful completion message even though account closure might still be in progress. You need to wait a few minutes before the account is fully closed. To check the status of the request, do one of the following:</p>
/// <ul>
/// <li> <p>Use the <code>AccountId</code> that you sent in the <code>CloseAccount</code> request to provide as a parameter to the <code>DescribeAccount</code> operation. </p> <p>While the close account request is in progress, Account status will indicate PENDING_CLOSURE. When the close account request completes, the status will change to SUSPENDED. </p> </li>
/// <li> <p>Check the CloudTrail log for the <code>CloseAccountResult</code> event that gets published after the account closes successfully. For information on using CloudTrail with Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_security_incident-response.html#orgs_cloudtrail-integration">Logging and monitoring in Organizations</a> in the <i>Organizations User Guide</i>.</p> </li>
/// </ul> <note>
/// <ul>
/// <li> <p>You can close only 10% of member accounts, between 10 and 200, within a rolling 30 day period. This quota is not bound by a calendar month, but starts when you close an account. After you reach this limit, you can close additional accounts. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing a member account in your organization</a> in the <i>Organizations User Guide</i>. </p> </li>
/// <li> <p>To reinstate a closed account, contact Amazon Web Services Support within the 90-day grace period while the account is in SUSPENDED status. </p> </li>
/// <li> <p>If the Amazon Web Services account you attempt to close is linked to an Amazon Web Services GovCloud (US) account, the <code>CloseAccount</code> request will close both accounts. To learn important pre-closure details, see <a href="https://docs.aws.amazon.com/govcloud-us/latest/UserGuide/Closing-govcloud-account.html"> Closing an Amazon Web Services GovCloud (US) account</a> in the <i> Amazon Web Services GovCloud User Guide</i>.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CloseAccountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::close_account::builders::CloseAccountInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::close_account::CloseAccountOutput,
        crate::operation::close_account::CloseAccountError,
    > for CloseAccountFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::close_account::CloseAccountOutput,
            crate::operation::close_account::CloseAccountError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CloseAccountFluentBuilder {
    /// Creates a new `CloseAccount`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CloseAccount as a reference.
    pub fn as_input(&self) -> &crate::operation::close_account::builders::CloseAccountInputBuilder {
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
        crate::operation::close_account::CloseAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::close_account::CloseAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::close_account::CloseAccount::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::close_account::CloseAccount::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::close_account::CloseAccountOutput,
        crate::operation::close_account::CloseAccountError,
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
    /// <p>Retrieves the Amazon Web Services account Id for the current <code>CloseAccount</code> API request. </p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>Retrieves the Amazon Web Services account Id for the current <code>CloseAccount</code> API request. </p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>Retrieves the Amazon Web Services account Id for the current <code>CloseAccount</code> API request. </p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
