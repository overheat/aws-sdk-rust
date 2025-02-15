// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_organization_config_rule_detailed_status::_get_organization_config_rule_detailed_status_output::GetOrganizationConfigRuleDetailedStatusOutputBuilder;

pub use crate::operation::get_organization_config_rule_detailed_status::_get_organization_config_rule_detailed_status_input::GetOrganizationConfigRuleDetailedStatusInputBuilder;

impl GetOrganizationConfigRuleDetailedStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_organization_config_rule_detailed_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetOrganizationConfigRuleDetailedStatus`.
///
/// <p>Returns detailed status for each member account within an organization for a given organization Config rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetOrganizationConfigRuleDetailedStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput,
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError,
    > for GetOrganizationConfigRuleDetailedStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput,
            crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetOrganizationConfigRuleDetailedStatusFluentBuilder {
    /// Creates a new `GetOrganizationConfigRuleDetailedStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetOrganizationConfigRuleDetailedStatus as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusInputBuilder {
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
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatus::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatus::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput,
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator {
        crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of your organization Config rule for which you want status details for member accounts.</p>
    pub fn organization_config_rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_config_rule_name(input.into());
        self
    }
    /// <p>The name of your organization Config rule for which you want status details for member accounts.</p>
    pub fn set_organization_config_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_config_rule_name(input);
        self
    }
    /// <p>The name of your organization Config rule for which you want status details for member accounts.</p>
    pub fn get_organization_config_rule_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_config_rule_name()
    }
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    pub fn filters(mut self, input: crate::types::StatusDetailFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<crate::types::StatusDetailFilters>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    pub fn get_filters(&self) -> &::std::option::Option<crate::types::StatusDetailFilters> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, Config uses the default. The default is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, Config uses the default. The default is 100.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, Config uses the default. The default is 100.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
