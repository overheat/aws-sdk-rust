// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::evaluate_feature::_evaluate_feature_output::EvaluateFeatureOutputBuilder;

pub use crate::operation::evaluate_feature::_evaluate_feature_input::EvaluateFeatureInputBuilder;

impl EvaluateFeatureInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::evaluate_feature::EvaluateFeatureOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::evaluate_feature::EvaluateFeatureError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.evaluate_feature();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EvaluateFeature`.
///
/// <p>This operation assigns a feature variation to one given user session. You pass in an <code>entityID</code> that represents the user. Evidently then checks the evaluation rules and assigns the variation.</p>
/// <p>The first rules that are evaluated are the override rules. If the user's <code>entityID</code> matches an override rule, the user is served the variation specified by that rule.</p>
/// <p>If there is a current launch with this feature that uses segment overrides, and if the user session's <code>evaluationContext</code> matches a segment rule defined in a segment override, the configuration in the segment overrides is used. For more information about segments, see <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_CreateSegment.html">CreateSegment</a> and <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
/// <p>If there is a launch with no segment overrides, the user might be assigned to a variation in the launch. The chance of this depends on the percentage of users that are allocated to that launch. If the user is enrolled in the launch, the variation they are served depends on the allocation of the various feature variations used for the launch.</p>
/// <p>If the user is not assigned to a launch, and there is an ongoing experiment for this feature, the user might be assigned to a variation in the experiment. The chance of this depends on the percentage of users that are allocated to that experiment.</p>
/// <p>If the experiment uses a segment, then only user sessions with <code>evaluationContext</code> values that match the segment rule are used in the experiment.</p>
/// <p>If the user is enrolled in the experiment, the variation they are served depends on the allocation of the various feature variations used for the experiment. </p>
/// <p>If the user is not assigned to a launch or experiment, they are served the default variation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EvaluateFeatureFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::evaluate_feature::builders::EvaluateFeatureInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::evaluate_feature::EvaluateFeatureOutput,
        crate::operation::evaluate_feature::EvaluateFeatureError,
    > for EvaluateFeatureFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::evaluate_feature::EvaluateFeatureOutput,
            crate::operation::evaluate_feature::EvaluateFeatureError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EvaluateFeatureFluentBuilder {
    /// Creates a new `EvaluateFeature`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EvaluateFeature as a reference.
    pub fn as_input(&self) -> &crate::operation::evaluate_feature::builders::EvaluateFeatureInputBuilder {
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
        crate::operation::evaluate_feature::EvaluateFeatureOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::evaluate_feature::EvaluateFeatureError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::evaluate_feature::EvaluateFeature::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::evaluate_feature::EvaluateFeature::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::evaluate_feature::EvaluateFeatureOutput,
        crate::operation::evaluate_feature::EvaluateFeatureError,
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
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project(input);
        self
    }
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn get_project(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project()
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn feature(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.feature(input.into());
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn set_feature(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_feature(input);
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn get_feature(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_feature()
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn get_entity_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_entity_id()
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn evaluation_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.evaluation_context(input.into());
        self
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn set_evaluation_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_evaluation_context(input);
        self
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn get_evaluation_context(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_evaluation_context()
    }
}
