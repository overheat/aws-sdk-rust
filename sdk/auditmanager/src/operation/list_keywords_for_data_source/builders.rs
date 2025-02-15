// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_keywords_for_data_source::_list_keywords_for_data_source_output::ListKeywordsForDataSourceOutputBuilder;

pub use crate::operation::list_keywords_for_data_source::_list_keywords_for_data_source_input::ListKeywordsForDataSourceInputBuilder;

impl ListKeywordsForDataSourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_keywords_for_data_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListKeywordsForDataSource`.
///
/// <p> Returns a list of keywords that are pre-mapped to the specified control data source. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListKeywordsForDataSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_keywords_for_data_source::builders::ListKeywordsForDataSourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceOutput,
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceError,
    > for ListKeywordsForDataSourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceOutput,
            crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListKeywordsForDataSourceFluentBuilder {
    /// Creates a new `ListKeywordsForDataSource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListKeywordsForDataSource as a reference.
    pub fn as_input(&self) -> &crate::operation::list_keywords_for_data_source::builders::ListKeywordsForDataSourceInputBuilder {
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
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_keywords_for_data_source::ListKeywordsForDataSource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceOutput,
        crate::operation::list_keywords_for_data_source::ListKeywordsForDataSourceError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_keywords_for_data_source::paginator::ListKeywordsForDataSourcePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_keywords_for_data_source::paginator::ListKeywordsForDataSourcePaginator {
        crate::operation::list_keywords_for_data_source::paginator::ListKeywordsForDataSourcePaginator::new(self.handle, self.inner)
    }
    /// <p> The control mapping data source that the keywords apply to. </p>
    pub fn source(mut self, input: crate::types::SourceType) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p> The control mapping data source that the keywords apply to. </p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::SourceType>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p> The control mapping data source that the keywords apply to. </p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::SourceType> {
        self.inner.get_source()
    }
    /// <p> The pagination token that's used to fetch the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> The pagination token that's used to fetch the next set of results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> The pagination token that's used to fetch the next set of results. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p> Represents the maximum number of results on a page or for an API request call. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> Represents the maximum number of results on a page or for an API request call. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> Represents the maximum number of results on a page or for an API request call. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
