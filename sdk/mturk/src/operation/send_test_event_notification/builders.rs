// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_test_event_notification::_send_test_event_notification_output::SendTestEventNotificationOutputBuilder;

pub use crate::operation::send_test_event_notification::_send_test_event_notification_input::SendTestEventNotificationInputBuilder;

impl SendTestEventNotificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_test_event_notification::SendTestEventNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_test_event_notification::SendTestEventNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_test_event_notification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendTestEventNotification`.
///
/// <p> The <code>SendTestEventNotification</code> operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notifications without setting up notifications for a real HIT type and trying to trigger them using the website. When you call this operation, the service attempts to send the test notification immediately. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendTestEventNotificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_test_event_notification::builders::SendTestEventNotificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_test_event_notification::SendTestEventNotificationOutput,
        crate::operation::send_test_event_notification::SendTestEventNotificationError,
    > for SendTestEventNotificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_test_event_notification::SendTestEventNotificationOutput,
            crate::operation::send_test_event_notification::SendTestEventNotificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendTestEventNotificationFluentBuilder {
    /// Creates a new `SendTestEventNotification`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendTestEventNotification as a reference.
    pub fn as_input(&self) -> &crate::operation::send_test_event_notification::builders::SendTestEventNotificationInputBuilder {
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
        crate::operation::send_test_event_notification::SendTestEventNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_test_event_notification::SendTestEventNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_test_event_notification::SendTestEventNotification::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_test_event_notification::SendTestEventNotification::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_test_event_notification::SendTestEventNotificationOutput,
        crate::operation::send_test_event_notification::SendTestEventNotificationError,
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
    /// <p> The notification specification to test. This value is identical to the value you would provide to the UpdateNotificationSettings operation when you establish the notification specification for a HIT type. </p>
    pub fn notification(mut self, input: crate::types::NotificationSpecification) -> Self {
        self.inner = self.inner.notification(input);
        self
    }
    /// <p> The notification specification to test. This value is identical to the value you would provide to the UpdateNotificationSettings operation when you establish the notification specification for a HIT type. </p>
    pub fn set_notification(mut self, input: ::std::option::Option<crate::types::NotificationSpecification>) -> Self {
        self.inner = self.inner.set_notification(input);
        self
    }
    /// <p> The notification specification to test. This value is identical to the value you would provide to the UpdateNotificationSettings operation when you establish the notification specification for a HIT type. </p>
    pub fn get_notification(&self) -> &::std::option::Option<crate::types::NotificationSpecification> {
        self.inner.get_notification()
    }
    /// <p> The event to simulate to test the notification specification. This event is included in the test message even if the notification specification does not include the event type. The notification specification does not filter out the test event. </p>
    pub fn test_event_type(mut self, input: crate::types::EventType) -> Self {
        self.inner = self.inner.test_event_type(input);
        self
    }
    /// <p> The event to simulate to test the notification specification. This event is included in the test message even if the notification specification does not include the event type. The notification specification does not filter out the test event. </p>
    pub fn set_test_event_type(mut self, input: ::std::option::Option<crate::types::EventType>) -> Self {
        self.inner = self.inner.set_test_event_type(input);
        self
    }
    /// <p> The event to simulate to test the notification specification. This event is included in the test message even if the notification specification does not include the event type. The notification specification does not filter out the test event. </p>
    pub fn get_test_event_type(&self) -> &::std::option::Option<crate::types::EventType> {
        self.inner.get_test_event_type()
    }
}
