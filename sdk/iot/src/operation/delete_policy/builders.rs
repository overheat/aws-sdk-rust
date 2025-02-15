// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_policy::_delete_policy_output::DeletePolicyOutputBuilder;

pub use crate::operation::delete_policy::_delete_policy_input::DeletePolicyInputBuilder;

impl DeletePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_policy::DeletePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_policy::DeletePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeletePolicy`.
///
/// <p>Deletes the specified policy.</p>
/// <p>A policy cannot be deleted if it has non-default versions or it is attached to any certificate.</p>
/// <p>To delete a policy, use the <code>DeletePolicyVersion</code> action to delete all non-default versions of the policy; use the <code>DetachPolicy</code> action to detach the policy from any certificate; and then use the DeletePolicy action to delete the policy.</p>
/// <p>When a policy is deleted using DeletePolicy, its default version is deleted with it.</p> <note>
/// <p>Because of the distributed nature of Amazon Web Services, it can take up to five minutes after a policy is detached before it's ready to be deleted.</p>
/// </note>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeletePolicy</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeletePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_policy::builders::DeletePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_policy::DeletePolicyOutput,
        crate::operation::delete_policy::DeletePolicyError,
    > for DeletePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_policy::DeletePolicyOutput,
            crate::operation::delete_policy::DeletePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeletePolicyFluentBuilder {
    /// Creates a new `DeletePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeletePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_policy::builders::DeletePolicyInputBuilder {
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
        crate::operation::delete_policy::DeletePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_policy::DeletePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_policy::DeletePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_policy::DeletePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_policy::DeletePolicyOutput,
        crate::operation::delete_policy::DeletePolicyError,
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
    /// <p>The name of the policy to delete.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>The name of the policy to delete.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The name of the policy to delete.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
}
