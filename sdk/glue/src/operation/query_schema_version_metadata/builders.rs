// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::query_schema_version_metadata::_query_schema_version_metadata_output::QuerySchemaVersionMetadataOutputBuilder;

pub use crate::operation::query_schema_version_metadata::_query_schema_version_metadata_input::QuerySchemaVersionMetadataInputBuilder;

impl QuerySchemaVersionMetadataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.query_schema_version_metadata();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `QuerySchemaVersionMetadata`.
///
/// <p>Queries for the schema version metadata information. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct QuerySchemaVersionMetadataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::query_schema_version_metadata::builders::QuerySchemaVersionMetadataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataOutput,
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataError,
    > for QuerySchemaVersionMetadataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataOutput,
            crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl QuerySchemaVersionMetadataFluentBuilder {
    /// Creates a new `QuerySchemaVersionMetadata`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the QuerySchemaVersionMetadata as a reference.
    pub fn as_input(&self) -> &crate::operation::query_schema_version_metadata::builders::QuerySchemaVersionMetadataInputBuilder {
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
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadata::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadata::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataOutput,
        crate::operation::query_schema_version_metadata::QuerySchemaVersionMetadataError,
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
    /// <p>A wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn schema_id(mut self, input: crate::types::SchemaId) -> Self {
        self.inner = self.inner.schema_id(input);
        self
    }
    /// <p>A wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn set_schema_id(mut self, input: ::std::option::Option<crate::types::SchemaId>) -> Self {
        self.inner = self.inner.set_schema_id(input);
        self
    }
    /// <p>A wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn get_schema_id(&self) -> &::std::option::Option<crate::types::SchemaId> {
        self.inner.get_schema_id()
    }
    /// <p>The version number of the schema.</p>
    pub fn schema_version_number(mut self, input: crate::types::SchemaVersionNumber) -> Self {
        self.inner = self.inner.schema_version_number(input);
        self
    }
    /// <p>The version number of the schema.</p>
    pub fn set_schema_version_number(mut self, input: ::std::option::Option<crate::types::SchemaVersionNumber>) -> Self {
        self.inner = self.inner.set_schema_version_number(input);
        self
    }
    /// <p>The version number of the schema.</p>
    pub fn get_schema_version_number(&self) -> &::std::option::Option<crate::types::SchemaVersionNumber> {
        self.inner.get_schema_version_number()
    }
    /// <p>The unique version ID of the schema version.</p>
    pub fn schema_version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_version_id(input.into());
        self
    }
    /// <p>The unique version ID of the schema version.</p>
    pub fn set_schema_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_version_id(input);
        self
    }
    /// <p>The unique version ID of the schema version.</p>
    pub fn get_schema_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_version_id()
    }
    /// Appends an item to `MetadataList`.
    ///
    /// To override the contents of this collection use [`set_metadata_list`](Self::set_metadata_list).
    ///
    /// <p>Search key-value pairs for metadata, if they are not provided all the metadata information will be fetched.</p>
    pub fn metadata_list(mut self, input: crate::types::MetadataKeyValuePair) -> Self {
        self.inner = self.inner.metadata_list(input);
        self
    }
    /// <p>Search key-value pairs for metadata, if they are not provided all the metadata information will be fetched.</p>
    pub fn set_metadata_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetadataKeyValuePair>>) -> Self {
        self.inner = self.inner.set_metadata_list(input);
        self
    }
    /// <p>Search key-value pairs for metadata, if they are not provided all the metadata information will be fetched.</p>
    pub fn get_metadata_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetadataKeyValuePair>> {
        self.inner.get_metadata_list()
    }
    /// <p>Maximum number of results required per page. If the value is not supplied, this will be defaulted to 25 per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results required per page. If the value is not supplied, this will be defaulted to 25 per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Maximum number of results required per page. If the value is not supplied, this will be defaulted to 25 per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
