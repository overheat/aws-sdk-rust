// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_language_models::_list_language_models_output::ListLanguageModelsOutputBuilder;

pub use crate::operation::list_language_models::_list_language_models_input::ListLanguageModelsInputBuilder;

impl ListLanguageModelsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_language_models::ListLanguageModelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_language_models::ListLanguageModelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_language_models();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListLanguageModels`.
///
/// <p>Provides a list of custom language models that match the specified criteria. If no criteria are specified, all custom language models are returned.</p>
/// <p>To get detailed information about a specific custom language model, use the operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListLanguageModelsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_language_models::builders::ListLanguageModelsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_language_models::ListLanguageModelsOutput,
        crate::operation::list_language_models::ListLanguageModelsError,
    > for ListLanguageModelsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_language_models::ListLanguageModelsOutput,
            crate::operation::list_language_models::ListLanguageModelsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListLanguageModelsFluentBuilder {
    /// Creates a new `ListLanguageModels`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListLanguageModels as a reference.
    pub fn as_input(&self) -> &crate::operation::list_language_models::builders::ListLanguageModelsInputBuilder {
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
        crate::operation::list_language_models::ListLanguageModelsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_language_models::ListLanguageModelsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_language_models::ListLanguageModels::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_language_models::ListLanguageModels::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_language_models::ListLanguageModelsOutput,
        crate::operation::list_language_models::ListLanguageModelsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_language_models::paginator::ListLanguageModelsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_language_models::paginator::ListLanguageModelsPaginator {
        crate::operation::list_language_models::paginator::ListLanguageModelsPaginator::new(self.handle, self.inner)
    }
    /// <p>Returns only custom language models with the specified status. Language models are ordered by creation date, with the newest model first. If you don't include <code>StatusEquals</code>, all custom language models are returned.</p>
    pub fn status_equals(mut self, input: crate::types::ModelStatus) -> Self {
        self.inner = self.inner.status_equals(input);
        self
    }
    /// <p>Returns only custom language models with the specified status. Language models are ordered by creation date, with the newest model first. If you don't include <code>StatusEquals</code>, all custom language models are returned.</p>
    pub fn set_status_equals(mut self, input: ::std::option::Option<crate::types::ModelStatus>) -> Self {
        self.inner = self.inner.set_status_equals(input);
        self
    }
    /// <p>Returns only custom language models with the specified status. Language models are ordered by creation date, with the newest model first. If you don't include <code>StatusEquals</code>, all custom language models are returned.</p>
    pub fn get_status_equals(&self) -> &::std::option::Option<crate::types::ModelStatus> {
        self.inner.get_status_equals()
    }
    /// <p>Returns only the custom language models that contain the specified string. The search is not case sensitive.</p>
    pub fn name_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>Returns only the custom language models that contain the specified string. The search is not case sensitive.</p>
    pub fn set_name_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>Returns only the custom language models that contain the specified string. The search is not case sensitive.</p>
    pub fn get_name_contains(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_contains()
    }
    /// <p>If your <code>ListLanguageModels</code> request returns more results than can be displayed, <code>NextToken</code> is displayed in the response with an associated string. To get the next page of results, copy this string and repeat your request, including <code>NextToken</code> with the value of the copied string. Repeat as needed to view all your results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If your <code>ListLanguageModels</code> request returns more results than can be displayed, <code>NextToken</code> is displayed in the response with an associated string. To get the next page of results, copy this string and repeat your request, including <code>NextToken</code> with the value of the copied string. Repeat as needed to view all your results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If your <code>ListLanguageModels</code> request returns more results than can be displayed, <code>NextToken</code> is displayed in the response with an associated string. To get the next page of results, copy this string and repeat your request, including <code>NextToken</code> with the value of the copied string. Repeat as needed to view all your results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of custom language models to return in each page of results. If there are fewer results than the value that you specify, only the actual results are returned. If you don't specify a value, a default of 5 is used.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of custom language models to return in each page of results. If there are fewer results than the value that you specify, only the actual results are returned. If you don't specify a value, a default of 5 is used.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of custom language models to return in each page of results. If there are fewer results than the value that you specify, only the actual results are returned. If you don't specify a value, a default of 5 is used.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
