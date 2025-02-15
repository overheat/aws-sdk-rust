// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_audit_suppressions::_list_audit_suppressions_output::ListAuditSuppressionsOutputBuilder;

pub use crate::operation::list_audit_suppressions::_list_audit_suppressions_input::ListAuditSuppressionsInputBuilder;

impl ListAuditSuppressionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_audit_suppressions::ListAuditSuppressionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_audit_suppressions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAuditSuppressions`.
///
/// <p> Lists your Device Defender audit listings. </p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListAuditSuppressions</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAuditSuppressionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput,
        crate::operation::list_audit_suppressions::ListAuditSuppressionsError,
    > for ListAuditSuppressionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput,
            crate::operation::list_audit_suppressions::ListAuditSuppressionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListAuditSuppressionsFluentBuilder {
    /// Creates a new `ListAuditSuppressions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAuditSuppressions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_audit_suppressions::builders::ListAuditSuppressionsInputBuilder {
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
        crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_audit_suppressions::ListAuditSuppressionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_audit_suppressions::ListAuditSuppressions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_audit_suppressions::ListAuditSuppressions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_audit_suppressions::ListAuditSuppressionsOutput,
        crate::operation::list_audit_suppressions::ListAuditSuppressionsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_audit_suppressions::paginator::ListAuditSuppressionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_audit_suppressions::paginator::ListAuditSuppressionsPaginator {
        crate::operation::list_audit_suppressions::paginator::ListAuditSuppressionsPaginator::new(self.handle, self.inner)
    }
    /// <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn check_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.check_name(input.into());
        self
    }
    /// <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn set_check_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_check_name(input);
        self
    }
    /// <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    pub fn get_check_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_check_name()
    }
    /// <p>Information that identifies the noncompliant resource.</p>
    pub fn resource_identifier(mut self, input: crate::types::ResourceIdentifier) -> Self {
        self.inner = self.inner.resource_identifier(input);
        self
    }
    /// <p>Information that identifies the noncompliant resource.</p>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<crate::types::ResourceIdentifier>) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// <p>Information that identifies the noncompliant resource.</p>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<crate::types::ResourceIdentifier> {
        self.inner.get_resource_identifier()
    }
    /// <p> Determines whether suppressions are listed in ascending order by expiration date or not. If parameter isn't provided, <code>ascendingOrder=true</code>. </p>
    pub fn ascending_order(mut self, input: bool) -> Self {
        self.inner = self.inner.ascending_order(input);
        self
    }
    /// <p> Determines whether suppressions are listed in ascending order by expiration date or not. If parameter isn't provided, <code>ascendingOrder=true</code>. </p>
    pub fn set_ascending_order(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ascending_order(input);
        self
    }
    /// <p> Determines whether suppressions are listed in ascending order by expiration date or not. If parameter isn't provided, <code>ascendingOrder=true</code>. </p>
    pub fn get_ascending_order(&self) -> &::std::option::Option<bool> {
        self.inner.get_ascending_order()
    }
    /// <p> The token for the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> The token for the next set of results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> The token for the next set of results. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p> The maximum number of results to return at one time. The default is 25. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> The maximum number of results to return at one time. The default is 25. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> The maximum number of results to return at one time. The default is 25. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
