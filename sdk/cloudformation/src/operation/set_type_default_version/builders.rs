// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_type_default_version::_set_type_default_version_output::SetTypeDefaultVersionOutputBuilder;

pub use crate::operation::set_type_default_version::_set_type_default_version_input::SetTypeDefaultVersionInputBuilder;

impl SetTypeDefaultVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_type_default_version::SetTypeDefaultVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_type_default_version::SetTypeDefaultVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_type_default_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetTypeDefaultVersion`.
///
/// <p>Specify the default version of an extension. The default version of an extension will be used in CloudFormation operations.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetTypeDefaultVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_type_default_version::builders::SetTypeDefaultVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_type_default_version::SetTypeDefaultVersionOutput,
        crate::operation::set_type_default_version::SetTypeDefaultVersionError,
    > for SetTypeDefaultVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_type_default_version::SetTypeDefaultVersionOutput,
            crate::operation::set_type_default_version::SetTypeDefaultVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetTypeDefaultVersionFluentBuilder {
    /// Creates a new `SetTypeDefaultVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetTypeDefaultVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::set_type_default_version::builders::SetTypeDefaultVersionInputBuilder {
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
        crate::operation::set_type_default_version::SetTypeDefaultVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_type_default_version::SetTypeDefaultVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_type_default_version::SetTypeDefaultVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_type_default_version::SetTypeDefaultVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_type_default_version::SetTypeDefaultVersionOutput,
        crate::operation::set_type_default_version::SetTypeDefaultVersionError,
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
    /// <p>The Amazon Resource Name (ARN) of the extension for which you want version summary information.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the extension for which you want version summary information.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the extension for which you want version summary information.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>The kind of extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn r#type(mut self, input: crate::types::RegistryType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The kind of extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::RegistryType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The kind of extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::RegistryType> {
        self.inner.get_type()
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.type_name(input.into());
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn set_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type_name(input);
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify either <code>TypeName</code> and <code>Type</code>, or <code>Arn</code>.</p>
    pub fn get_type_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_type_name()
    }
    /// <p>The ID of a specific version of the extension. The version ID is the value at the end of the Amazon Resource Name (ARN) assigned to the extension version when it is registered.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>The ID of a specific version of the extension. The version ID is the value at the end of the Amazon Resource Name (ARN) assigned to the extension version when it is registered.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    /// <p>The ID of a specific version of the extension. The version ID is the value at the end of the Amazon Resource Name (ARN) assigned to the extension version when it is registered.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id()
    }
}
