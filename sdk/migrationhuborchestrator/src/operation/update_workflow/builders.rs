// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_workflow::_update_workflow_output::UpdateWorkflowOutputBuilder;

pub use crate::operation::update_workflow::_update_workflow_input::UpdateWorkflowInputBuilder;

impl UpdateWorkflowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_workflow::UpdateWorkflowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_workflow::UpdateWorkflowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_workflow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateWorkflow`.
///
/// <p>Update a migration workflow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateWorkflowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_workflow::builders::UpdateWorkflowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_workflow::UpdateWorkflowOutput,
        crate::operation::update_workflow::UpdateWorkflowError,
    > for UpdateWorkflowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_workflow::UpdateWorkflowOutput,
            crate::operation::update_workflow::UpdateWorkflowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateWorkflowFluentBuilder {
    /// Creates a new `UpdateWorkflow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateWorkflow as a reference.
    pub fn as_input(&self) -> &crate::operation::update_workflow::builders::UpdateWorkflowInputBuilder {
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
        crate::operation::update_workflow::UpdateWorkflowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_workflow::UpdateWorkflowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_workflow::UpdateWorkflow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_workflow::UpdateWorkflow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_workflow::UpdateWorkflowOutput,
        crate::operation::update_workflow::UpdateWorkflowError,
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
    /// <p>The ID of the migration workflow.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The name of the migration workflow.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the migration workflow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the migration workflow.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description of the migration workflow.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the migration workflow.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the migration workflow.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Adds a key-value pair to `inputParameters`.
    ///
    /// To override the contents of this collection use [`set_input_parameters`](Self::set_input_parameters).
    ///
    /// <p>The input parameters required to update a migration workflow.</p>
    pub fn input_parameters(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::StepInput) -> Self {
        self.inner = self.inner.input_parameters(k.into(), v);
        self
    }
    /// <p>The input parameters required to update a migration workflow.</p>
    pub fn set_input_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::StepInput>>,
    ) -> Self {
        self.inner = self.inner.set_input_parameters(input);
        self
    }
    /// <p>The input parameters required to update a migration workflow.</p>
    pub fn get_input_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::StepInput>> {
        self.inner.get_input_parameters()
    }
    /// Appends an item to `stepTargets`.
    ///
    /// To override the contents of this collection use [`set_step_targets`](Self::set_step_targets).
    ///
    /// <p>The servers on which a step will be run.</p>
    pub fn step_targets(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.step_targets(input.into());
        self
    }
    /// <p>The servers on which a step will be run.</p>
    pub fn set_step_targets(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_step_targets(input);
        self
    }
    /// <p>The servers on which a step will be run.</p>
    pub fn get_step_targets(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_step_targets()
    }
}
