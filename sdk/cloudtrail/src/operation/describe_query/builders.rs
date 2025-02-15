// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_query::_describe_query_output::DescribeQueryOutputBuilder;

pub use crate::operation::describe_query::_describe_query_input::DescribeQueryInputBuilder;

impl DescribeQueryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_query::DescribeQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_query::DescribeQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_query();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeQuery`.
///
/// <p>Returns metadata about a query, including query run time in milliseconds, number of events scanned and matched, and query status. If the query results were delivered to an S3 bucket, the response also provides the S3 URI and the delivery status.</p>
/// <p>You must specify either a <code>QueryID</code> or a <code>QueryAlias</code>. Specifying the <code>QueryAlias</code> parameter returns information about the last query run for the alias.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeQueryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_query::builders::DescribeQueryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_query::DescribeQueryOutput,
        crate::operation::describe_query::DescribeQueryError,
    > for DescribeQueryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_query::DescribeQueryOutput,
            crate::operation::describe_query::DescribeQueryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeQueryFluentBuilder {
    /// Creates a new `DescribeQuery`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeQuery as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_query::builders::DescribeQueryInputBuilder {
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
        crate::operation::describe_query::DescribeQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_query::DescribeQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_query::DescribeQuery::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_query::DescribeQuery::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_query::DescribeQueryOutput,
        crate::operation::describe_query::DescribeQueryError,
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
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn event_data_store(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_data_store(input.into());
        self
    }
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn set_event_data_store(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_data_store(input);
        self
    }
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn get_event_data_store(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_data_store()
    }
    /// <p>The query ID.</p>
    pub fn query_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_id(input.into());
        self
    }
    /// <p>The query ID.</p>
    pub fn set_query_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_id(input);
        self
    }
    /// <p>The query ID.</p>
    pub fn get_query_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_id()
    }
    /// <p> The alias that identifies a query template. </p>
    pub fn query_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_alias(input.into());
        self
    }
    /// <p> The alias that identifies a query template. </p>
    pub fn set_query_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_alias(input);
        self
    }
    /// <p> The alias that identifies a query template. </p>
    pub fn get_query_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_alias()
    }
}
