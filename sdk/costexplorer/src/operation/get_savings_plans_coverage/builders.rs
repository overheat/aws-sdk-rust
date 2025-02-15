// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_savings_plans_coverage::_get_savings_plans_coverage_output::GetSavingsPlansCoverageOutputBuilder;

pub use crate::operation::get_savings_plans_coverage::_get_savings_plans_coverage_input::GetSavingsPlansCoverageInputBuilder;

impl GetSavingsPlansCoverageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_savings_plans_coverage();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetSavingsPlansCoverage`.
///
/// <p>Retrieves the Savings Plans covered for your account. This enables you to see how much of your cost is covered by a Savings Plan. An organization’s management account can see the coverage of the associated member accounts. This supports dimensions, Cost Categories, and nested expressions. For any time period, you can filter data for Savings Plans usage with the following dimensions:</p>
/// <ul>
/// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
/// <li> <p> <code>REGION</code> </p> </li>
/// <li> <p> <code>SERVICE</code> </p> </li>
/// <li> <p> <code>INSTANCE_FAMILY</code> </p> </li>
/// </ul>
/// <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSavingsPlansCoverageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_savings_plans_coverage::builders::GetSavingsPlansCoverageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageOutput,
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageError,
    > for GetSavingsPlansCoverageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageOutput,
            crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetSavingsPlansCoverageFluentBuilder {
    /// Creates a new `GetSavingsPlansCoverage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetSavingsPlansCoverage as a reference.
    pub fn as_input(&self) -> &crate::operation::get_savings_plans_coverage::builders::GetSavingsPlansCoverageInputBuilder {
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
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageOutput,
        crate::operation::get_savings_plans_coverage::GetSavingsPlansCoverageError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_savings_plans_coverage::paginator::GetSavingsPlansCoveragePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_savings_plans_coverage::paginator::GetSavingsPlansCoveragePaginator {
        crate::operation::get_savings_plans_coverage::paginator::GetSavingsPlansCoveragePaginator::new(self.handle, self.inner)
    }
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    pub fn time_period(mut self, input: crate::types::DateInterval) -> Self {
        self.inner = self.inner.time_period(input);
        self
    }
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    pub fn set_time_period(mut self, input: ::std::option::Option<crate::types::DateInterval>) -> Self {
        self.inner = self.inner.set_time_period(input);
        self
    }
    /// <p>The time period that you want the usage and costs for. The <code>Start</code> date must be within 13 months. The <code>End</code> date must be after the <code>Start</code> date, and before the current date. Future dates can't be used as an <code>End</code> date.</p>
    pub fn get_time_period(&self) -> &::std::option::Option<crate::types::DateInterval> {
        self.inner.get_time_period()
    }
    /// Appends an item to `GroupBy`.
    ///
    /// To override the contents of this collection use [`set_group_by`](Self::set_group_by).
    ///
    /// <p>You can group the data using the attributes <code>INSTANCE_FAMILY</code>, <code>REGION</code>, or <code>SERVICE</code>.</p>
    pub fn group_by(mut self, input: crate::types::GroupDefinition) -> Self {
        self.inner = self.inner.group_by(input);
        self
    }
    /// <p>You can group the data using the attributes <code>INSTANCE_FAMILY</code>, <code>REGION</code>, or <code>SERVICE</code>.</p>
    pub fn set_group_by(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GroupDefinition>>) -> Self {
        self.inner = self.inner.set_group_by(input);
        self
    }
    /// <p>You can group the data using the attributes <code>INSTANCE_FAMILY</code>, <code>REGION</code>, or <code>SERVICE</code>.</p>
    pub fn get_group_by(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GroupDefinition>> {
        self.inner.get_group_by()
    }
    /// <p>The granularity of the Amazon Web Services cost data for your Savings Plans. <code>Granularity</code> can't be set if <code>GroupBy</code> is set.</p>
    /// <p>The <code>GetSavingsPlansCoverage</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    pub fn granularity(mut self, input: crate::types::Granularity) -> Self {
        self.inner = self.inner.granularity(input);
        self
    }
    /// <p>The granularity of the Amazon Web Services cost data for your Savings Plans. <code>Granularity</code> can't be set if <code>GroupBy</code> is set.</p>
    /// <p>The <code>GetSavingsPlansCoverage</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    pub fn set_granularity(mut self, input: ::std::option::Option<crate::types::Granularity>) -> Self {
        self.inner = self.inner.set_granularity(input);
        self
    }
    /// <p>The granularity of the Amazon Web Services cost data for your Savings Plans. <code>Granularity</code> can't be set if <code>GroupBy</code> is set.</p>
    /// <p>The <code>GetSavingsPlansCoverage</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    pub fn get_granularity(&self) -> &::std::option::Option<crate::types::Granularity> {
        self.inner.get_granularity()
    }
    /// <p>Filters Savings Plans coverage data by dimensions. You can filter data for Savings Plans usage with the following dimensions:</p>
    /// <ul>
    /// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
    /// <li> <p> <code>REGION</code> </p> </li>
    /// <li> <p> <code>SERVICE</code> </p> </li>
    /// <li> <p> <code>INSTANCE_FAMILY</code> </p> </li>
    /// </ul>
    /// <p> <code>GetSavingsPlansCoverage</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension. If there are multiple values for a dimension, they are OR'd together.</p>
    /// <p>Cost category is also supported.</p>
    pub fn filter(mut self, input: crate::types::Expression) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>Filters Savings Plans coverage data by dimensions. You can filter data for Savings Plans usage with the following dimensions:</p>
    /// <ul>
    /// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
    /// <li> <p> <code>REGION</code> </p> </li>
    /// <li> <p> <code>SERVICE</code> </p> </li>
    /// <li> <p> <code>INSTANCE_FAMILY</code> </p> </li>
    /// </ul>
    /// <p> <code>GetSavingsPlansCoverage</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension. If there are multiple values for a dimension, they are OR'd together.</p>
    /// <p>Cost category is also supported.</p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::Expression>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Filters Savings Plans coverage data by dimensions. You can filter data for Savings Plans usage with the following dimensions:</p>
    /// <ul>
    /// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
    /// <li> <p> <code>REGION</code> </p> </li>
    /// <li> <p> <code>SERVICE</code> </p> </li>
    /// <li> <p> <code>INSTANCE_FAMILY</code> </p> </li>
    /// </ul>
    /// <p> <code>GetSavingsPlansCoverage</code> uses the same <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> object as the other operations, but only <code>AND</code> is supported among each dimension. If there are multiple values for a dimension, they are OR'd together.</p>
    /// <p>Cost category is also supported.</p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::Expression> {
        self.inner.get_filter()
    }
    /// Appends an item to `Metrics`.
    ///
    /// To override the contents of this collection use [`set_metrics`](Self::set_metrics).
    ///
    /// <p>The measurement that you want your Savings Plans coverage reported in. The only valid value is <code>SpendCoveredBySavingsPlans</code>.</p>
    pub fn metrics(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metrics(input.into());
        self
    }
    /// <p>The measurement that you want your Savings Plans coverage reported in. The only valid value is <code>SpendCoveredBySavingsPlans</code>.</p>
    pub fn set_metrics(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_metrics(input);
        self
    }
    /// <p>The measurement that you want your Savings Plans coverage reported in. The only valid value is <code>SpendCoveredBySavingsPlans</code>.</p>
    pub fn get_metrics(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_metrics()
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The number of items to be returned in a response. The default is <code>20</code>, with a minimum value of <code>1</code>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of items to be returned in a response. The default is <code>20</code>, with a minimum value of <code>1</code>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The number of items to be returned in a response. The default is <code>20</code>, with a minimum value of <code>1</code>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The value that you want to sort the data by.</p>
    /// <p>The following values are supported for <code>Key</code>:</p>
    /// <ul>
    /// <li> <p> <code>SpendCoveredBySavingsPlan</code> </p> </li>
    /// <li> <p> <code>OnDemandCost</code> </p> </li>
    /// <li> <p> <code>CoveragePercentage</code> </p> </li>
    /// <li> <p> <code>TotalCost</code> </p> </li>
    /// <li> <p> <code>InstanceFamily</code> </p> </li>
    /// <li> <p> <code>Region</code> </p> </li>
    /// <li> <p> <code>Service</code> </p> </li>
    /// </ul>
    /// <p>The supported values for <code>SortOrder</code> are <code>ASCENDING</code> and <code>DESCENDING</code>.</p>
    pub fn sort_by(mut self, input: crate::types::SortDefinition) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The value that you want to sort the data by.</p>
    /// <p>The following values are supported for <code>Key</code>:</p>
    /// <ul>
    /// <li> <p> <code>SpendCoveredBySavingsPlan</code> </p> </li>
    /// <li> <p> <code>OnDemandCost</code> </p> </li>
    /// <li> <p> <code>CoveragePercentage</code> </p> </li>
    /// <li> <p> <code>TotalCost</code> </p> </li>
    /// <li> <p> <code>InstanceFamily</code> </p> </li>
    /// <li> <p> <code>Region</code> </p> </li>
    /// <li> <p> <code>Service</code> </p> </li>
    /// </ul>
    /// <p>The supported values for <code>SortOrder</code> are <code>ASCENDING</code> and <code>DESCENDING</code>.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::SortDefinition>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The value that you want to sort the data by.</p>
    /// <p>The following values are supported for <code>Key</code>:</p>
    /// <ul>
    /// <li> <p> <code>SpendCoveredBySavingsPlan</code> </p> </li>
    /// <li> <p> <code>OnDemandCost</code> </p> </li>
    /// <li> <p> <code>CoveragePercentage</code> </p> </li>
    /// <li> <p> <code>TotalCost</code> </p> </li>
    /// <li> <p> <code>InstanceFamily</code> </p> </li>
    /// <li> <p> <code>Region</code> </p> </li>
    /// <li> <p> <code>Service</code> </p> </li>
    /// </ul>
    /// <p>The supported values for <code>SortOrder</code> are <code>ASCENDING</code> and <code>DESCENDING</code>.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::SortDefinition> {
        self.inner.get_sort_by()
    }
}
