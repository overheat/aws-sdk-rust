// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_time_series_service_statistics::_get_time_series_service_statistics_output::GetTimeSeriesServiceStatisticsOutputBuilder;

pub use crate::operation::get_time_series_service_statistics::_get_time_series_service_statistics_input::GetTimeSeriesServiceStatisticsInputBuilder;

impl GetTimeSeriesServiceStatisticsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_time_series_service_statistics();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTimeSeriesServiceStatistics`.
///
/// <p>Get an aggregation of service statistics defined by a specific time range.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTimeSeriesServiceStatisticsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_time_series_service_statistics::builders::GetTimeSeriesServiceStatisticsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsError,
    > for GetTimeSeriesServiceStatisticsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
            crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetTimeSeriesServiceStatisticsFluentBuilder {
    /// Creates a new `GetTimeSeriesServiceStatistics`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTimeSeriesServiceStatistics as a reference.
    pub fn as_input(&self) -> &crate::operation::get_time_series_service_statistics::builders::GetTimeSeriesServiceStatisticsInputBuilder {
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
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatistics::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatistics::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
        crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_time_series_service_statistics::paginator::GetTimeSeriesServiceStatisticsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_time_series_service_statistics::paginator::GetTimeSeriesServiceStatisticsPaginator {
        crate::operation::get_time_series_service_statistics::paginator::GetTimeSeriesServiceStatisticsPaginator::new(self.handle, self.inner)
    }
    /// <p>The start of the time frame for which to aggregate statistics.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start of the time frame for which to aggregate statistics.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The start of the time frame for which to aggregate statistics.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The end of the time frame for which to aggregate statistics.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end of the time frame for which to aggregate statistics.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The end of the time frame for which to aggregate statistics.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The case-sensitive name of the group for which to pull statistics from.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>The case-sensitive name of the group for which to pull statistics from.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>The case-sensitive name of the group for which to pull statistics from.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_name()
    }
    /// <p>The Amazon Resource Name (ARN) of the group for which to pull statistics from.</p>
    pub fn group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the group for which to pull statistics from.</p>
    pub fn set_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the group for which to pull statistics from.</p>
    pub fn get_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_arn()
    }
    /// <p>A filter expression defining entities that will be aggregated for statistics. Supports ID, service, and edge functions. If no selector expression is specified, edge statistics are returned. </p>
    pub fn entity_selector_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_selector_expression(input.into());
        self
    }
    /// <p>A filter expression defining entities that will be aggregated for statistics. Supports ID, service, and edge functions. If no selector expression is specified, edge statistics are returned. </p>
    pub fn set_entity_selector_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_selector_expression(input);
        self
    }
    /// <p>A filter expression defining entities that will be aggregated for statistics. Supports ID, service, and edge functions. If no selector expression is specified, edge statistics are returned. </p>
    pub fn get_entity_selector_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_entity_selector_expression()
    }
    /// <p>Aggregation period in seconds.</p>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p>Aggregation period in seconds.</p>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p>Aggregation period in seconds.</p>
    pub fn get_period(&self) -> &::std::option::Option<i32> {
        self.inner.get_period()
    }
    /// <p>The forecasted high and low fault count values. Forecast enabled requests require the EntitySelectorExpression ID be provided.</p>
    pub fn forecast_statistics(mut self, input: bool) -> Self {
        self.inner = self.inner.forecast_statistics(input);
        self
    }
    /// <p>The forecasted high and low fault count values. Forecast enabled requests require the EntitySelectorExpression ID be provided.</p>
    pub fn set_forecast_statistics(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_forecast_statistics(input);
        self
    }
    /// <p>The forecasted high and low fault count values. Forecast enabled requests require the EntitySelectorExpression ID be provided.</p>
    pub fn get_forecast_statistics(&self) -> &::std::option::Option<bool> {
        self.inner.get_forecast_statistics()
    }
    /// <p>Pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Pagination token.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
