// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_elastic_load_balancers::_describe_elastic_load_balancers_output::DescribeElasticLoadBalancersOutputBuilder;

pub use crate::operation::describe_elastic_load_balancers::_describe_elastic_load_balancers_input::DescribeElasticLoadBalancersInputBuilder;

impl DescribeElasticLoadBalancersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_elastic_load_balancers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeElasticLoadBalancers`.
///
/// <p>Describes a stack's Elastic Load Balancing instances.</p> <note>
/// <p>This call accepts only one resource-identifying parameter.</p>
/// </note>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeElasticLoadBalancersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_elastic_load_balancers::builders::DescribeElasticLoadBalancersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersOutput,
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersError,
    > for DescribeElasticLoadBalancersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersOutput,
            crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeElasticLoadBalancersFluentBuilder {
    /// Creates a new `DescribeElasticLoadBalancers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeElasticLoadBalancers as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_elastic_load_balancers::builders::DescribeElasticLoadBalancersInputBuilder {
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
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersOutput,
        crate::operation::describe_elastic_load_balancers::DescribeElasticLoadBalancersError,
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
    /// <p>A stack ID. The action describes the stack's Elastic Load Balancing instances.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_id(input.into());
        self
    }
    /// <p>A stack ID. The action describes the stack's Elastic Load Balancing instances.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_id(input);
        self
    }
    /// <p>A stack ID. The action describes the stack's Elastic Load Balancing instances.</p>
    pub fn get_stack_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stack_id()
    }
    /// Appends an item to `LayerIds`.
    ///
    /// To override the contents of this collection use [`set_layer_ids`](Self::set_layer_ids).
    ///
    /// <p>A list of layer IDs. The action describes the Elastic Load Balancing instances for the specified layers.</p>
    pub fn layer_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.layer_ids(input.into());
        self
    }
    /// <p>A list of layer IDs. The action describes the Elastic Load Balancing instances for the specified layers.</p>
    pub fn set_layer_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_layer_ids(input);
        self
    }
    /// <p>A list of layer IDs. The action describes the Elastic Load Balancing instances for the specified layers.</p>
    pub fn get_layer_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_layer_ids()
    }
}
