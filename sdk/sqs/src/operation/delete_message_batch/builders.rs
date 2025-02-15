// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_message_batch::_delete_message_batch_output::DeleteMessageBatchOutputBuilder;

pub use crate::operation::delete_message_batch::_delete_message_batch_input::DeleteMessageBatchInputBuilder;

impl DeleteMessageBatchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_message_batch::DeleteMessageBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_message_batch::DeleteMessageBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_message_batch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteMessageBatch`.
///
/// <p>Deletes up to ten messages from the specified queue. This is a batch version of <code> <code>DeleteMessage</code>.</code> The result of the action on each message is reported individually in the response.</p> <important>
/// <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteMessageBatchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_message_batch::builders::DeleteMessageBatchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_message_batch::DeleteMessageBatchOutput,
        crate::operation::delete_message_batch::DeleteMessageBatchError,
    > for DeleteMessageBatchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_message_batch::DeleteMessageBatchOutput,
            crate::operation::delete_message_batch::DeleteMessageBatchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteMessageBatchFluentBuilder {
    /// Creates a new `DeleteMessageBatch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteMessageBatch as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_message_batch::builders::DeleteMessageBatchInputBuilder {
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
        crate::operation::delete_message_batch::DeleteMessageBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_message_batch::DeleteMessageBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_message_batch::DeleteMessageBatch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_message_batch::DeleteMessageBatch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_message_batch::DeleteMessageBatchOutput,
        crate::operation::delete_message_batch::DeleteMessageBatchError,
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
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
    /// Appends an item to `Entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn entries(mut self, input: crate::types::DeleteMessageBatchRequestEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>>) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>Lists the receipt handles for the messages to be deleted.</p>
    pub fn get_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeleteMessageBatchRequestEntry>> {
        self.inner.get_entries()
    }
}
