// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_provisioned_model_throughput::_create_provisioned_model_throughput_output::CreateProvisionedModelThroughputOutputBuilder;

pub use crate::operation::create_provisioned_model_throughput::_create_provisioned_model_throughput_input::CreateProvisionedModelThroughputInputBuilder;

impl CreateProvisionedModelThroughputInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_provisioned_model_throughput();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateProvisionedModelThroughput`.
///
/// <p>Creates a provisioned throughput with dedicated capacity for a foundation model or a fine-tuned model.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/what-is-service.html">Provisioned throughput</a> in the Bedrock User Guide.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateProvisionedModelThroughputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_provisioned_model_throughput::builders::CreateProvisionedModelThroughputInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputOutput,
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputError,
    > for CreateProvisionedModelThroughputFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputOutput,
            crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateProvisionedModelThroughputFluentBuilder {
    /// Creates a new `CreateProvisionedModelThroughput`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateProvisionedModelThroughput as a reference.
    pub fn as_input(&self) -> &crate::operation::create_provisioned_model_throughput::builders::CreateProvisionedModelThroughputInputBuilder {
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
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughput::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughput::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputOutput,
        crate::operation::create_provisioned_model_throughput::CreateProvisionedModelThroughputError,
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
    /// <p>Unique token value that you can provide. If this token matches a previous request, Bedrock ignores the request, but does not return an error.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Unique token value that you can provide. If this token matches a previous request, Bedrock ignores the request, but does not return an error.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Unique token value that you can provide. If this token matches a previous request, Bedrock ignores the request, but does not return an error.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>Number of model units to allocate.</p>
    pub fn model_units(mut self, input: i32) -> Self {
        self.inner = self.inner.model_units(input);
        self
    }
    /// <p>Number of model units to allocate.</p>
    pub fn set_model_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_model_units(input);
        self
    }
    /// <p>Number of model units to allocate.</p>
    pub fn get_model_units(&self) -> &::std::option::Option<i32> {
        self.inner.get_model_units()
    }
    /// <p>Unique name for this provisioned throughput.</p>
    pub fn provisioned_model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provisioned_model_name(input.into());
        self
    }
    /// <p>Unique name for this provisioned throughput.</p>
    pub fn set_provisioned_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provisioned_model_name(input);
        self
    }
    /// <p>Unique name for this provisioned throughput.</p>
    pub fn get_provisioned_model_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provisioned_model_name()
    }
    /// <p>Name or ARN of the model to associate with this provisioned throughput.</p>
    pub fn model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_id(input.into());
        self
    }
    /// <p>Name or ARN of the model to associate with this provisioned throughput.</p>
    pub fn set_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_id(input);
        self
    }
    /// <p>Name or ARN of the model to associate with this provisioned throughput.</p>
    pub fn get_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_id()
    }
    /// <p>Commitment duration requested for the provisioned throughput.</p>
    pub fn commitment_duration(mut self, input: crate::types::CommitmentDuration) -> Self {
        self.inner = self.inner.commitment_duration(input);
        self
    }
    /// <p>Commitment duration requested for the provisioned throughput.</p>
    pub fn set_commitment_duration(mut self, input: ::std::option::Option<crate::types::CommitmentDuration>) -> Self {
        self.inner = self.inner.set_commitment_duration(input);
        self
    }
    /// <p>Commitment duration requested for the provisioned throughput.</p>
    pub fn get_commitment_duration(&self) -> &::std::option::Option<crate::types::CommitmentDuration> {
        self.inner.get_commitment_duration()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to associate with this provisioned throughput.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Tags to associate with this provisioned throughput.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags to associate with this provisioned throughput.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
