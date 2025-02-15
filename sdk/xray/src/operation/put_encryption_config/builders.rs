// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_encryption_config::_put_encryption_config_output::PutEncryptionConfigOutputBuilder;

pub use crate::operation::put_encryption_config::_put_encryption_config_input::PutEncryptionConfigInputBuilder;

impl PutEncryptionConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_encryption_config::PutEncryptionConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_encryption_config::PutEncryptionConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_encryption_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutEncryptionConfig`.
///
/// <p>Updates the encryption configuration for X-Ray data.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutEncryptionConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_encryption_config::builders::PutEncryptionConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_encryption_config::PutEncryptionConfigOutput,
        crate::operation::put_encryption_config::PutEncryptionConfigError,
    > for PutEncryptionConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_encryption_config::PutEncryptionConfigOutput,
            crate::operation::put_encryption_config::PutEncryptionConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutEncryptionConfigFluentBuilder {
    /// Creates a new `PutEncryptionConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutEncryptionConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::put_encryption_config::builders::PutEncryptionConfigInputBuilder {
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
        crate::operation::put_encryption_config::PutEncryptionConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_encryption_config::PutEncryptionConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_encryption_config::PutEncryptionConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_encryption_config::PutEncryptionConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_encryption_config::PutEncryptionConfigOutput,
        crate::operation::put_encryption_config::PutEncryptionConfigError,
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
    /// <p>An Amazon Web Services KMS key in one of the following formats:</p>
    /// <ul>
    /// <li> <p> <b>Alias</b> - The name of the key. For example, <code>alias/MyKey</code>.</p> </li>
    /// <li> <p> <b>Key ID</b> - The KMS key ID of the key. For example, <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Amazon Web Services X-Ray does not support asymmetric KMS keys.</p> </li>
    /// <li> <p> <b>ARN</b> - The full Amazon Resource Name of the key ID or alias. For example, <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Use this format to specify a key in a different account.</p> </li>
    /// </ul>
    /// <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>An Amazon Web Services KMS key in one of the following formats:</p>
    /// <ul>
    /// <li> <p> <b>Alias</b> - The name of the key. For example, <code>alias/MyKey</code>.</p> </li>
    /// <li> <p> <b>Key ID</b> - The KMS key ID of the key. For example, <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Amazon Web Services X-Ray does not support asymmetric KMS keys.</p> </li>
    /// <li> <p> <b>ARN</b> - The full Amazon Resource Name of the key ID or alias. For example, <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Use this format to specify a key in a different account.</p> </li>
    /// </ul>
    /// <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>An Amazon Web Services KMS key in one of the following formats:</p>
    /// <ul>
    /// <li> <p> <b>Alias</b> - The name of the key. For example, <code>alias/MyKey</code>.</p> </li>
    /// <li> <p> <b>Key ID</b> - The KMS key ID of the key. For example, <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Amazon Web Services X-Ray does not support asymmetric KMS keys.</p> </li>
    /// <li> <p> <b>ARN</b> - The full Amazon Resource Name of the key ID or alias. For example, <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Use this format to specify a key in a different account.</p> </li>
    /// </ul>
    /// <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
    /// <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set to <code>NONE</code> for default encryption.</p>
    pub fn r#type(mut self, input: crate::types::EncryptionType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set to <code>NONE</code> for default encryption.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::EncryptionType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set to <code>NONE</code> for default encryption.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::EncryptionType> {
        self.inner.get_type()
    }
}
