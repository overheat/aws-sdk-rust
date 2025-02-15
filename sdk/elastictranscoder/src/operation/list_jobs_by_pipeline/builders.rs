// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_jobs_by_pipeline::_list_jobs_by_pipeline_output::ListJobsByPipelineOutputBuilder;

pub use crate::operation::list_jobs_by_pipeline::_list_jobs_by_pipeline_input::ListJobsByPipelineInputBuilder;

impl ListJobsByPipelineInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs_by_pipeline::ListJobsByPipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_jobs_by_pipeline();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListJobsByPipeline`.
///
/// <p>The ListJobsByPipeline operation gets a list of the jobs currently in a pipeline.</p>
/// <p>Elastic Transcoder returns all of the jobs currently in the specified pipeline. The response body contains one element for each job that satisfies the search criteria.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListJobsByPipelineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_jobs_by_pipeline::builders::ListJobsByPipelineInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineOutput,
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineError,
    > for ListJobsByPipelineFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_jobs_by_pipeline::ListJobsByPipelineOutput,
            crate::operation::list_jobs_by_pipeline::ListJobsByPipelineError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListJobsByPipelineFluentBuilder {
    /// Creates a new `ListJobsByPipeline`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListJobsByPipeline as a reference.
    pub fn as_input(&self) -> &crate::operation::list_jobs_by_pipeline::builders::ListJobsByPipelineInputBuilder {
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
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs_by_pipeline::ListJobsByPipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_jobs_by_pipeline::ListJobsByPipeline::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_jobs_by_pipeline::ListJobsByPipeline::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineOutput,
        crate::operation::list_jobs_by_pipeline::ListJobsByPipelineError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_jobs_by_pipeline::paginator::ListJobsByPipelinePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_jobs_by_pipeline::paginator::ListJobsByPipelinePaginator {
        crate::operation::list_jobs_by_pipeline::paginator::ListJobsByPipelinePaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the pipeline for which you want to get job information.</p>
    pub fn pipeline_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pipeline_id(input.into());
        self
    }
    /// <p>The ID of the pipeline for which you want to get job information.</p>
    pub fn set_pipeline_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pipeline_id(input);
        self
    }
    /// <p>The ID of the pipeline for which you want to get job information.</p>
    pub fn get_pipeline_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pipeline_id()
    }
    /// <p> To list jobs in chronological order by the date and time that they were submitted, enter <code>true</code>. To list jobs in reverse chronological order, enter <code>false</code>. </p>
    pub fn ascending(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ascending(input.into());
        self
    }
    /// <p> To list jobs in chronological order by the date and time that they were submitted, enter <code>true</code>. To list jobs in reverse chronological order, enter <code>false</code>. </p>
    pub fn set_ascending(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ascending(input);
        self
    }
    /// <p> To list jobs in chronological order by the date and time that they were submitted, enter <code>true</code>. To list jobs in reverse chronological order, enter <code>false</code>. </p>
    pub fn get_ascending(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ascending()
    }
    /// <p> When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results. </p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p> When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results. </p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p> When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results. </p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
}
