// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ota_updates::_list_ota_updates_output::ListOtaUpdatesOutputBuilder;

pub use crate::operation::list_ota_updates::_list_ota_updates_input::ListOtaUpdatesInputBuilder;

impl ListOtaUpdatesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ota_updates::ListOtaUpdatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ota_updates::ListOTAUpdatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ota_updates();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListOTAUpdates`.
///
/// <p>Lists OTA updates.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListOTAUpdates</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListOTAUpdatesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ota_updates::builders::ListOtaUpdatesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ota_updates::ListOtaUpdatesOutput,
        crate::operation::list_ota_updates::ListOTAUpdatesError,
    > for ListOTAUpdatesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ota_updates::ListOtaUpdatesOutput,
            crate::operation::list_ota_updates::ListOTAUpdatesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListOTAUpdatesFluentBuilder {
    /// Creates a new `ListOTAUpdates`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListOTAUpdates as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ota_updates::builders::ListOtaUpdatesInputBuilder {
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
        crate::operation::list_ota_updates::ListOtaUpdatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ota_updates::ListOTAUpdatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ota_updates::ListOTAUpdates::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ota_updates::ListOTAUpdates::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_ota_updates::ListOtaUpdatesOutput,
        crate::operation::list_ota_updates::ListOTAUpdatesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_ota_updates::paginator::ListOtaUpdatesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_ota_updates::paginator::ListOtaUpdatesPaginator {
        crate::operation::list_ota_updates::paginator::ListOtaUpdatesPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token used to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token used to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token used to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The OTA update job status.</p>
    pub fn ota_update_status(mut self, input: crate::types::OtaUpdateStatus) -> Self {
        self.inner = self.inner.ota_update_status(input);
        self
    }
    /// <p>The OTA update job status.</p>
    pub fn set_ota_update_status(mut self, input: ::std::option::Option<crate::types::OtaUpdateStatus>) -> Self {
        self.inner = self.inner.set_ota_update_status(input);
        self
    }
    /// <p>The OTA update job status.</p>
    pub fn get_ota_update_status(&self) -> &::std::option::Option<crate::types::OtaUpdateStatus> {
        self.inner.get_ota_update_status()
    }
}
