// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_pipeline_executions::_list_pipeline_executions_output::ListPipelineExecutionsOutputBuilder;

pub use crate::operation::list_pipeline_executions::_list_pipeline_executions_input::ListPipelineExecutionsInputBuilder;

impl ListPipelineExecutionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_pipeline_executions::ListPipelineExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pipeline_executions::ListPipelineExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_pipeline_executions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPipelineExecutions`.
///
/// <p>Gets a summary of the most recent executions for a pipeline.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPipelineExecutionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_pipeline_executions::builders::ListPipelineExecutionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_pipeline_executions::ListPipelineExecutionsOutput,
        crate::operation::list_pipeline_executions::ListPipelineExecutionsError,
    > for ListPipelineExecutionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_pipeline_executions::ListPipelineExecutionsOutput,
            crate::operation::list_pipeline_executions::ListPipelineExecutionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPipelineExecutionsFluentBuilder {
    /// Creates a new `ListPipelineExecutions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPipelineExecutions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_pipeline_executions::builders::ListPipelineExecutionsInputBuilder {
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
        crate::operation::list_pipeline_executions::ListPipelineExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pipeline_executions::ListPipelineExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_pipeline_executions::ListPipelineExecutions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_pipeline_executions::ListPipelineExecutions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_pipeline_executions::ListPipelineExecutionsOutput,
        crate::operation::list_pipeline_executions::ListPipelineExecutionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_pipeline_executions::paginator::ListPipelineExecutionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_pipeline_executions::paginator::ListPipelineExecutionsPaginator {
        crate::operation::list_pipeline_executions::paginator::ListPipelineExecutionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn pipeline_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pipeline_name(input.into());
        self
    }
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn set_pipeline_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pipeline_name(input);
        self
    }
    /// <p>The name of the pipeline for which you want to get execution summary information.</p>
    pub fn get_pipeline_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pipeline_name()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned nextToken value. Pipeline history is limited to the most recent 12 months, based on pipeline execution start times. Default value is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token that was returned from the previous <code>ListPipelineExecutions</code> call, which can be used to return the next set of pipeline executions in the list.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
