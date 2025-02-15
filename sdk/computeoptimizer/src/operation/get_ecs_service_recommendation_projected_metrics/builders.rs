// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ecs_service_recommendation_projected_metrics::_get_ecs_service_recommendation_projected_metrics_output::GetEcsServiceRecommendationProjectedMetricsOutputBuilder;

pub use crate::operation::get_ecs_service_recommendation_projected_metrics::_get_ecs_service_recommendation_projected_metrics_input::GetEcsServiceRecommendationProjectedMetricsInputBuilder;

impl GetEcsServiceRecommendationProjectedMetricsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetEcsServiceRecommendationProjectedMetricsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetricsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_ecs_service_recommendation_projected_metrics();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetECSServiceRecommendationProjectedMetrics`.
///
/// <p> Returns the projected metrics of Amazon ECS service recommendations. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetECSServiceRecommendationProjectedMetricsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_ecs_service_recommendation_projected_metrics::builders::GetEcsServiceRecommendationProjectedMetricsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetEcsServiceRecommendationProjectedMetricsOutput,
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetricsError,
    > for GetECSServiceRecommendationProjectedMetricsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_ecs_service_recommendation_projected_metrics::GetEcsServiceRecommendationProjectedMetricsOutput,
            crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetricsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetECSServiceRecommendationProjectedMetricsFluentBuilder {
    /// Creates a new `GetECSServiceRecommendationProjectedMetrics`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetECSServiceRecommendationProjectedMetrics as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::get_ecs_service_recommendation_projected_metrics::builders::GetEcsServiceRecommendationProjectedMetricsInputBuilder {
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
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetEcsServiceRecommendationProjectedMetricsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetricsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetrics::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetrics::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetEcsServiceRecommendationProjectedMetricsOutput,
        crate::operation::get_ecs_service_recommendation_projected_metrics::GetECSServiceRecommendationProjectedMetricsError,
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
    /// <p> The ARN that identifies the Amazon ECS service. </p>
    /// <p> The following is the format of the ARN: </p>
    /// <p> <code>arn:aws:ecs:region:aws_account_id:service/cluster-name/service-name</code> </p>
    pub fn service_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_arn(input.into());
        self
    }
    /// <p> The ARN that identifies the Amazon ECS service. </p>
    /// <p> The following is the format of the ARN: </p>
    /// <p> <code>arn:aws:ecs:region:aws_account_id:service/cluster-name/service-name</code> </p>
    pub fn set_service_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_arn(input);
        self
    }
    /// <p> The ARN that identifies the Amazon ECS service. </p>
    /// <p> The following is the format of the ARN: </p>
    /// <p> <code>arn:aws:ecs:region:aws_account_id:service/cluster-name/service-name</code> </p>
    pub fn get_service_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_arn()
    }
    /// <p> The statistic of the projected metrics. </p>
    pub fn stat(mut self, input: crate::types::MetricStatistic) -> Self {
        self.inner = self.inner.stat(input);
        self
    }
    /// <p> The statistic of the projected metrics. </p>
    pub fn set_stat(mut self, input: ::std::option::Option<crate::types::MetricStatistic>) -> Self {
        self.inner = self.inner.set_stat(input);
        self
    }
    /// <p> The statistic of the projected metrics. </p>
    pub fn get_stat(&self) -> &::std::option::Option<crate::types::MetricStatistic> {
        self.inner.get_stat()
    }
    /// <p> The granularity, in seconds, of the projected metrics data points. </p>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p> The granularity, in seconds, of the projected metrics data points. </p>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p> The granularity, in seconds, of the projected metrics data points. </p>
    pub fn get_period(&self) -> &::std::option::Option<i32> {
        self.inner.get_period()
    }
    /// <p> The timestamp of the first projected metrics data point to return. </p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p> The timestamp of the first projected metrics data point to return. </p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p> The timestamp of the first projected metrics data point to return. </p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p> The timestamp of the last projected metrics data point to return. </p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p> The timestamp of the last projected metrics data point to return. </p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p> The timestamp of the last projected metrics data point to return. </p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
}
