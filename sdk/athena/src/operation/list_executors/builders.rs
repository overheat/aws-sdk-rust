// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_executors::_list_executors_output::ListExecutorsOutputBuilder;

pub use crate::operation::list_executors::_list_executors_input::ListExecutorsInputBuilder;

impl ListExecutorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_executors::ListExecutorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_executors::ListExecutorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_executors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListExecutors`.
///
/// <p>Lists, in descending order, the executors that joined a session. Newer executors are listed first; older executors are listed later. The result can be optionally filtered by state.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListExecutorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_executors::builders::ListExecutorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_executors::ListExecutorsOutput,
        crate::operation::list_executors::ListExecutorsError,
    > for ListExecutorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_executors::ListExecutorsOutput,
            crate::operation::list_executors::ListExecutorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListExecutorsFluentBuilder {
    /// Creates a new `ListExecutors`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListExecutors as a reference.
    pub fn as_input(&self) -> &crate::operation::list_executors::builders::ListExecutorsInputBuilder {
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
        crate::operation::list_executors::ListExecutorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_executors::ListExecutorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_executors::ListExecutors::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_executors::ListExecutors::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_executors::ListExecutorsOutput,
        crate::operation::list_executors::ListExecutorsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_executors::paginator::ListExecutorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_executors::paginator::ListExecutorsPaginator {
        crate::operation::list_executors::paginator::ListExecutorsPaginator::new(self.handle, self.inner)
    }
    /// <p>The session ID.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The session ID.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The session ID.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// <p>A filter for a specific executor state. A description of each state follows.</p>
    /// <p> <code>CREATING</code> - The executor is being started, including acquiring resources.</p>
    /// <p> <code>CREATED</code> - The executor has been started.</p>
    /// <p> <code>REGISTERED</code> - The executor has been registered.</p>
    /// <p> <code>TERMINATING</code> - The executor is in the process of shutting down.</p>
    /// <p> <code>TERMINATED</code> - The executor is no longer running.</p>
    /// <p> <code>FAILED</code> - Due to a failure, the executor is no longer running.</p>
    pub fn executor_state_filter(mut self, input: crate::types::ExecutorState) -> Self {
        self.inner = self.inner.executor_state_filter(input);
        self
    }
    /// <p>A filter for a specific executor state. A description of each state follows.</p>
    /// <p> <code>CREATING</code> - The executor is being started, including acquiring resources.</p>
    /// <p> <code>CREATED</code> - The executor has been started.</p>
    /// <p> <code>REGISTERED</code> - The executor has been registered.</p>
    /// <p> <code>TERMINATING</code> - The executor is in the process of shutting down.</p>
    /// <p> <code>TERMINATED</code> - The executor is no longer running.</p>
    /// <p> <code>FAILED</code> - Due to a failure, the executor is no longer running.</p>
    pub fn set_executor_state_filter(mut self, input: ::std::option::Option<crate::types::ExecutorState>) -> Self {
        self.inner = self.inner.set_executor_state_filter(input);
        self
    }
    /// <p>A filter for a specific executor state. A description of each state follows.</p>
    /// <p> <code>CREATING</code> - The executor is being started, including acquiring resources.</p>
    /// <p> <code>CREATED</code> - The executor has been started.</p>
    /// <p> <code>REGISTERED</code> - The executor has been registered.</p>
    /// <p> <code>TERMINATING</code> - The executor is in the process of shutting down.</p>
    /// <p> <code>TERMINATED</code> - The executor is no longer running.</p>
    /// <p> <code>FAILED</code> - Due to a failure, the executor is no longer running.</p>
    pub fn get_executor_state_filter(&self) -> &::std::option::Option<crate::types::ExecutorState> {
        self.inner.get_executor_state_filter()
    }
    /// <p>The maximum number of executors to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of executors to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of executors to return.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
