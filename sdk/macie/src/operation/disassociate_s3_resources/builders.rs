// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_s3_resources::_disassociate_s3_resources_output::DisassociateS3ResourcesOutputBuilder;

pub use crate::operation::disassociate_s3_resources::_disassociate_s3_resources_input::DisassociateS3ResourcesInputBuilder;

impl DisassociateS3ResourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_s3_resources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateS3Resources`.
///
/// <p>(Discontinued) Removes specified S3 resources from being monitored by Amazon Macie Classic. If <code>memberAccountId</code> isn't specified, the action removes specified S3 resources from Macie Classic for the current Macie Classic administrator account. If <code>memberAccountId</code> is specified, the action removes specified S3 resources from Macie Classic for the specified member account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateS3ResourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_s3_resources::builders::DisassociateS3ResourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesOutput,
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError,
    > for DisassociateS3ResourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesOutput,
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateS3ResourcesFluentBuilder {
    /// Creates a new `DisassociateS3Resources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateS3Resources as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_s3_resources::builders::DisassociateS3ResourcesInputBuilder {
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
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_s3_resources::DisassociateS3Resources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_s3_resources::DisassociateS3Resources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesOutput,
        crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError,
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
    /// <p>(Discontinued) The ID of the Amazon Macie Classic member account whose resources you want to remove from being monitored by Macie Classic.</p>
    pub fn member_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.member_account_id(input.into());
        self
    }
    /// <p>(Discontinued) The ID of the Amazon Macie Classic member account whose resources you want to remove from being monitored by Macie Classic.</p>
    pub fn set_member_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_member_account_id(input);
        self
    }
    /// <p>(Discontinued) The ID of the Amazon Macie Classic member account whose resources you want to remove from being monitored by Macie Classic.</p>
    pub fn get_member_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_member_account_id()
    }
    /// Appends an item to `associatedS3Resources`.
    ///
    /// To override the contents of this collection use [`set_associated_s3_resources`](Self::set_associated_s3_resources).
    ///
    /// <p>(Discontinued) The S3 resources (buckets or prefixes) that you want to remove from being monitored and classified by Amazon Macie Classic.</p>
    pub fn associated_s3_resources(mut self, input: crate::types::S3Resource) -> Self {
        self.inner = self.inner.associated_s3_resources(input);
        self
    }
    /// <p>(Discontinued) The S3 resources (buckets or prefixes) that you want to remove from being monitored and classified by Amazon Macie Classic.</p>
    pub fn set_associated_s3_resources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::S3Resource>>) -> Self {
        self.inner = self.inner.set_associated_s3_resources(input);
        self
    }
    /// <p>(Discontinued) The S3 resources (buckets or prefixes) that you want to remove from being monitored and classified by Amazon Macie Classic.</p>
    pub fn get_associated_s3_resources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::S3Resource>> {
        self.inner.get_associated_s3_resources()
    }
}
