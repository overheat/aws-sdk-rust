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
/// <p>Permanently deletes an Firewall Manager policy. </p>
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
    /// <p>The ID of the policy that you want to delete. You can retrieve this ID from <code>PutPolicy</code> and <code>ListPolicies</code>.</p>
    pub fn policy_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_id(input.into());
        self
    }
    /// <p>The ID of the policy that you want to delete. You can retrieve this ID from <code>PutPolicy</code> and <code>ListPolicies</code>.</p>
    pub fn set_policy_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_id(input);
        self
    }
    /// <p>The ID of the policy that you want to delete. You can retrieve this ID from <code>PutPolicy</code> and <code>ListPolicies</code>.</p>
    pub fn get_policy_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_id()
    }
    /// <p>If <code>True</code>, the request performs cleanup according to the policy type. </p>
    /// <p>For WAF and Shield Advanced policies, the cleanup does the following:</p>
    /// <ul>
    /// <li> <p>Deletes rule groups created by Firewall Manager</p> </li>
    /// <li> <p>Removes web ACLs from in-scope resources</p> </li>
    /// <li> <p>Deletes web ACLs that contain no rules or rule groups</p> </li>
    /// </ul>
    /// <p>For security group policies, the cleanup does the following for each security group in the policy:</p>
    /// <ul>
    /// <li> <p>Disassociates the security group from in-scope resources </p> </li>
    /// <li> <p>Deletes the security group if it was created through Firewall Manager and if it's no longer associated with any resources through another policy</p> </li>
    /// </ul>
    /// <p>After the cleanup, in-scope resources are no longer protected by web ACLs in this policy. Protection of out-of-scope resources remains unchanged. Scope is determined by tags that you create and accounts that you associate with the policy. When creating the policy, if you specify that only resources in specific accounts or with specific tags are in scope of the policy, those accounts and resources are handled by the policy. All others are out of scope. If you don't specify tags or accounts, all resources are in scope. </p>
    pub fn delete_all_policy_resources(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_all_policy_resources(input);
        self
    }
    /// <p>If <code>True</code>, the request performs cleanup according to the policy type. </p>
    /// <p>For WAF and Shield Advanced policies, the cleanup does the following:</p>
    /// <ul>
    /// <li> <p>Deletes rule groups created by Firewall Manager</p> </li>
    /// <li> <p>Removes web ACLs from in-scope resources</p> </li>
    /// <li> <p>Deletes web ACLs that contain no rules or rule groups</p> </li>
    /// </ul>
    /// <p>For security group policies, the cleanup does the following for each security group in the policy:</p>
    /// <ul>
    /// <li> <p>Disassociates the security group from in-scope resources </p> </li>
    /// <li> <p>Deletes the security group if it was created through Firewall Manager and if it's no longer associated with any resources through another policy</p> </li>
    /// </ul>
    /// <p>After the cleanup, in-scope resources are no longer protected by web ACLs in this policy. Protection of out-of-scope resources remains unchanged. Scope is determined by tags that you create and accounts that you associate with the policy. When creating the policy, if you specify that only resources in specific accounts or with specific tags are in scope of the policy, those accounts and resources are handled by the policy. All others are out of scope. If you don't specify tags or accounts, all resources are in scope. </p>
    pub fn set_delete_all_policy_resources(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_all_policy_resources(input);
        self
    }
    /// <p>If <code>True</code>, the request performs cleanup according to the policy type. </p>
    /// <p>For WAF and Shield Advanced policies, the cleanup does the following:</p>
    /// <ul>
    /// <li> <p>Deletes rule groups created by Firewall Manager</p> </li>
    /// <li> <p>Removes web ACLs from in-scope resources</p> </li>
    /// <li> <p>Deletes web ACLs that contain no rules or rule groups</p> </li>
    /// </ul>
    /// <p>For security group policies, the cleanup does the following for each security group in the policy:</p>
    /// <ul>
    /// <li> <p>Disassociates the security group from in-scope resources </p> </li>
    /// <li> <p>Deletes the security group if it was created through Firewall Manager and if it's no longer associated with any resources through another policy</p> </li>
    /// </ul>
    /// <p>After the cleanup, in-scope resources are no longer protected by web ACLs in this policy. Protection of out-of-scope resources remains unchanged. Scope is determined by tags that you create and accounts that you associate with the policy. When creating the policy, if you specify that only resources in specific accounts or with specific tags are in scope of the policy, those accounts and resources are handled by the policy. All others are out of scope. If you don't specify tags or accounts, all resources are in scope. </p>
    pub fn get_delete_all_policy_resources(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_all_policy_resources()
    }
}
