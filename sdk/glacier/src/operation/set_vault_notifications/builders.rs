// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_vault_notifications::_set_vault_notifications_output::SetVaultNotificationsOutputBuilder;

pub use crate::operation::set_vault_notifications::_set_vault_notifications_input::SetVaultNotificationsInputBuilder;

impl SetVaultNotificationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_vault_notifications::SetVaultNotificationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_vault_notifications::SetVaultNotificationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_vault_notifications();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetVaultNotifications`.
///
/// <p>This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications.</p>
/// <p>To configure vault notifications, send a PUT request to the <code>notification-configuration</code> subresource of the vault. The request should include a JSON document that provides an Amazon SNS topic and specific events for which you want Amazon S3 Glacier to send notifications to the topic.</p>
/// <p>Amazon SNS topics must grant permission to the vault to be allowed to publish notifications to the topic. You can configure a vault to publish a notification for the following vault events:</p>
/// <ul>
/// <li> <p> <b>ArchiveRetrievalCompleted</b> This event occurs when a job that was initiated for an archive retrieval is completed (<code>InitiateJob</code>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <code>DescribeJob</code>. </p> </li>
/// <li> <p> <b>InventoryRetrievalCompleted</b> This event occurs when a job that was initiated for an inventory retrieval is completed (<code>InitiateJob</code>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <code>DescribeJob</code>. </p> </li>
/// </ul>
/// <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p>
/// <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-put.html">Set Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetVaultNotificationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_vault_notifications::builders::SetVaultNotificationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_vault_notifications::SetVaultNotificationsOutput,
        crate::operation::set_vault_notifications::SetVaultNotificationsError,
    > for SetVaultNotificationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_vault_notifications::SetVaultNotificationsOutput,
            crate::operation::set_vault_notifications::SetVaultNotificationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetVaultNotificationsFluentBuilder {
    /// Creates a new `SetVaultNotifications`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetVaultNotifications as a reference.
    pub fn as_input(&self) -> &crate::operation::set_vault_notifications::builders::SetVaultNotificationsInputBuilder {
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
        crate::operation::set_vault_notifications::SetVaultNotificationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_vault_notifications::SetVaultNotificationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_vault_notifications::SetVaultNotifications::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_vault_notifications::SetVaultNotifications::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_vault_notifications::SetVaultNotificationsOutput,
        crate::operation::set_vault_notifications::SetVaultNotificationsError,
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
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vault_name(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vault_name(input);
        self
    }
    /// <p>The name of the vault.</p>
    pub fn get_vault_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vault_name()
    }
    /// <p>Provides options for specifying notification configuration.</p>
    pub fn vault_notification_config(mut self, input: crate::types::VaultNotificationConfig) -> Self {
        self.inner = self.inner.vault_notification_config(input);
        self
    }
    /// <p>Provides options for specifying notification configuration.</p>
    pub fn set_vault_notification_config(mut self, input: ::std::option::Option<crate::types::VaultNotificationConfig>) -> Self {
        self.inner = self.inner.set_vault_notification_config(input);
        self
    }
    /// <p>Provides options for specifying notification configuration.</p>
    pub fn get_vault_notification_config(&self) -> &::std::option::Option<crate::types::VaultNotificationConfig> {
        self.inner.get_vault_notification_config()
    }
}
