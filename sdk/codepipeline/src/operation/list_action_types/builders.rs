// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_action_types::_list_action_types_output::ListActionTypesOutputBuilder;

pub use crate::operation::list_action_types::_list_action_types_input::ListActionTypesInputBuilder;

impl ListActionTypesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_action_types::ListActionTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_action_types::ListActionTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_action_types();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListActionTypes`.
///
/// <p>Gets a summary of all CodePipeline action types associated with your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListActionTypesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_action_types::builders::ListActionTypesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_action_types::ListActionTypesOutput,
        crate::operation::list_action_types::ListActionTypesError,
    > for ListActionTypesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_action_types::ListActionTypesOutput,
            crate::operation::list_action_types::ListActionTypesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListActionTypesFluentBuilder {
    /// Creates a new `ListActionTypes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListActionTypes as a reference.
    pub fn as_input(&self) -> &crate::operation::list_action_types::builders::ListActionTypesInputBuilder {
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
        crate::operation::list_action_types::ListActionTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_action_types::ListActionTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_action_types::ListActionTypes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_action_types::ListActionTypes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_action_types::ListActionTypesOutput,
        crate::operation::list_action_types::ListActionTypesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_action_types::paginator::ListActionTypesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_action_types::paginator::ListActionTypesPaginator {
        crate::operation::list_action_types::paginator::ListActionTypesPaginator::new(self.handle, self.inner)
    }
    /// <p>Filters the list of action types to those created by a specified entity.</p>
    pub fn action_owner_filter(mut self, input: crate::types::ActionOwner) -> Self {
        self.inner = self.inner.action_owner_filter(input);
        self
    }
    /// <p>Filters the list of action types to those created by a specified entity.</p>
    pub fn set_action_owner_filter(mut self, input: ::std::option::Option<crate::types::ActionOwner>) -> Self {
        self.inner = self.inner.set_action_owner_filter(input);
        self
    }
    /// <p>Filters the list of action types to those created by a specified entity.</p>
    pub fn get_action_owner_filter(&self) -> &::std::option::Option<crate::types::ActionOwner> {
        self.inner.get_action_owner_filter()
    }
    /// <p>An identifier that was returned from the previous list action types call, which can be used to return the next set of action types in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous list action types call, which can be used to return the next set of action types in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An identifier that was returned from the previous list action types call, which can be used to return the next set of action types in the list.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The Region to filter on for the list of action types.</p>
    pub fn region_filter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.region_filter(input.into());
        self
    }
    /// <p>The Region to filter on for the list of action types.</p>
    pub fn set_region_filter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_region_filter(input);
        self
    }
    /// <p>The Region to filter on for the list of action types.</p>
    pub fn get_region_filter(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_region_filter()
    }
}
