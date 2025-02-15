// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_tracker::_update_tracker_output::UpdateTrackerOutputBuilder;

pub use crate::operation::update_tracker::_update_tracker_input::UpdateTrackerInputBuilder;

impl UpdateTrackerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_tracker::UpdateTrackerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_tracker::UpdateTrackerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_tracker();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateTracker`.
///
/// <p>Updates the specified properties of a given tracker resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTrackerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_tracker::builders::UpdateTrackerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_tracker::UpdateTrackerOutput,
        crate::operation::update_tracker::UpdateTrackerError,
    > for UpdateTrackerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_tracker::UpdateTrackerOutput,
            crate::operation::update_tracker::UpdateTrackerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateTrackerFluentBuilder {
    /// Creates a new `UpdateTracker`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateTracker as a reference.
    pub fn as_input(&self) -> &crate::operation::update_tracker::builders::UpdateTrackerInputBuilder {
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
        crate::operation::update_tracker::UpdateTrackerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_tracker::UpdateTrackerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_tracker::UpdateTracker::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_tracker::UpdateTracker::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_tracker::UpdateTrackerOutput,
        crate::operation::update_tracker::UpdateTrackerError,
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
    /// <p>The name of the tracker resource to update.</p>
    pub fn tracker_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tracker_name(input.into());
        self
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn set_tracker_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tracker_name(input);
        self
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn get_tracker_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_tracker_name()
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. If included, the only allowed value is RequestBasedUsage.", since = "2022-02-01")]
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.inner = self.inner.pricing_plan(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. If included, the only allowed value is RequestBasedUsage.", since = "2022-02-01")]
    pub fn set_pricing_plan(mut self, input: ::std::option::Option<crate::types::PricingPlan>) -> Self {
        self.inner = self.inner.set_pricing_plan(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. If included, the only allowed value is RequestBasedUsage.", since = "2022-02-01")]
    pub fn get_pricing_plan(&self) -> &::std::option::Option<crate::types::PricingPlan> {
        self.inner.get_pricing_plan()
    }
    /// <p>This parameter is no longer used.</p>
    #[deprecated(note = "Deprecated. No longer allowed.", since = "2022-02-01")]
    pub fn pricing_plan_data_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pricing_plan_data_source(input.into());
        self
    }
    /// <p>This parameter is no longer used.</p>
    #[deprecated(note = "Deprecated. No longer allowed.", since = "2022-02-01")]
    pub fn set_pricing_plan_data_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pricing_plan_data_source(input);
        self
    }
    /// <p>This parameter is no longer used.</p>
    #[deprecated(note = "Deprecated. No longer allowed.", since = "2022-02-01")]
    pub fn get_pricing_plan_data_source(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pricing_plan_data_source()
    }
    /// <p>Updates the description for the tracker resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Updates the description for the tracker resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Updates the description for the tracker resource.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Updates the position filtering for the tracker resource.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li> <p> <code>TimeBased</code> - Location updates are evaluated against linked geofence collections, but not every location update is stored. If your update frequency is more often than 30 seconds, only one update per 30 seconds is stored for each unique device ID. </p> </li>
    /// <li> <p> <code>DistanceBased</code> - If the device has moved less than 30 m (98.4 ft), location updates are ignored. Location updates within this distance are neither evaluated against linked geofence collections, nor stored. This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through. Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map. </p> </li>
    /// <li> <p> <code>AccuracyBased</code> - If the device has moved less than the measured accuracy, location updates are ignored. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device has moved less than 15 m. Ignored location updates are neither evaluated against linked geofence collections, nor stored. This helps educe the effects of GPS noise when displaying device trajectories on a map, and can help control costs by reducing the number of geofence evaluations. </p> </li>
    /// </ul>
    pub fn position_filtering(mut self, input: crate::types::PositionFiltering) -> Self {
        self.inner = self.inner.position_filtering(input);
        self
    }
    /// <p>Updates the position filtering for the tracker resource.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li> <p> <code>TimeBased</code> - Location updates are evaluated against linked geofence collections, but not every location update is stored. If your update frequency is more often than 30 seconds, only one update per 30 seconds is stored for each unique device ID. </p> </li>
    /// <li> <p> <code>DistanceBased</code> - If the device has moved less than 30 m (98.4 ft), location updates are ignored. Location updates within this distance are neither evaluated against linked geofence collections, nor stored. This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through. Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map. </p> </li>
    /// <li> <p> <code>AccuracyBased</code> - If the device has moved less than the measured accuracy, location updates are ignored. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device has moved less than 15 m. Ignored location updates are neither evaluated against linked geofence collections, nor stored. This helps educe the effects of GPS noise when displaying device trajectories on a map, and can help control costs by reducing the number of geofence evaluations. </p> </li>
    /// </ul>
    pub fn set_position_filtering(mut self, input: ::std::option::Option<crate::types::PositionFiltering>) -> Self {
        self.inner = self.inner.set_position_filtering(input);
        self
    }
    /// <p>Updates the position filtering for the tracker resource.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li> <p> <code>TimeBased</code> - Location updates are evaluated against linked geofence collections, but not every location update is stored. If your update frequency is more often than 30 seconds, only one update per 30 seconds is stored for each unique device ID. </p> </li>
    /// <li> <p> <code>DistanceBased</code> - If the device has moved less than 30 m (98.4 ft), location updates are ignored. Location updates within this distance are neither evaluated against linked geofence collections, nor stored. This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through. Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map. </p> </li>
    /// <li> <p> <code>AccuracyBased</code> - If the device has moved less than the measured accuracy, location updates are ignored. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device has moved less than 15 m. Ignored location updates are neither evaluated against linked geofence collections, nor stored. This helps educe the effects of GPS noise when displaying device trajectories on a map, and can help control costs by reducing the number of geofence evaluations. </p> </li>
    /// </ul>
    pub fn get_position_filtering(&self) -> &::std::option::Option<crate::types::PositionFiltering> {
        self.inner.get_position_filtering()
    }
    /// <p>Whether to enable position <code>UPDATE</code> events from this tracker to be sent to EventBridge.</p> <note>
    /// <p>You do not need enable this feature to get <code>ENTER</code> and <code>EXIT</code> events for geofences with this tracker. Those events are always sent to EventBridge.</p>
    /// </note>
    pub fn event_bridge_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.event_bridge_enabled(input);
        self
    }
    /// <p>Whether to enable position <code>UPDATE</code> events from this tracker to be sent to EventBridge.</p> <note>
    /// <p>You do not need enable this feature to get <code>ENTER</code> and <code>EXIT</code> events for geofences with this tracker. Those events are always sent to EventBridge.</p>
    /// </note>
    pub fn set_event_bridge_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_event_bridge_enabled(input);
        self
    }
    /// <p>Whether to enable position <code>UPDATE</code> events from this tracker to be sent to EventBridge.</p> <note>
    /// <p>You do not need enable this feature to get <code>ENTER</code> and <code>EXIT</code> events for geofences with this tracker. Those events are always sent to EventBridge.</p>
    /// </note>
    pub fn get_event_bridge_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_event_bridge_enabled()
    }
    /// <p>Enables <code>GeospatialQueries</code> for a tracker that uses a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a>.</p>
    /// <p>This parameter is only used if you are using a KMS customer managed key.</p>
    pub fn kms_key_enable_geospatial_queries(mut self, input: bool) -> Self {
        self.inner = self.inner.kms_key_enable_geospatial_queries(input);
        self
    }
    /// <p>Enables <code>GeospatialQueries</code> for a tracker that uses a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a>.</p>
    /// <p>This parameter is only used if you are using a KMS customer managed key.</p>
    pub fn set_kms_key_enable_geospatial_queries(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_kms_key_enable_geospatial_queries(input);
        self
    }
    /// <p>Enables <code>GeospatialQueries</code> for a tracker that uses a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a>.</p>
    /// <p>This parameter is only used if you are using a KMS customer managed key.</p>
    pub fn get_kms_key_enable_geospatial_queries(&self) -> &::std::option::Option<bool> {
        self.inner.get_kms_key_enable_geospatial_queries()
    }
}
