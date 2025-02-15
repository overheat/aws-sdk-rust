// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_conformance_pack_status::_describe_conformance_pack_status_output::DescribeConformancePackStatusOutputBuilder;

pub use crate::operation::describe_conformance_pack_status::_describe_conformance_pack_status_input::DescribeConformancePackStatusInputBuilder;

impl DescribeConformancePackStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_conformance_pack_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeConformancePackStatus`.
///
/// <p>Provides one or more conformance packs deployment status.</p> <note>
/// <p>If there are no conformance packs then you will see an empty result.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeConformancePackStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
    > for DescribeConformancePackStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeConformancePackStatusFluentBuilder {
    /// Creates a new `DescribeConformancePackStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeConformancePackStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder {
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
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_conformance_pack_status::DescribeConformancePackStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator {
        crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ConformancePackNames`.
    ///
    /// To override the contents of this collection use [`set_conformance_pack_names`](Self::set_conformance_pack_names).
    ///
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn conformance_pack_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.conformance_pack_names(input.into());
        self
    }
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn set_conformance_pack_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_conformance_pack_names(input);
        self
    }
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn get_conformance_pack_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_conformance_pack_names()
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
