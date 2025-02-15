// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_auto_ml_jobs::_list_auto_ml_jobs_output::ListAutoMlJobsOutputBuilder;

pub use crate::operation::list_auto_ml_jobs::_list_auto_ml_jobs_input::ListAutoMlJobsInputBuilder;

impl ListAutoMlJobsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_auto_ml_jobs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAutoMLJobs`.
///
/// <p>Request a list of jobs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAutoMLJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_auto_ml_jobs::builders::ListAutoMlJobsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
    > for ListAutoMLJobsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListAutoMLJobsFluentBuilder {
    /// Creates a new `ListAutoMLJobs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAutoMLJobs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_auto_ml_jobs::builders::ListAutoMlJobsInputBuilder {
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
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_auto_ml_jobs::ListAutoMLJobs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_auto_ml_jobs::ListAutoMLJobs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_auto_ml_jobs::ListAutoMlJobsOutput,
        crate::operation::list_auto_ml_jobs::ListAutoMLJobsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator {
        crate::operation::list_auto_ml_jobs::paginator::ListAutoMlJobsPaginator::new(self.handle, self.inner)
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_creation_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn get_creation_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_after()
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_creation_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn get_creation_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_before()
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn last_modified_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_last_modified_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_modified_time_after(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn get_last_modified_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_modified_time_after()
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn last_modified_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.last_modified_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn set_last_modified_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_last_modified_time_before(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for time.</p>
    pub fn get_last_modified_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_last_modified_time_before()
    }
    /// <p>Request a list of jobs, using a search filter for name.</p>
    pub fn name_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>Request a list of jobs, using a search filter for name.</p>
    pub fn set_name_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>Request a list of jobs, using a search filter for name.</p>
    pub fn get_name_contains(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_contains()
    }
    /// <p>Request a list of jobs, using a filter for status.</p>
    pub fn status_equals(mut self, input: crate::types::AutoMlJobStatus) -> Self {
        self.inner = self.inner.status_equals(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for status.</p>
    pub fn set_status_equals(mut self, input: ::std::option::Option<crate::types::AutoMlJobStatus>) -> Self {
        self.inner = self.inner.set_status_equals(input);
        self
    }
    /// <p>Request a list of jobs, using a filter for status.</p>
    pub fn get_status_equals(&self) -> &::std::option::Option<crate::types::AutoMlJobStatus> {
        self.inner.get_status_equals()
    }
    /// <p>The sort order for the results. The default is <code>Descending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::AutoMlSortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order for the results. The default is <code>Descending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::AutoMlSortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The sort order for the results. The default is <code>Descending</code>.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::AutoMlSortOrder> {
        self.inner.get_sort_order()
    }
    /// <p>The parameter by which to sort the results. The default is <code>Name</code>.</p>
    pub fn sort_by(mut self, input: crate::types::AutoMlSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The parameter by which to sort the results. The default is <code>Name</code>.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::AutoMlSortBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The parameter by which to sort the results. The default is <code>Name</code>.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::AutoMlSortBy> {
        self.inner.get_sort_by()
    }
    /// <p>Request a list of jobs up to a specified limit.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Request a list of jobs up to a specified limit.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Request a list of jobs up to a specified limit.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If the previous response was truncated, you receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous response was truncated, you receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the previous response was truncated, you receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
