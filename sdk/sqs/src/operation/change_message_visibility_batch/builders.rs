// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::change_message_visibility_batch::_change_message_visibility_batch_output::ChangeMessageVisibilityBatchOutputBuilder;

pub use crate::operation::change_message_visibility_batch::_change_message_visibility_batch_input::ChangeMessageVisibilityBatchInputBuilder;

impl ChangeMessageVisibilityBatchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.change_message_visibility_batch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ChangeMessageVisibilityBatch`.
///
/// <p>Changes the visibility timeout of multiple messages. This is a batch version of <code> <code>ChangeMessageVisibility</code>.</code> The result of the action on each message is reported individually in the response. You can send up to 10 <code> <code>ChangeMessageVisibility</code> </code> requests with each <code>ChangeMessageVisibilityBatch</code> action.</p> <important>
/// <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ChangeMessageVisibilityBatchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
    > for ChangeMessageVisibilityBatchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
            crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ChangeMessageVisibilityBatchFluentBuilder {
    /// Creates a new `ChangeMessageVisibilityBatch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ChangeMessageVisibilityBatch as a reference.
    pub fn as_input(&self) -> &crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchInputBuilder {
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
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput,
        crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError,
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
    /// <p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
    /// Appends an item to `Entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p>
    pub fn entries(mut self, input: crate::types::ChangeMessageVisibilityBatchRequestEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p>
    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ChangeMessageVisibilityBatchRequestEntry>>) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p>
    pub fn get_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ChangeMessageVisibilityBatchRequestEntry>> {
        self.inner.get_entries()
    }
}
