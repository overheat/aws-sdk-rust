// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::check_schema_version_validity::_check_schema_version_validity_output::CheckSchemaVersionValidityOutputBuilder;

pub use crate::operation::check_schema_version_validity::_check_schema_version_validity_input::CheckSchemaVersionValidityInputBuilder;

impl CheckSchemaVersionValidityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_schema_version_validity::CheckSchemaVersionValidityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.check_schema_version_validity();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CheckSchemaVersionValidity`.
///
/// <p>Validates the supplied schema. This call has no side effects, it simply validates using the supplied schema using <code>DataFormat</code> as the format. Since it does not take a schema set name, no compatibility checks are performed.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CheckSchemaVersionValidityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::check_schema_version_validity::builders::CheckSchemaVersionValidityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityOutput,
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityError,
    > for CheckSchemaVersionValidityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::check_schema_version_validity::CheckSchemaVersionValidityOutput,
            crate::operation::check_schema_version_validity::CheckSchemaVersionValidityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CheckSchemaVersionValidityFluentBuilder {
    /// Creates a new `CheckSchemaVersionValidity`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CheckSchemaVersionValidity as a reference.
    pub fn as_input(&self) -> &crate::operation::check_schema_version_validity::builders::CheckSchemaVersionValidityInputBuilder {
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
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_schema_version_validity::CheckSchemaVersionValidityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::check_schema_version_validity::CheckSchemaVersionValidity::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidity::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityOutput,
        crate::operation::check_schema_version_validity::CheckSchemaVersionValidityError,
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
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn data_format(mut self, input: crate::types::DataFormat) -> Self {
        self.inner = self.inner.data_format(input);
        self
    }
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn set_data_format(mut self, input: ::std::option::Option<crate::types::DataFormat>) -> Self {
        self.inner = self.inner.set_data_format(input);
        self
    }
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn get_data_format(&self) -> &::std::option::Option<crate::types::DataFormat> {
        self.inner.get_data_format()
    }
    /// <p>The definition of the schema that has to be validated.</p>
    pub fn schema_definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_definition(input.into());
        self
    }
    /// <p>The definition of the schema that has to be validated.</p>
    pub fn set_schema_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_definition(input);
        self
    }
    /// <p>The definition of the schema that has to be validated.</p>
    pub fn get_schema_definition(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_definition()
    }
}
