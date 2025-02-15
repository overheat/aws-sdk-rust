// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_warm_pool::_put_warm_pool_output::PutWarmPoolOutputBuilder;

pub use crate::operation::put_warm_pool::_put_warm_pool_input::PutWarmPoolInputBuilder;

impl PutWarmPoolInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_warm_pool::PutWarmPoolOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_warm_pool::PutWarmPoolError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_warm_pool();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutWarmPool`.
///
/// <p>Creates or updates a warm pool for the specified Auto Scaling group. A warm pool is a pool of pre-initialized EC2 instances that sits alongside the Auto Scaling group. Whenever your application needs to scale out, the Auto Scaling group can draw on the warm pool to meet its new desired capacity. For more information and example configurations, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-warm-pools.html">Warm pools for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
/// <p>This operation must be called from the Region in which the Auto Scaling group was created. This operation cannot be called on an Auto Scaling group that has a mixed instances policy or a launch template or launch configuration that requests Spot Instances.</p>
/// <p>You can view the instances in the warm pool using the <code>DescribeWarmPool</code> API call. If you are no longer using a warm pool, you can delete it by calling the <code>DeleteWarmPool</code> API.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutWarmPoolFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_warm_pool::builders::PutWarmPoolInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_warm_pool::PutWarmPoolOutput,
        crate::operation::put_warm_pool::PutWarmPoolError,
    > for PutWarmPoolFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_warm_pool::PutWarmPoolOutput,
            crate::operation::put_warm_pool::PutWarmPoolError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutWarmPoolFluentBuilder {
    /// Creates a new `PutWarmPool`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutWarmPool as a reference.
    pub fn as_input(&self) -> &crate::operation::put_warm_pool::builders::PutWarmPoolInputBuilder {
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
        crate::operation::put_warm_pool::PutWarmPoolOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_warm_pool::PutWarmPoolError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_warm_pool::PutWarmPool::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_warm_pool::PutWarmPool::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_warm_pool::PutWarmPoolOutput,
        crate::operation::put_warm_pool::PutWarmPoolError,
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
    /// <p>The name of the Auto Scaling group.</p>
    pub fn auto_scaling_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.auto_scaling_group_name(input.into());
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn set_auto_scaling_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_auto_scaling_group_name(input);
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn get_auto_scaling_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_auto_scaling_group_name()
    }
    /// <p>Specifies the maximum number of instances that are allowed to be in the warm pool or in any state except <code>Terminated</code> for the Auto Scaling group. This is an optional property. Specify it only if you do not want the warm pool size to be determined by the difference between the group's maximum capacity and its desired capacity. </p> <important>
    /// <p>If a value for <code>MaxGroupPreparedCapacity</code> is not specified, Amazon EC2 Auto Scaling launches and maintains the difference between the group's maximum capacity and its desired capacity. If you specify a value for <code>MaxGroupPreparedCapacity</code>, Amazon EC2 Auto Scaling uses the difference between the <code>MaxGroupPreparedCapacity</code> and the desired capacity instead. </p>
    /// <p>The size of the warm pool is dynamic. Only when <code>MaxGroupPreparedCapacity</code> and <code>MinSize</code> are set to the same value does the warm pool have an absolute size.</p>
    /// </important>
    /// <p>If the desired capacity of the Auto Scaling group is higher than the <code>MaxGroupPreparedCapacity</code>, the capacity of the warm pool is 0, unless you specify a value for <code>MinSize</code>. To remove a value that you previously set, include the property but specify -1 for the value. </p>
    pub fn max_group_prepared_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.max_group_prepared_capacity(input);
        self
    }
    /// <p>Specifies the maximum number of instances that are allowed to be in the warm pool or in any state except <code>Terminated</code> for the Auto Scaling group. This is an optional property. Specify it only if you do not want the warm pool size to be determined by the difference between the group's maximum capacity and its desired capacity. </p> <important>
    /// <p>If a value for <code>MaxGroupPreparedCapacity</code> is not specified, Amazon EC2 Auto Scaling launches and maintains the difference between the group's maximum capacity and its desired capacity. If you specify a value for <code>MaxGroupPreparedCapacity</code>, Amazon EC2 Auto Scaling uses the difference between the <code>MaxGroupPreparedCapacity</code> and the desired capacity instead. </p>
    /// <p>The size of the warm pool is dynamic. Only when <code>MaxGroupPreparedCapacity</code> and <code>MinSize</code> are set to the same value does the warm pool have an absolute size.</p>
    /// </important>
    /// <p>If the desired capacity of the Auto Scaling group is higher than the <code>MaxGroupPreparedCapacity</code>, the capacity of the warm pool is 0, unless you specify a value for <code>MinSize</code>. To remove a value that you previously set, include the property but specify -1 for the value. </p>
    pub fn set_max_group_prepared_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_group_prepared_capacity(input);
        self
    }
    /// <p>Specifies the maximum number of instances that are allowed to be in the warm pool or in any state except <code>Terminated</code> for the Auto Scaling group. This is an optional property. Specify it only if you do not want the warm pool size to be determined by the difference between the group's maximum capacity and its desired capacity. </p> <important>
    /// <p>If a value for <code>MaxGroupPreparedCapacity</code> is not specified, Amazon EC2 Auto Scaling launches and maintains the difference between the group's maximum capacity and its desired capacity. If you specify a value for <code>MaxGroupPreparedCapacity</code>, Amazon EC2 Auto Scaling uses the difference between the <code>MaxGroupPreparedCapacity</code> and the desired capacity instead. </p>
    /// <p>The size of the warm pool is dynamic. Only when <code>MaxGroupPreparedCapacity</code> and <code>MinSize</code> are set to the same value does the warm pool have an absolute size.</p>
    /// </important>
    /// <p>If the desired capacity of the Auto Scaling group is higher than the <code>MaxGroupPreparedCapacity</code>, the capacity of the warm pool is 0, unless you specify a value for <code>MinSize</code>. To remove a value that you previously set, include the property but specify -1 for the value. </p>
    pub fn get_max_group_prepared_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_group_prepared_capacity()
    }
    /// <p>Specifies the minimum number of instances to maintain in the warm pool. This helps you to ensure that there is always a certain number of warmed instances available to handle traffic spikes. Defaults to 0 if not specified.</p>
    pub fn min_size(mut self, input: i32) -> Self {
        self.inner = self.inner.min_size(input);
        self
    }
    /// <p>Specifies the minimum number of instances to maintain in the warm pool. This helps you to ensure that there is always a certain number of warmed instances available to handle traffic spikes. Defaults to 0 if not specified.</p>
    pub fn set_min_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_min_size(input);
        self
    }
    /// <p>Specifies the minimum number of instances to maintain in the warm pool. This helps you to ensure that there is always a certain number of warmed instances available to handle traffic spikes. Defaults to 0 if not specified.</p>
    pub fn get_min_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_min_size()
    }
    /// <p>Sets the instance state to transition to after the lifecycle actions are complete. Default is <code>Stopped</code>.</p>
    pub fn pool_state(mut self, input: crate::types::WarmPoolState) -> Self {
        self.inner = self.inner.pool_state(input);
        self
    }
    /// <p>Sets the instance state to transition to after the lifecycle actions are complete. Default is <code>Stopped</code>.</p>
    pub fn set_pool_state(mut self, input: ::std::option::Option<crate::types::WarmPoolState>) -> Self {
        self.inner = self.inner.set_pool_state(input);
        self
    }
    /// <p>Sets the instance state to transition to after the lifecycle actions are complete. Default is <code>Stopped</code>.</p>
    pub fn get_pool_state(&self) -> &::std::option::Option<crate::types::WarmPoolState> {
        self.inner.get_pool_state()
    }
    /// <p>Indicates whether instances in the Auto Scaling group can be returned to the warm pool on scale in. The default is to terminate instances in the Auto Scaling group when the group scales in.</p>
    pub fn instance_reuse_policy(mut self, input: crate::types::InstanceReusePolicy) -> Self {
        self.inner = self.inner.instance_reuse_policy(input);
        self
    }
    /// <p>Indicates whether instances in the Auto Scaling group can be returned to the warm pool on scale in. The default is to terminate instances in the Auto Scaling group when the group scales in.</p>
    pub fn set_instance_reuse_policy(mut self, input: ::std::option::Option<crate::types::InstanceReusePolicy>) -> Self {
        self.inner = self.inner.set_instance_reuse_policy(input);
        self
    }
    /// <p>Indicates whether instances in the Auto Scaling group can be returned to the warm pool on scale in. The default is to terminate instances in the Auto Scaling group when the group scales in.</p>
    pub fn get_instance_reuse_policy(&self) -> &::std::option::Option<crate::types::InstanceReusePolicy> {
        self.inner.get_instance_reuse_policy()
    }
}
