// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_test_cases::_describe_test_cases_output::DescribeTestCasesOutputBuilder;

pub use crate::operation::describe_test_cases::_describe_test_cases_input::DescribeTestCasesInputBuilder;

impl DescribeTestCasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_test_cases::DescribeTestCasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_test_cases::DescribeTestCasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_test_cases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTestCases`.
///
/// <p> Returns a list of details about test cases for a report. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTestCasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_test_cases::builders::DescribeTestCasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_test_cases::DescribeTestCasesOutput,
        crate::operation::describe_test_cases::DescribeTestCasesError,
    > for DescribeTestCasesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_test_cases::DescribeTestCasesOutput,
            crate::operation::describe_test_cases::DescribeTestCasesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTestCasesFluentBuilder {
    /// Creates a new `DescribeTestCases`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTestCases as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_test_cases::builders::DescribeTestCasesInputBuilder {
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
        crate::operation::describe_test_cases::DescribeTestCasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_test_cases::DescribeTestCasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_test_cases::DescribeTestCases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_test_cases::DescribeTestCases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_test_cases::DescribeTestCasesOutput,
        crate::operation::describe_test_cases::DescribeTestCasesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_test_cases::paginator::DescribeTestCasesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_test_cases::paginator::DescribeTestCasesPaginator {
        crate::operation::describe_test_cases::paginator::DescribeTestCasesPaginator::new(self.handle, self.inner)
    }
    /// <p> The ARN of the report for which test cases are returned. </p>
    pub fn report_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.report_arn(input.into());
        self
    }
    /// <p> The ARN of the report for which test cases are returned. </p>
    pub fn set_report_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_report_arn(input);
        self
    }
    /// <p> The ARN of the report for which test cases are returned. </p>
    pub fn get_report_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_report_arn()
    }
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p> The maximum number of paginated test cases returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>TestCase</code> objects. The default value is 100. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> The maximum number of paginated test cases returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>TestCase</code> objects. The default value is 100. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> The maximum number of paginated test cases returned per response. Use <code>nextToken</code> to iterate pages in the list of returned <code>TestCase</code> objects. The default value is 100. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p> A <code>TestCaseFilter</code> object used to filter the returned reports. </p>
    pub fn filter(mut self, input: crate::types::TestCaseFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p> A <code>TestCaseFilter</code> object used to filter the returned reports. </p>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::TestCaseFilter>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p> A <code>TestCaseFilter</code> object used to filter the returned reports. </p>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::TestCaseFilter> {
        self.inner.get_filter()
    }
}
