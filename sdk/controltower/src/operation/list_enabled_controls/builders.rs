// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_enabled_controls::_list_enabled_controls_output::ListEnabledControlsOutputBuilder;

pub use crate::operation::list_enabled_controls::_list_enabled_controls_input::ListEnabledControlsInputBuilder;

impl ListEnabledControlsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_enabled_controls::ListEnabledControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_enabled_controls::ListEnabledControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_enabled_controls();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListEnabledControls`.
///
/// <p>Lists the controls enabled by AWS Control Tower on the specified organizational unit and the accounts it contains. For usage examples, see <a href="https://docs.aws.amazon.com/controltower/latest/userguide/control-api-examples-short.html"> <i>the AWS Control Tower User Guide</i> </a> </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListEnabledControlsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_enabled_controls::builders::ListEnabledControlsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_enabled_controls::ListEnabledControlsOutput,
        crate::operation::list_enabled_controls::ListEnabledControlsError,
    > for ListEnabledControlsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_enabled_controls::ListEnabledControlsOutput,
            crate::operation::list_enabled_controls::ListEnabledControlsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListEnabledControlsFluentBuilder {
    /// Creates a new `ListEnabledControls`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListEnabledControls as a reference.
    pub fn as_input(&self) -> &crate::operation::list_enabled_controls::builders::ListEnabledControlsInputBuilder {
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
        crate::operation::list_enabled_controls::ListEnabledControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_enabled_controls::ListEnabledControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_enabled_controls::ListEnabledControls::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_enabled_controls::ListEnabledControls::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_enabled_controls::ListEnabledControlsOutput,
        crate::operation::list_enabled_controls::ListEnabledControlsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_enabled_controls::paginator::ListEnabledControlsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_enabled_controls::paginator::ListEnabledControlsPaginator {
        crate::operation::list_enabled_controls::paginator::ListEnabledControlsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ARN of the organizational unit. For information on how to find the <code>targetIdentifier</code>, see <a href="https://docs.aws.amazon.com/controltower/latest/APIReference/Welcome.html">the overview page</a>.</p>
    pub fn target_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_identifier(input.into());
        self
    }
    /// <p>The ARN of the organizational unit. For information on how to find the <code>targetIdentifier</code>, see <a href="https://docs.aws.amazon.com/controltower/latest/APIReference/Welcome.html">the overview page</a>.</p>
    pub fn set_target_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_identifier(input);
        self
    }
    /// <p>The ARN of the organizational unit. For information on how to find the <code>targetIdentifier</code>, see <a href="https://docs.aws.amazon.com/controltower/latest/APIReference/Welcome.html">the overview page</a>.</p>
    pub fn get_target_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_identifier()
    }
    /// <p>The token to continue the list from a previous API call with the same parameters.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to continue the list from a previous API call with the same parameters.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to continue the list from a previous API call with the same parameters.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>How many results to return per API call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>How many results to return per API call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>How many results to return per API call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
