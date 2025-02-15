// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_filter::_update_filter_output::UpdateFilterOutputBuilder;

pub use crate::operation::update_filter::_update_filter_input::UpdateFilterInputBuilder;

impl UpdateFilterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_filter::UpdateFilterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_filter::UpdateFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_filter();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateFilter`.
///
/// <p>Updates the filter specified by the filter name.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFilterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_filter::builders::UpdateFilterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_filter::UpdateFilterOutput,
        crate::operation::update_filter::UpdateFilterError,
    > for UpdateFilterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_filter::UpdateFilterOutput,
            crate::operation::update_filter::UpdateFilterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateFilterFluentBuilder {
    /// Creates a new `UpdateFilter`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateFilter as a reference.
    pub fn as_input(&self) -> &crate::operation::update_filter::builders::UpdateFilterInputBuilder {
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
        crate::operation::update_filter::UpdateFilterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_filter::UpdateFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_filter::UpdateFilter::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_filter::UpdateFilter::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_filter::UpdateFilterOutput,
        crate::operation::update_filter::UpdateFilterError,
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
    /// <p>The unique ID of the detector that specifies the GuardDuty service where you want to update a filter.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector that specifies the GuardDuty service where you want to update a filter.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID of the detector that specifies the GuardDuty service where you want to update a filter.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>The name of the filter.</p>
    pub fn filter_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_name(input.into());
        self
    }
    /// <p>The name of the filter.</p>
    pub fn set_filter_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_filter_name(input);
        self
    }
    /// <p>The name of the filter.</p>
    pub fn get_filter_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_filter_name()
    }
    /// <p>The description of the filter. Valid characters include alphanumeric characters, and special characters such as hyphen, period, colon, underscore, parentheses (<code>{ }</code>, <code>[ ]</code>, and <code>( )</code>), forward slash, horizontal tab, vertical tab, newline, form feed, return, and whitespace.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the filter. Valid characters include alphanumeric characters, and special characters such as hyphen, period, colon, underscore, parentheses (<code>{ }</code>, <code>[ ]</code>, and <code>( )</code>), forward slash, horizontal tab, vertical tab, newline, form feed, return, and whitespace.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the filter. Valid characters include alphanumeric characters, and special characters such as hyphen, period, colon, underscore, parentheses (<code>{ }</code>, <code>[ ]</code>, and <code>( )</code>), forward slash, horizontal tab, vertical tab, newline, form feed, return, and whitespace.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    pub fn action(mut self, input: crate::types::FilterAction) -> Self {
        self.inner = self.inner.action(input);
        self
    }
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::FilterAction>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    pub fn get_action(&self) -> &::std::option::Option<crate::types::FilterAction> {
        self.inner.get_action()
    }
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    pub fn rank(mut self, input: i32) -> Self {
        self.inner = self.inner.rank(input);
        self
    }
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    pub fn set_rank(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_rank(input);
        self
    }
    /// <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    pub fn get_rank(&self) -> &::std::option::Option<i32> {
        self.inner.get_rank()
    }
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    pub fn finding_criteria(mut self, input: crate::types::FindingCriteria) -> Self {
        self.inner = self.inner.finding_criteria(input);
        self
    }
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    pub fn set_finding_criteria(mut self, input: ::std::option::Option<crate::types::FindingCriteria>) -> Self {
        self.inner = self.inner.set_finding_criteria(input);
        self
    }
    /// <p>Represents the criteria to be used in the filter for querying findings.</p>
    pub fn get_finding_criteria(&self) -> &::std::option::Option<crate::types::FindingCriteria> {
        self.inner.get_finding_criteria()
    }
}
