// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::increase_replica_count::_increase_replica_count_output::IncreaseReplicaCountOutputBuilder;

pub use crate::operation::increase_replica_count::_increase_replica_count_input::IncreaseReplicaCountInputBuilder;

impl IncreaseReplicaCountInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::increase_replica_count::IncreaseReplicaCountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::increase_replica_count::IncreaseReplicaCountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.increase_replica_count();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `IncreaseReplicaCount`.
///
/// <p>Dynamically increases the number of replicas in a Redis (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Redis (cluster mode enabled) replication group. This operation is performed with no cluster down time.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct IncreaseReplicaCountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::increase_replica_count::builders::IncreaseReplicaCountInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::increase_replica_count::IncreaseReplicaCountOutput,
        crate::operation::increase_replica_count::IncreaseReplicaCountError,
    > for IncreaseReplicaCountFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::increase_replica_count::IncreaseReplicaCountOutput,
            crate::operation::increase_replica_count::IncreaseReplicaCountError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl IncreaseReplicaCountFluentBuilder {
    /// Creates a new `IncreaseReplicaCount`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the IncreaseReplicaCount as a reference.
    pub fn as_input(&self) -> &crate::operation::increase_replica_count::builders::IncreaseReplicaCountInputBuilder {
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
        crate::operation::increase_replica_count::IncreaseReplicaCountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::increase_replica_count::IncreaseReplicaCountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::increase_replica_count::IncreaseReplicaCount::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::increase_replica_count::IncreaseReplicaCount::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::increase_replica_count::IncreaseReplicaCountOutput,
        crate::operation::increase_replica_count::IncreaseReplicaCountError,
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
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn replication_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_group_id(input.into());
        self
    }
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn set_replication_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_group_id(input);
        self
    }
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub fn get_replication_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_group_id()
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn new_replica_count(mut self, input: i32) -> Self {
        self.inner = self.inner.new_replica_count(input);
        self
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn set_new_replica_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_new_replica_count(input);
        self
    }
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub fn get_new_replica_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_new_replica_count()
    }
    /// Appends an item to `ReplicaConfiguration`.
    ///
    /// To override the contents of this collection use [`set_replica_configuration`](Self::set_replica_configuration).
    ///
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn replica_configuration(mut self, input: crate::types::ConfigureShard) -> Self {
        self.inner = self.inner.replica_configuration(input);
        self
    }
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn set_replica_configuration(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigureShard>>) -> Self {
        self.inner = self.inner.set_replica_configuration(input);
        self
    }
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub fn get_replica_configuration(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConfigureShard>> {
        self.inner.get_replica_configuration()
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub fn get_apply_immediately(&self) -> &::std::option::Option<bool> {
        self.inner.get_apply_immediately()
    }
}
