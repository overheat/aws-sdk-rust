// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_pricing_plans_associated_with_pricing_rule::_list_pricing_plans_associated_with_pricing_rule_output::ListPricingPlansAssociatedWithPricingRuleOutputBuilder;

pub use crate::operation::list_pricing_plans_associated_with_pricing_rule::_list_pricing_plans_associated_with_pricing_rule_input::ListPricingPlansAssociatedWithPricingRuleInputBuilder;

impl ListPricingPlansAssociatedWithPricingRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_pricing_plans_associated_with_pricing_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPricingPlansAssociatedWithPricingRule`.
///
/// <p> A list of the pricing plans that are associated with a pricing rule. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPricingPlansAssociatedWithPricingRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_pricing_plans_associated_with_pricing_rule::builders::ListPricingPlansAssociatedWithPricingRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleOutput,
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleError,
    > for ListPricingPlansAssociatedWithPricingRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleOutput,
            crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPricingPlansAssociatedWithPricingRuleFluentBuilder {
    /// Creates a new `ListPricingPlansAssociatedWithPricingRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPricingPlansAssociatedWithPricingRule as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::list_pricing_plans_associated_with_pricing_rule::builders::ListPricingPlansAssociatedWithPricingRuleInputBuilder {
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
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRule::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRule::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleOutput,
        crate::operation::list_pricing_plans_associated_with_pricing_rule::ListPricingPlansAssociatedWithPricingRuleError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_pricing_plans_associated_with_pricing_rule::paginator::ListPricingPlansAssociatedWithPricingRulePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_pricing_plans_associated_with_pricing_rule::paginator::ListPricingPlansAssociatedWithPricingRulePaginator {
        crate::operation::list_pricing_plans_associated_with_pricing_rule::paginator::ListPricingPlansAssociatedWithPricingRulePaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p> The pricing plan billing period for which associations will be listed. </p>
    pub fn billing_period(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.billing_period(input.into());
        self
    }
    /// <p> The pricing plan billing period for which associations will be listed. </p>
    pub fn set_billing_period(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_billing_period(input);
        self
    }
    /// <p> The pricing plan billing period for which associations will be listed. </p>
    pub fn get_billing_period(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_billing_period()
    }
    /// <p> The pricing rule Amazon Resource Name (ARN) for which associations will be listed. </p>
    pub fn pricing_rule_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pricing_rule_arn(input.into());
        self
    }
    /// <p> The pricing rule Amazon Resource Name (ARN) for which associations will be listed. </p>
    pub fn set_pricing_rule_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pricing_rule_arn(input);
        self
    }
    /// <p> The pricing rule Amazon Resource Name (ARN) for which associations will be listed. </p>
    pub fn get_pricing_rule_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pricing_rule_arn()
    }
    /// <p> The optional maximum number of pricing rule associations to retrieve. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> The optional maximum number of pricing rule associations to retrieve. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> The optional maximum number of pricing rule associations to retrieve. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p> The optional pagination token returned by a previous call. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> The optional pagination token returned by a previous call. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> The optional pagination token returned by a previous call. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
