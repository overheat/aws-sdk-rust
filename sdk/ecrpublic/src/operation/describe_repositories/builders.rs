// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_repositories::_describe_repositories_output::DescribeRepositoriesOutputBuilder;

pub use crate::operation::describe_repositories::_describe_repositories_input::DescribeRepositoriesInputBuilder;

impl DescribeRepositoriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_repositories::DescribeRepositoriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_repositories::DescribeRepositoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_repositories();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeRepositories`.
///
/// <p>Describes repositories that are in a public registry.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeRepositoriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_repositories::builders::DescribeRepositoriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_repositories::DescribeRepositoriesOutput,
        crate::operation::describe_repositories::DescribeRepositoriesError,
    > for DescribeRepositoriesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_repositories::DescribeRepositoriesOutput,
            crate::operation::describe_repositories::DescribeRepositoriesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeRepositoriesFluentBuilder {
    /// Creates a new `DescribeRepositories`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeRepositories as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_repositories::builders::DescribeRepositoriesInputBuilder {
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
        crate::operation::describe_repositories::DescribeRepositoriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_repositories::DescribeRepositoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_repositories::DescribeRepositories::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_repositories::DescribeRepositories::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_repositories::DescribeRepositoriesOutput,
        crate::operation::describe_repositories::DescribeRepositoriesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_repositories::paginator::DescribeRepositoriesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_repositories::paginator::DescribeRepositoriesPaginator {
        crate::operation::describe_repositories::paginator::DescribeRepositoriesPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Web Services account ID that's associated with the registry that contains the repositories to be described. If you do not specify a registry, the default public registry is assumed.</p>
    pub fn registry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.registry_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that's associated with the registry that contains the repositories to be described. If you do not specify a registry, the default public registry is assumed.</p>
    pub fn set_registry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_registry_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID that's associated with the registry that contains the repositories to be described. If you do not specify a registry, the default public registry is assumed.</p>
    pub fn get_registry_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_registry_id()
    }
    /// Appends an item to `repositoryNames`.
    ///
    /// To override the contents of this collection use [`set_repository_names`](Self::set_repository_names).
    ///
    /// <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    pub fn repository_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_names(input.into());
        self
    }
    /// <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    pub fn set_repository_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_repository_names(input);
        self
    }
    /// <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    pub fn get_repository_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_repository_names()
    }
    /// <p>The <code>nextToken</code> value that's returned from a previous paginated <code>DescribeRepositories</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. If there are no more results to return, this value is <code>null</code>. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value that's returned from a previous paginated <code>DescribeRepositories</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. If there are no more results to return, this value is <code>null</code>. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> value that's returned from a previous paginated <code>DescribeRepositories</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. If there are no more results to return, this value is <code>null</code>. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of repository results that's returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter isn't used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of repository results that's returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter isn't used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of repository results that's returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter isn't used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. If you specify repositories with <code>repositoryNames</code>, you can't use this option.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
