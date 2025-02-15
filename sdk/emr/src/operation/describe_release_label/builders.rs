// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_release_label::_describe_release_label_output::DescribeReleaseLabelOutputBuilder;

pub use crate::operation::describe_release_label::_describe_release_label_input::DescribeReleaseLabelInputBuilder;

impl DescribeReleaseLabelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_release_label::DescribeReleaseLabelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_release_label::DescribeReleaseLabelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_release_label();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeReleaseLabel`.
///
/// <p>Provides Amazon EMR release label details, such as the releases available the Region where the API request is run, and the available applications for a specific Amazon EMR release label. Can also list Amazon EMR releases that support a specified version of Spark.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeReleaseLabelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_release_label::builders::DescribeReleaseLabelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_release_label::DescribeReleaseLabelOutput,
        crate::operation::describe_release_label::DescribeReleaseLabelError,
    > for DescribeReleaseLabelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_release_label::DescribeReleaseLabelOutput,
            crate::operation::describe_release_label::DescribeReleaseLabelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeReleaseLabelFluentBuilder {
    /// Creates a new `DescribeReleaseLabel`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeReleaseLabel as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_release_label::builders::DescribeReleaseLabelInputBuilder {
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
        crate::operation::describe_release_label::DescribeReleaseLabelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_release_label::DescribeReleaseLabelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_release_label::DescribeReleaseLabel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_release_label::DescribeReleaseLabel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_release_label::DescribeReleaseLabelOutput,
        crate::operation::describe_release_label::DescribeReleaseLabelError,
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
    /// <p>The target release label to be described.</p>
    pub fn release_label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.release_label(input.into());
        self
    }
    /// <p>The target release label to be described.</p>
    pub fn set_release_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_release_label(input);
        self
    }
    /// <p>The target release label to be described.</p>
    pub fn get_release_label(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_release_label()
    }
    /// <p>The pagination token. Reserved for future use. Currently set to null.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token. Reserved for future use. Currently set to null.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token. Reserved for future use. Currently set to null.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Reserved for future use. Currently set to null.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Reserved for future use. Currently set to null.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Reserved for future use. Currently set to null.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
