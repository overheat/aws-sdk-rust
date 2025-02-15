// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_account_password_policy::_get_account_password_policy_output::GetAccountPasswordPolicyOutputBuilder;

pub use crate::operation::get_account_password_policy::_get_account_password_policy_input::GetAccountPasswordPolicyInputBuilder;

impl GetAccountPasswordPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_account_password_policy::GetAccountPasswordPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_account_password_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAccountPasswordPolicy`.
///
/// <p>Retrieves the password policy for the Amazon Web Services account. This tells you the complexity requirements and mandatory rotation periods for the IAM user passwords in your account. For more information about using a password policy, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_ManagingPasswordPolicies.html">Managing an IAM password policy</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAccountPasswordPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_account_password_policy::builders::GetAccountPasswordPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput,
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyError,
    > for GetAccountPasswordPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput,
            crate::operation::get_account_password_policy::GetAccountPasswordPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAccountPasswordPolicyFluentBuilder {
    /// Creates a new `GetAccountPasswordPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAccountPasswordPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::get_account_password_policy::builders::GetAccountPasswordPolicyInputBuilder {
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
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_account_password_policy::GetAccountPasswordPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_account_password_policy::GetAccountPasswordPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_account_password_policy::GetAccountPasswordPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyOutput,
        crate::operation::get_account_password_policy::GetAccountPasswordPolicyError,
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
}
