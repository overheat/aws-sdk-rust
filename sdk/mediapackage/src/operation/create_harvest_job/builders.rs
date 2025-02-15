// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_harvest_job::_create_harvest_job_output::CreateHarvestJobOutputBuilder;

pub use crate::operation::create_harvest_job::_create_harvest_job_input::CreateHarvestJobInputBuilder;

impl CreateHarvestJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_harvest_job::CreateHarvestJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_harvest_job::CreateHarvestJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_harvest_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateHarvestJob`.
///
/// Creates a new HarvestJob record.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateHarvestJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_harvest_job::builders::CreateHarvestJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_harvest_job::CreateHarvestJobOutput,
        crate::operation::create_harvest_job::CreateHarvestJobError,
    > for CreateHarvestJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_harvest_job::CreateHarvestJobOutput,
            crate::operation::create_harvest_job::CreateHarvestJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateHarvestJobFluentBuilder {
    /// Creates a new `CreateHarvestJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateHarvestJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_harvest_job::builders::CreateHarvestJobInputBuilder {
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
        crate::operation::create_harvest_job::CreateHarvestJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_harvest_job::CreateHarvestJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_harvest_job::CreateHarvestJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_harvest_job::CreateHarvestJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_harvest_job::CreateHarvestJobOutput,
        crate::operation::create_harvest_job::CreateHarvestJobError,
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
    /// The end of the time-window which will be harvested
    pub fn end_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.end_time(input.into());
        self
    }
    /// The end of the time-window which will be harvested
    pub fn set_end_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// The end of the time-window which will be harvested
    pub fn get_end_time(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_end_time()
    }
    /// The ID of the HarvestJob. The ID must be unique within the region and it cannot be changed after the HarvestJob is submitted
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// The ID of the HarvestJob. The ID must be unique within the region and it cannot be changed after the HarvestJob is submitted
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// The ID of the HarvestJob. The ID must be unique within the region and it cannot be changed after the HarvestJob is submitted
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// The ID of the OriginEndpoint that the HarvestJob will harvest from. This cannot be changed after the HarvestJob is submitted.
    pub fn origin_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.origin_endpoint_id(input.into());
        self
    }
    /// The ID of the OriginEndpoint that the HarvestJob will harvest from. This cannot be changed after the HarvestJob is submitted.
    pub fn set_origin_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_origin_endpoint_id(input);
        self
    }
    /// The ID of the OriginEndpoint that the HarvestJob will harvest from. This cannot be changed after the HarvestJob is submitted.
    pub fn get_origin_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_origin_endpoint_id()
    }
    /// Configuration parameters for where in an S3 bucket to place the harvested content
    pub fn s3_destination(mut self, input: crate::types::S3Destination) -> Self {
        self.inner = self.inner.s3_destination(input);
        self
    }
    /// Configuration parameters for where in an S3 bucket to place the harvested content
    pub fn set_s3_destination(mut self, input: ::std::option::Option<crate::types::S3Destination>) -> Self {
        self.inner = self.inner.set_s3_destination(input);
        self
    }
    /// Configuration parameters for where in an S3 bucket to place the harvested content
    pub fn get_s3_destination(&self) -> &::std::option::Option<crate::types::S3Destination> {
        self.inner.get_s3_destination()
    }
    /// The start of the time-window which will be harvested
    pub fn start_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_time(input.into());
        self
    }
    /// The start of the time-window which will be harvested
    pub fn set_start_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// The start of the time-window which will be harvested
    pub fn get_start_time(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_start_time()
    }
}
