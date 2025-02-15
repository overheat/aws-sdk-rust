// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_route_analysis::_start_route_analysis_output::StartRouteAnalysisOutputBuilder;

pub use crate::operation::start_route_analysis::_start_route_analysis_input::StartRouteAnalysisInputBuilder;

impl StartRouteAnalysisInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_route_analysis::StartRouteAnalysisOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_route_analysis::StartRouteAnalysisError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_route_analysis();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartRouteAnalysis`.
///
/// <p>Starts analyzing the routing path between the specified source and destination. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/route-analyzer.html">Route Analyzer</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartRouteAnalysisFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_route_analysis::builders::StartRouteAnalysisInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_route_analysis::StartRouteAnalysisOutput,
        crate::operation::start_route_analysis::StartRouteAnalysisError,
    > for StartRouteAnalysisFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_route_analysis::StartRouteAnalysisOutput,
            crate::operation::start_route_analysis::StartRouteAnalysisError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartRouteAnalysisFluentBuilder {
    /// Creates a new `StartRouteAnalysis`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartRouteAnalysis as a reference.
    pub fn as_input(&self) -> &crate::operation::start_route_analysis::builders::StartRouteAnalysisInputBuilder {
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
        crate::operation::start_route_analysis::StartRouteAnalysisOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_route_analysis::StartRouteAnalysisError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_route_analysis::StartRouteAnalysis::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_route_analysis::StartRouteAnalysis::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_route_analysis::StartRouteAnalysisOutput,
        crate::operation::start_route_analysis::StartRouteAnalysisError,
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
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_network_id()
    }
    /// <p>The source from which traffic originates.</p>
    pub fn source(mut self, input: crate::types::RouteAnalysisEndpointOptionsSpecification) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>The source from which traffic originates.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::RouteAnalysisEndpointOptionsSpecification>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The source from which traffic originates.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::RouteAnalysisEndpointOptionsSpecification> {
        self.inner.get_source()
    }
    /// <p>The destination.</p>
    pub fn destination(mut self, input: crate::types::RouteAnalysisEndpointOptionsSpecification) -> Self {
        self.inner = self.inner.destination(input);
        self
    }
    /// <p>The destination.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<crate::types::RouteAnalysisEndpointOptionsSpecification>) -> Self {
        self.inner = self.inner.set_destination(input);
        self
    }
    /// <p>The destination.</p>
    pub fn get_destination(&self) -> &::std::option::Option<crate::types::RouteAnalysisEndpointOptionsSpecification> {
        self.inner.get_destination()
    }
    /// <p>Indicates whether to analyze the return path. The default is <code>false</code>.</p>
    pub fn include_return_path(mut self, input: bool) -> Self {
        self.inner = self.inner.include_return_path(input);
        self
    }
    /// <p>Indicates whether to analyze the return path. The default is <code>false</code>.</p>
    pub fn set_include_return_path(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_return_path(input);
        self
    }
    /// <p>Indicates whether to analyze the return path. The default is <code>false</code>.</p>
    pub fn get_include_return_path(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_return_path()
    }
    /// <p>Indicates whether to include the location of middlebox appliances in the route analysis. The default is <code>false</code>.</p>
    pub fn use_middleboxes(mut self, input: bool) -> Self {
        self.inner = self.inner.use_middleboxes(input);
        self
    }
    /// <p>Indicates whether to include the location of middlebox appliances in the route analysis. The default is <code>false</code>.</p>
    pub fn set_use_middleboxes(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_middleboxes(input);
        self
    }
    /// <p>Indicates whether to include the location of middlebox appliances in the route analysis. The default is <code>false</code>.</p>
    pub fn get_use_middleboxes(&self) -> &::std::option::Option<bool> {
        self.inner.get_use_middleboxes()
    }
}
