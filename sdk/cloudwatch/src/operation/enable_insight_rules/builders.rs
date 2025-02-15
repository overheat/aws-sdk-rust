// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_insight_rules::_enable_insight_rules_output::EnableInsightRulesOutputBuilder;

pub use crate::operation::enable_insight_rules::_enable_insight_rules_input::EnableInsightRulesInputBuilder;

impl EnableInsightRulesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_insight_rules::EnableInsightRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_insight_rules::EnableInsightRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_insight_rules();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableInsightRules`.
///
/// <p>Enables the specified Contributor Insights rules. When rules are enabled, they immediately begin analyzing log data.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableInsightRulesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_insight_rules::builders::EnableInsightRulesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::enable_insight_rules::EnableInsightRulesOutput,
        crate::operation::enable_insight_rules::EnableInsightRulesError,
    > for EnableInsightRulesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::enable_insight_rules::EnableInsightRulesOutput,
            crate::operation::enable_insight_rules::EnableInsightRulesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EnableInsightRulesFluentBuilder {
    /// Creates a new `EnableInsightRules`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableInsightRules as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_insight_rules::builders::EnableInsightRulesInputBuilder {
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
        crate::operation::enable_insight_rules::EnableInsightRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_insight_rules::EnableInsightRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_insight_rules::EnableInsightRules::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_insight_rules::EnableInsightRules::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::enable_insight_rules::EnableInsightRulesOutput,
        crate::operation::enable_insight_rules::EnableInsightRulesError,
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
    /// Appends an item to `RuleNames`.
    ///
    /// To override the contents of this collection use [`set_rule_names`](Self::set_rule_names).
    ///
    /// <p>An array of the rule names to enable. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
    pub fn rule_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_names(input.into());
        self
    }
    /// <p>An array of the rule names to enable. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
    pub fn set_rule_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_rule_names(input);
        self
    }
    /// <p>An array of the rule names to enable. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
    pub fn get_rule_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_rule_names()
    }
}
