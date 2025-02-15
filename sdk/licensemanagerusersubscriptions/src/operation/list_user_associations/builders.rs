// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_user_associations::_list_user_associations_output::ListUserAssociationsOutputBuilder;

pub use crate::operation::list_user_associations::_list_user_associations_input::ListUserAssociationsInputBuilder;

impl ListUserAssociationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_user_associations::ListUserAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_user_associations::ListUserAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_user_associations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListUserAssociations`.
///
/// <p>Lists user associations for an identity provider.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListUserAssociationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_user_associations::builders::ListUserAssociationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_user_associations::ListUserAssociationsOutput,
        crate::operation::list_user_associations::ListUserAssociationsError,
    > for ListUserAssociationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_user_associations::ListUserAssociationsOutput,
            crate::operation::list_user_associations::ListUserAssociationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListUserAssociationsFluentBuilder {
    /// Creates a new `ListUserAssociations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListUserAssociations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_user_associations::builders::ListUserAssociationsInputBuilder {
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
        crate::operation::list_user_associations::ListUserAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_user_associations::ListUserAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_user_associations::ListUserAssociations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_user_associations::ListUserAssociations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_user_associations::ListUserAssociationsOutput,
        crate::operation::list_user_associations::ListUserAssociationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_user_associations::paginator::ListUserAssociationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_user_associations::paginator::ListUserAssociationsPaginator {
        crate::operation::list_user_associations::paginator::ListUserAssociationsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the EC2 instance, which provides user-based subscriptions.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn identity_provider(mut self, input: crate::types::IdentityProvider) -> Self {
        self.inner = self.inner.identity_provider(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn set_identity_provider(mut self, input: ::std::option::Option<crate::types::IdentityProvider>) -> Self {
        self.inner = self.inner.set_identity_provider(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn get_identity_provider(&self) -> &::std::option::Option<crate::types::IdentityProvider> {
        self.inner.get_identity_provider()
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Maximum number of results to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of structures that you can use to filter the results to those that match one or more sets of key-value pairs that you specify.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of structures that you can use to filter the results to those that match one or more sets of key-value pairs that you specify.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>An array of structures that you can use to filter the results to those that match one or more sets of key-value pairs that you specify.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
