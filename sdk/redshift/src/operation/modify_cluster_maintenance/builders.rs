// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_cluster_maintenance::_modify_cluster_maintenance_output::ModifyClusterMaintenanceOutputBuilder;

pub use crate::operation::modify_cluster_maintenance::_modify_cluster_maintenance_input::ModifyClusterMaintenanceInputBuilder;

impl ModifyClusterMaintenanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_cluster_maintenance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyClusterMaintenance`.
///
/// <p>Modifies the maintenance settings of a cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyClusterMaintenanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_cluster_maintenance::builders::ModifyClusterMaintenanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceOutput,
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceError,
    > for ModifyClusterMaintenanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceOutput,
            crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyClusterMaintenanceFluentBuilder {
    /// Creates a new `ModifyClusterMaintenance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyClusterMaintenance as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_cluster_maintenance::builders::ModifyClusterMaintenanceInputBuilder {
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
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_cluster_maintenance::ModifyClusterMaintenance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceOutput,
        crate::operation::modify_cluster_maintenance::ModifyClusterMaintenanceError,
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
    /// <p>A unique identifier for the cluster.</p>
    pub fn cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_identifier(input.into());
        self
    }
    /// <p>A unique identifier for the cluster.</p>
    pub fn set_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_identifier(input);
        self
    }
    /// <p>A unique identifier for the cluster.</p>
    pub fn get_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_identifier()
    }
    /// <p>A boolean indicating whether to enable the deferred maintenance window. </p>
    pub fn defer_maintenance(mut self, input: bool) -> Self {
        self.inner = self.inner.defer_maintenance(input);
        self
    }
    /// <p>A boolean indicating whether to enable the deferred maintenance window. </p>
    pub fn set_defer_maintenance(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_defer_maintenance(input);
        self
    }
    /// <p>A boolean indicating whether to enable the deferred maintenance window. </p>
    pub fn get_defer_maintenance(&self) -> &::std::option::Option<bool> {
        self.inner.get_defer_maintenance()
    }
    /// <p>A unique identifier for the deferred maintenance window.</p>
    pub fn defer_maintenance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.defer_maintenance_identifier(input.into());
        self
    }
    /// <p>A unique identifier for the deferred maintenance window.</p>
    pub fn set_defer_maintenance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_defer_maintenance_identifier(input);
        self
    }
    /// <p>A unique identifier for the deferred maintenance window.</p>
    pub fn get_defer_maintenance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_defer_maintenance_identifier()
    }
    /// <p>A timestamp indicating the start time for the deferred maintenance window.</p>
    pub fn defer_maintenance_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.defer_maintenance_start_time(input);
        self
    }
    /// <p>A timestamp indicating the start time for the deferred maintenance window.</p>
    pub fn set_defer_maintenance_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_defer_maintenance_start_time(input);
        self
    }
    /// <p>A timestamp indicating the start time for the deferred maintenance window.</p>
    pub fn get_defer_maintenance_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_defer_maintenance_start_time()
    }
    /// <p>A timestamp indicating end time for the deferred maintenance window. If you specify an end time, you can't specify a duration.</p>
    pub fn defer_maintenance_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.defer_maintenance_end_time(input);
        self
    }
    /// <p>A timestamp indicating end time for the deferred maintenance window. If you specify an end time, you can't specify a duration.</p>
    pub fn set_defer_maintenance_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_defer_maintenance_end_time(input);
        self
    }
    /// <p>A timestamp indicating end time for the deferred maintenance window. If you specify an end time, you can't specify a duration.</p>
    pub fn get_defer_maintenance_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_defer_maintenance_end_time()
    }
    /// <p>An integer indicating the duration of the maintenance window in days. If you specify a duration, you can't specify an end time. The duration must be 45 days or less.</p>
    pub fn defer_maintenance_duration(mut self, input: i32) -> Self {
        self.inner = self.inner.defer_maintenance_duration(input);
        self
    }
    /// <p>An integer indicating the duration of the maintenance window in days. If you specify a duration, you can't specify an end time. The duration must be 45 days or less.</p>
    pub fn set_defer_maintenance_duration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_defer_maintenance_duration(input);
        self
    }
    /// <p>An integer indicating the duration of the maintenance window in days. If you specify a duration, you can't specify an end time. The duration must be 45 days or less.</p>
    pub fn get_defer_maintenance_duration(&self) -> &::std::option::Option<i32> {
        self.inner.get_defer_maintenance_duration()
    }
}
