// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dataset::_create_dataset_output::CreateDatasetOutputBuilder;

pub use crate::operation::create_dataset::_create_dataset_input::CreateDatasetInputBuilder;

impl CreateDatasetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_dataset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDataset`.
///
/// <p>Creates a container for a collection of data being ingested for analysis. The dataset contains the metadata describing where the data is and what the data actually looks like. For example, it contains the location of the data source, the data schema, and other information. A dataset also contains any tags associated with the ingested data. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatasetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dataset::builders::CreateDatasetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
    > for CreateDatasetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_dataset::CreateDatasetOutput,
            crate::operation::create_dataset::CreateDatasetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDatasetFluentBuilder {
    /// Creates a new `CreateDataset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDataset as a reference.
    pub fn as_input(&self) -> &crate::operation::create_dataset::builders::CreateDatasetInputBuilder {
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
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_dataset::CreateDataset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_dataset::CreateDataset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
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
    /// <p>The name of the dataset being created. </p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_name(input.into());
        self
    }
    /// <p>The name of the dataset being created. </p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_name(input);
        self
    }
    /// <p>The name of the dataset being created. </p>
    pub fn get_dataset_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_name()
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn dataset_schema(mut self, input: crate::types::DatasetSchema) -> Self {
        self.inner = self.inner.dataset_schema(input);
        self
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn set_dataset_schema(mut self, input: ::std::option::Option<crate::types::DatasetSchema>) -> Self {
        self.inner = self.inner.set_dataset_schema(input);
        self
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn get_dataset_schema(&self) -> &::std::option::Option<crate::types::DatasetSchema> {
        self.inner.get_dataset_schema()
    }
    /// <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout for Equipment. </p>
    pub fn server_side_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.server_side_kms_key_id(input.into());
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout for Equipment. </p>
    pub fn set_server_side_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_server_side_kms_key_id(input);
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout for Equipment. </p>
    pub fn get_server_side_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_server_side_kms_key_id()
    }
    /// <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags associated with the ingested data described in the dataset. </p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Any tags associated with the ingested data described in the dataset. </p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Any tags associated with the ingested data described in the dataset. </p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
