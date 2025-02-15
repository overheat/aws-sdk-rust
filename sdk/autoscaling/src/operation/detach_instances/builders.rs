// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detach_instances::_detach_instances_output::DetachInstancesOutputBuilder;

pub use crate::operation::detach_instances::_detach_instances_input::DetachInstancesInputBuilder;

impl DetachInstancesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::detach_instances::DetachInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detach_instances::DetachInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.detach_instances();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DetachInstances`.
///
/// <p>Removes one or more instances from the specified Auto Scaling group.</p>
/// <p>After the instances are detached, you can manage them independent of the Auto Scaling group.</p>
/// <p>If you do not specify the option to decrement the desired capacity, Amazon EC2 Auto Scaling launches instances to replace the ones that are detached.</p>
/// <p>If there is a Classic Load Balancer attached to the Auto Scaling group, the instances are deregistered from the load balancer. If there are target groups attached to the Auto Scaling group, the instances are deregistered from the target groups.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/detach-instance-asg.html">Detach EC2 instances from your Auto Scaling group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetachInstancesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detach_instances::builders::DetachInstancesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::detach_instances::DetachInstancesOutput,
        crate::operation::detach_instances::DetachInstancesError,
    > for DetachInstancesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::detach_instances::DetachInstancesOutput,
            crate::operation::detach_instances::DetachInstancesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DetachInstancesFluentBuilder {
    /// Creates a new `DetachInstances`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DetachInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::detach_instances::builders::DetachInstancesInputBuilder {
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
        crate::operation::detach_instances::DetachInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detach_instances::DetachInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::detach_instances::DetachInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::detach_instances::DetachInstances::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::detach_instances::DetachInstancesOutput,
        crate::operation::detach_instances::DetachInstancesError,
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
    /// Appends an item to `InstanceIds`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub fn instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_ids(input.into());
        self
    }
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub fn set_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_instance_ids(input);
        self
    }
    /// <p>The IDs of the instances. You can specify up to 20 instances.</p>
    pub fn get_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_instance_ids()
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
    /// <p>Indicates whether the Auto Scaling group decrements the desired capacity value by the number of instances detached.</p>
    pub fn should_decrement_desired_capacity(mut self, input: bool) -> Self {
        self.inner = self.inner.should_decrement_desired_capacity(input);
        self
    }
    /// <p>Indicates whether the Auto Scaling group decrements the desired capacity value by the number of instances detached.</p>
    pub fn set_should_decrement_desired_capacity(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_should_decrement_desired_capacity(input);
        self
    }
    /// <p>Indicates whether the Auto Scaling group decrements the desired capacity value by the number of instances detached.</p>
    pub fn get_should_decrement_desired_capacity(&self) -> &::std::option::Option<bool> {
        self.inner.get_should_decrement_desired_capacity()
    }
}
