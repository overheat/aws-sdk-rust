// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_describe_model_package::_batch_describe_model_package_output::BatchDescribeModelPackageOutputBuilder;

pub use crate::operation::batch_describe_model_package::_batch_describe_model_package_input::BatchDescribeModelPackageInputBuilder;

impl BatchDescribeModelPackageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_describe_model_package::BatchDescribeModelPackageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_describe_model_package();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDescribeModelPackage`.
///
/// <p>This action batch describes a list of versioned model packages</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDescribeModelPackageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_describe_model_package::builders::BatchDescribeModelPackageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageOutput,
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageError,
    > for BatchDescribeModelPackageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_describe_model_package::BatchDescribeModelPackageOutput,
            crate::operation::batch_describe_model_package::BatchDescribeModelPackageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchDescribeModelPackageFluentBuilder {
    /// Creates a new `BatchDescribeModelPackage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDescribeModelPackage as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_describe_model_package::builders::BatchDescribeModelPackageInputBuilder {
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
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_describe_model_package::BatchDescribeModelPackageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_describe_model_package::BatchDescribeModelPackage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_describe_model_package::BatchDescribeModelPackage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageOutput,
        crate::operation::batch_describe_model_package::BatchDescribeModelPackageError,
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
    /// Appends an item to `ModelPackageArnList`.
    ///
    /// To override the contents of this collection use [`set_model_package_arn_list`](Self::set_model_package_arn_list).
    ///
    /// <p>The list of Amazon Resource Name (ARN) of the model package groups.</p>
    pub fn model_package_arn_list(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_package_arn_list(input.into());
        self
    }
    /// <p>The list of Amazon Resource Name (ARN) of the model package groups.</p>
    pub fn set_model_package_arn_list(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_model_package_arn_list(input);
        self
    }
    /// <p>The list of Amazon Resource Name (ARN) of the model package groups.</p>
    pub fn get_model_package_arn_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_model_package_arn_list()
    }
}
