// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_trial::_create_trial_output::CreateTrialOutputBuilder;

pub use crate::operation::create_trial::_create_trial_input::CreateTrialInputBuilder;

impl CreateTrialInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_trial::CreateTrialOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_trial::CreateTrialError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_trial();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateTrial`.
///
/// <p>Creates an SageMaker <i>trial</i>. A trial is a set of steps called <i>trial components</i> that produce a machine learning model. A trial is part of a single SageMaker <i>experiment</i>.</p>
/// <p>When you use SageMaker Studio or the SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the Amazon Web Services SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p>
/// <p>You can add tags to a trial and then use the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search for the tags.</p>
/// <p>To get a list of all your trials, call the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ListTrials.html">ListTrials</a> API. To view a trial's properties, call the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DescribeTrial.html">DescribeTrial</a> API. To create a trial component, call the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTrialComponent.html">CreateTrialComponent</a> API.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTrialFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_trial::builders::CreateTrialInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_trial::CreateTrialOutput,
        crate::operation::create_trial::CreateTrialError,
    > for CreateTrialFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_trial::CreateTrialOutput,
            crate::operation::create_trial::CreateTrialError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateTrialFluentBuilder {
    /// Creates a new `CreateTrial`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateTrial as a reference.
    pub fn as_input(&self) -> &crate::operation::create_trial::builders::CreateTrialInputBuilder {
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
        crate::operation::create_trial::CreateTrialOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_trial::CreateTrialError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_trial::CreateTrial::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_trial::CreateTrial::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_trial::CreateTrialOutput,
        crate::operation::create_trial::CreateTrialError,
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
    /// <p>The name of the trial. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
    pub fn trial_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trial_name(input.into());
        self
    }
    /// <p>The name of the trial. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
    pub fn set_trial_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_trial_name(input);
        self
    }
    /// <p>The name of the trial. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
    pub fn get_trial_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_trial_name()
    }
    /// <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
    /// <p>The name of the experiment to associate the trial with.</p>
    pub fn experiment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.experiment_name(input.into());
        self
    }
    /// <p>The name of the experiment to associate the trial with.</p>
    pub fn set_experiment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_experiment_name(input);
        self
    }
    /// <p>The name of the experiment to associate the trial with.</p>
    pub fn get_experiment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_experiment_name()
    }
    /// <p>Metadata properties of the tracking entity, trial, or trial component.</p>
    pub fn metadata_properties(mut self, input: crate::types::MetadataProperties) -> Self {
        self.inner = self.inner.metadata_properties(input);
        self
    }
    /// <p>Metadata properties of the tracking entity, trial, or trial component.</p>
    pub fn set_metadata_properties(mut self, input: ::std::option::Option<crate::types::MetadataProperties>) -> Self {
        self.inner = self.inner.set_metadata_properties(input);
        self
    }
    /// <p>Metadata properties of the tracking entity, trial, or trial component.</p>
    pub fn get_metadata_properties(&self) -> &::std::option::Option<crate::types::MetadataProperties> {
        self.inner.get_metadata_properties()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to associate with the trial. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags to associate with the trial. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tags to associate with the trial. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
