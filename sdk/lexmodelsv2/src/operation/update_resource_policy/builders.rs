// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_resource_policy::_update_resource_policy_output::UpdateResourcePolicyOutputBuilder;

pub use crate::operation::update_resource_policy::_update_resource_policy_input::UpdateResourcePolicyInputBuilder;

impl UpdateResourcePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_resource_policy::UpdateResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_resource_policy::UpdateResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_resource_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateResourcePolicy`.
///
/// <p>Replaces the existing resource policy for a bot or bot alias with a new one. If the policy doesn't exist, Amazon Lex returns an exception.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateResourcePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_resource_policy::builders::UpdateResourcePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_resource_policy::UpdateResourcePolicyOutput,
        crate::operation::update_resource_policy::UpdateResourcePolicyError,
    > for UpdateResourcePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_resource_policy::UpdateResourcePolicyOutput,
            crate::operation::update_resource_policy::UpdateResourcePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateResourcePolicyFluentBuilder {
    /// Creates a new `UpdateResourcePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateResourcePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::update_resource_policy::builders::UpdateResourcePolicyInputBuilder {
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
        crate::operation::update_resource_policy::UpdateResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_resource_policy::UpdateResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_resource_policy::UpdateResourcePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_resource_policy::UpdateResourcePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_resource_policy::UpdateResourcePolicyOutput,
        crate::operation::update_resource_policy::UpdateResourcePolicyError,
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
    /// <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bot or bot alias that the resource policy is attached to.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>A resource policy to add to the resource. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy reference </a>. </p>
    /// <p>If the policy isn't valid, Amazon Lex returns a validation exception.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>A resource policy to add to the resource. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy reference </a>. </p>
    /// <p>If the policy isn't valid, Amazon Lex returns a validation exception.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>A resource policy to add to the resource. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy reference </a>. </p>
    /// <p>If the policy isn't valid, Amazon Lex returns a validation exception.</p>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy()
    }
    /// <p>The identifier of the revision of the policy to update. If this revision ID doesn't match the current revision ID, Amazon Lex throws an exception.</p>
    /// <p>If you don't specify a revision, Amazon Lex overwrites the contents of the policy with the new values.</p>
    pub fn expected_revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_revision_id(input.into());
        self
    }
    /// <p>The identifier of the revision of the policy to update. If this revision ID doesn't match the current revision ID, Amazon Lex throws an exception.</p>
    /// <p>If you don't specify a revision, Amazon Lex overwrites the contents of the policy with the new values.</p>
    pub fn set_expected_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_revision_id(input);
        self
    }
    /// <p>The identifier of the revision of the policy to update. If this revision ID doesn't match the current revision ID, Amazon Lex throws an exception.</p>
    /// <p>If you don't specify a revision, Amazon Lex overwrites the contents of the policy with the new values.</p>
    pub fn get_expected_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_revision_id()
    }
}
