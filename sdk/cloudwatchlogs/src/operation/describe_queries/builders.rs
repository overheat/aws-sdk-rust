// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_queries::_describe_queries_output::DescribeQueriesOutputBuilder;

pub use crate::operation::describe_queries::_describe_queries_input::DescribeQueriesInputBuilder;

impl DescribeQueriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_queries::DescribeQueriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_queries::DescribeQueriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_queries();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeQueries`.
///
/// <p>Returns a list of CloudWatch Logs Insights queries that are scheduled, running, or have been run recently in this account. You can request all queries or limit it to queries of a specific log group or queries with a certain status.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeQueriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_queries::builders::DescribeQueriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_queries::DescribeQueriesOutput,
        crate::operation::describe_queries::DescribeQueriesError,
    > for DescribeQueriesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_queries::DescribeQueriesOutput,
            crate::operation::describe_queries::DescribeQueriesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeQueriesFluentBuilder {
    /// Creates a new `DescribeQueries`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeQueries as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_queries::builders::DescribeQueriesInputBuilder {
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
        crate::operation::describe_queries::DescribeQueriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_queries::DescribeQueriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_queries::DescribeQueries::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_queries::DescribeQueries::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_queries::DescribeQueriesOutput,
        crate::operation::describe_queries::DescribeQueriesError,
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
    /// <p>Limits the returned queries to only those for the specified log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>Limits the returned queries to only those for the specified log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>Limits the returned queries to only those for the specified log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_name()
    }
    /// <p>Limits the returned queries to only those that have the specified status. Valid values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, and <code>Scheduled</code>.</p>
    pub fn status(mut self, input: crate::types::QueryStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>Limits the returned queries to only those that have the specified status. Valid values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, and <code>Scheduled</code>.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::QueryStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Limits the returned queries to only those that have the specified status. Valid values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, and <code>Scheduled</code>.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::QueryStatus> {
        self.inner.get_status()
    }
    /// <p>Limits the number of returned queries to the specified number.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Limits the number of returned queries to the specified number.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Limits the number of returned queries to the specified number.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
