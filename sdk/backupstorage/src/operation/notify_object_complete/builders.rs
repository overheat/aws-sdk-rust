// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::notify_object_complete::_notify_object_complete_output::NotifyObjectCompleteOutputBuilder;

pub use crate::operation::notify_object_complete::_notify_object_complete_input::NotifyObjectCompleteInputBuilder;

impl NotifyObjectCompleteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::notify_object_complete::NotifyObjectCompleteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::notify_object_complete::NotifyObjectCompleteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.notify_object_complete();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `NotifyObjectComplete`.
///
/// Complete upload
#[derive(::std::fmt::Debug)]
pub struct NotifyObjectCompleteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::notify_object_complete::builders::NotifyObjectCompleteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::notify_object_complete::NotifyObjectCompleteOutput,
        crate::operation::notify_object_complete::NotifyObjectCompleteError,
    > for NotifyObjectCompleteFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::notify_object_complete::NotifyObjectCompleteOutput,
            crate::operation::notify_object_complete::NotifyObjectCompleteError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl NotifyObjectCompleteFluentBuilder {
    /// Creates a new `NotifyObjectComplete`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the NotifyObjectComplete as a reference.
    pub fn as_input(&self) -> &crate::operation::notify_object_complete::builders::NotifyObjectCompleteInputBuilder {
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
        crate::operation::notify_object_complete::NotifyObjectCompleteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::notify_object_complete::NotifyObjectCompleteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::notify_object_complete::NotifyObjectComplete::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::notify_object_complete::NotifyObjectComplete::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::notify_object_complete::NotifyObjectCompleteOutput,
        crate::operation::notify_object_complete::NotifyObjectCompleteError,
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
    /// Backup job Id for the in-progress backup
    pub fn backup_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_job_id(input.into());
        self
    }
    /// Backup job Id for the in-progress backup
    pub fn set_backup_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_job_id(input);
        self
    }
    /// Backup job Id for the in-progress backup
    pub fn get_backup_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_job_id()
    }
    /// Upload Id for the in-progress upload
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    /// Upload Id for the in-progress upload
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
    /// Upload Id for the in-progress upload
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_upload_id()
    }
    /// Object checksum
    pub fn object_checksum(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.object_checksum(input.into());
        self
    }
    /// Object checksum
    pub fn set_object_checksum(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_object_checksum(input);
        self
    }
    /// Object checksum
    pub fn get_object_checksum(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_object_checksum()
    }
    /// Checksum algorithm
    pub fn object_checksum_algorithm(mut self, input: crate::types::SummaryChecksumAlgorithm) -> Self {
        self.inner = self.inner.object_checksum_algorithm(input);
        self
    }
    /// Checksum algorithm
    pub fn set_object_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::SummaryChecksumAlgorithm>) -> Self {
        self.inner = self.inner.set_object_checksum_algorithm(input);
        self
    }
    /// Checksum algorithm
    pub fn get_object_checksum_algorithm(&self) -> &::std::option::Option<crate::types::SummaryChecksumAlgorithm> {
        self.inner.get_object_checksum_algorithm()
    }
    /// Optional metadata associated with an Object. Maximum string length is 256 bytes.
    pub fn metadata_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metadata_string(input.into());
        self
    }
    /// Optional metadata associated with an Object. Maximum string length is 256 bytes.
    pub fn set_metadata_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metadata_string(input);
        self
    }
    /// Optional metadata associated with an Object. Maximum string length is 256 bytes.
    pub fn get_metadata_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metadata_string()
    }
    /// Optional metadata associated with an Object. Maximum length is 4MB.
    pub fn metadata_blob(mut self, input: ::aws_smithy_types::byte_stream::ByteStream) -> Self {
        self.inner = self.inner.metadata_blob(input);
        self
    }
    /// Optional metadata associated with an Object. Maximum length is 4MB.
    pub fn set_metadata_blob(mut self, input: ::std::option::Option<::aws_smithy_types::byte_stream::ByteStream>) -> Self {
        self.inner = self.inner.set_metadata_blob(input);
        self
    }
    /// Optional metadata associated with an Object. Maximum length is 4MB.
    pub fn get_metadata_blob(&self) -> &::std::option::Option<::aws_smithy_types::byte_stream::ByteStream> {
        self.inner.get_metadata_blob()
    }
    /// The size of MetadataBlob.
    pub fn metadata_blob_length(mut self, input: i64) -> Self {
        self.inner = self.inner.metadata_blob_length(input);
        self
    }
    /// The size of MetadataBlob.
    pub fn set_metadata_blob_length(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_metadata_blob_length(input);
        self
    }
    /// The size of MetadataBlob.
    pub fn get_metadata_blob_length(&self) -> &::std::option::Option<i64> {
        self.inner.get_metadata_blob_length()
    }
    /// Checksum of MetadataBlob.
    pub fn metadata_blob_checksum(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metadata_blob_checksum(input.into());
        self
    }
    /// Checksum of MetadataBlob.
    pub fn set_metadata_blob_checksum(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metadata_blob_checksum(input);
        self
    }
    /// Checksum of MetadataBlob.
    pub fn get_metadata_blob_checksum(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metadata_blob_checksum()
    }
    /// Checksum algorithm.
    pub fn metadata_blob_checksum_algorithm(mut self, input: crate::types::DataChecksumAlgorithm) -> Self {
        self.inner = self.inner.metadata_blob_checksum_algorithm(input);
        self
    }
    /// Checksum algorithm.
    pub fn set_metadata_blob_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::DataChecksumAlgorithm>) -> Self {
        self.inner = self.inner.set_metadata_blob_checksum_algorithm(input);
        self
    }
    /// Checksum algorithm.
    pub fn get_metadata_blob_checksum_algorithm(&self) -> &::std::option::Option<crate::types::DataChecksumAlgorithm> {
        self.inner.get_metadata_blob_checksum_algorithm()
    }
}
