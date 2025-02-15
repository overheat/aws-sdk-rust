// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::submit_multi_region_access_point_routes::_submit_multi_region_access_point_routes_output::SubmitMultiRegionAccessPointRoutesOutputBuilder;

pub use crate::operation::submit_multi_region_access_point_routes::_submit_multi_region_access_point_routes_input::SubmitMultiRegionAccessPointRoutesInputBuilder;

impl SubmitMultiRegionAccessPointRoutesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.submit_multi_region_access_point_routes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SubmitMultiRegionAccessPointRoutes`.
///
/// <p>Submits an updated route configuration for a Multi-Region Access Point. This API operation updates the routing status for the specified Regions from active to passive, or from passive to active. A value of <code>0</code> indicates a passive status, which means that traffic won't be routed to the specified Region. A value of <code>100</code> indicates an active status, which means that traffic will be routed to the specified Region. At least one Region must be active at all times.</p>
/// <p>When the routing configuration is changed, any in-progress operations (uploads, copies, deletes, and so on) to formerly active Regions will continue to run to their final completion state (success or failure). The routing configurations of any Regions that aren’t specified remain unchanged.</p> <note>
/// <p>Updated routing configurations might not be immediately applied. It can take up to 2 minutes for your changes to take effect.</p>
/// </note>
/// <p>To submit routing control changes and failover requests, use the Amazon S3 failover control infrastructure endpoints in these five Amazon Web Services Regions:</p>
/// <ul>
/// <li> <p> <code>us-east-1</code> </p> </li>
/// <li> <p> <code>us-west-2</code> </p> </li>
/// <li> <p> <code>ap-southeast-2</code> </p> </li>
/// <li> <p> <code>ap-northeast-1</code> </p> </li>
/// <li> <p> <code>eu-west-1</code> </p> </li>
/// </ul> <note>
/// <p>Your Amazon S3 bucket does not need to be in these five Regions.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SubmitMultiRegionAccessPointRoutesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput,
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError,
    > for SubmitMultiRegionAccessPointRoutesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput,
            crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SubmitMultiRegionAccessPointRoutesFluentBuilder {
    /// Creates a new `SubmitMultiRegionAccessPointRoutes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SubmitMultiRegionAccessPointRoutes as a reference.
    pub fn as_input(&self) -> &crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesInputBuilder {
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
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutes::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput,
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesError,
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
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn mrap(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mrap(input.into());
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn set_mrap(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mrap(input);
        self
    }
    /// <p>The Multi-Region Access Point ARN.</p>
    pub fn get_mrap(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mrap()
    }
    /// Appends an item to `RouteUpdates`.
    ///
    /// To override the contents of this collection use [`set_route_updates`](Self::set_route_updates).
    ///
    /// <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
    pub fn route_updates(mut self, input: crate::types::MultiRegionAccessPointRoute) -> Self {
        self.inner = self.inner.route_updates(input);
        self
    }
    /// <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
    pub fn set_route_updates(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MultiRegionAccessPointRoute>>) -> Self {
        self.inner = self.inner.set_route_updates(input);
        self
    }
    /// <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
    pub fn get_route_updates(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MultiRegionAccessPointRoute>> {
        self.inner.get_route_updates()
    }
}
