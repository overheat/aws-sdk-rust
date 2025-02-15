// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cluster::_create_cluster_output::CreateClusterOutputBuilder;

pub use crate::operation::create_cluster::_create_cluster_input::CreateClusterInputBuilder;

impl CreateClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_cluster::CreateClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cluster::CreateClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCluster`.
///
/// <p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p> <note>
/// <p>When you call the <code>CreateCluster</code> API operation, Amazon ECS attempts to create the Amazon ECS service-linked role for your account. This is so that it can manage required resources in other Amazon Web Services services on your behalf. However, if the user that makes the call doesn't have permissions to create the service-linked role, it isn't created. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_cluster::builders::CreateClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_cluster::CreateClusterOutput,
        crate::operation::create_cluster::CreateClusterError,
    > for CreateClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_cluster::CreateClusterOutput,
            crate::operation::create_cluster::CreateClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateClusterFluentBuilder {
    /// Creates a new `CreateCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::create_cluster::builders::CreateClusterInputBuilder {
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
        crate::operation::create_cluster::CreateClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cluster::CreateClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_cluster::CreateCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_cluster::CreateCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_cluster::CreateClusterOutput,
        crate::operation::create_cluster::CreateClusterError,
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
    /// <p>The name of your cluster. If you don't specify a name for your cluster, you create a cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. </p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_name(input.into());
        self
    }
    /// <p>The name of your cluster. If you don't specify a name for your cluster, you create a cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. </p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_name(input);
        self
    }
    /// <p>The name of your cluster. If you don't specify a name for your cluster, you create a cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. </p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_name()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case-sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case-sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case-sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// Appends an item to `settings`.
    ///
    /// To override the contents of this collection use [`set_settings`](Self::set_settings).
    ///
    /// <p>The setting to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p>
    pub fn settings(mut self, input: crate::types::ClusterSetting) -> Self {
        self.inner = self.inner.settings(input);
        self
    }
    /// <p>The setting to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p>
    pub fn set_settings(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ClusterSetting>>) -> Self {
        self.inner = self.inner.set_settings(input);
        self
    }
    /// <p>The setting to use when creating a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p>
    pub fn get_settings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ClusterSetting>> {
        self.inner.get_settings()
    }
    /// <p>The <code>execute</code> command configuration for the cluster.</p>
    pub fn configuration(mut self, input: crate::types::ClusterConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>The <code>execute</code> command configuration for the cluster.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::ClusterConfiguration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>The <code>execute</code> command configuration for the cluster.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::ClusterConfiguration> {
        self.inner.get_configuration()
    }
    /// Appends an item to `capacityProviders`.
    ///
    /// To override the contents of this collection use [`set_capacity_providers`](Self::set_capacity_providers).
    ///
    /// <p>The short name of one or more capacity providers to associate with the cluster. A capacity provider must be associated with a cluster before it can be included as part of the default capacity provider strategy of the cluster or used in a capacity provider strategy when calling the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> actions.</p>
    /// <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must be created but not associated with another cluster. New Auto Scaling group capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
    /// <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>
    /// <p>The <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutCapacityProvider.html">PutCapacityProvider</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    pub fn capacity_providers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.capacity_providers(input.into());
        self
    }
    /// <p>The short name of one or more capacity providers to associate with the cluster. A capacity provider must be associated with a cluster before it can be included as part of the default capacity provider strategy of the cluster or used in a capacity provider strategy when calling the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> actions.</p>
    /// <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must be created but not associated with another cluster. New Auto Scaling group capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
    /// <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>
    /// <p>The <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutCapacityProvider.html">PutCapacityProvider</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    pub fn set_capacity_providers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_capacity_providers(input);
        self
    }
    /// <p>The short name of one or more capacity providers to associate with the cluster. A capacity provider must be associated with a cluster before it can be included as part of the default capacity provider strategy of the cluster or used in a capacity provider strategy when calling the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> actions.</p>
    /// <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity provider must be created but not associated with another cluster. New Auto Scaling group capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
    /// <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or <code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers are available to all accounts and only need to be associated with a cluster to be used.</p>
    /// <p>The <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutCapacityProvider.html">PutCapacityProvider</a> API operation is used to update the list of available capacity providers for a cluster after the cluster is created.</p>
    pub fn get_capacity_providers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_capacity_providers()
    }
    /// Appends an item to `defaultCapacityProviderStrategy`.
    ///
    /// To override the contents of this collection use [`set_default_capacity_provider_strategy`](Self::set_default_capacity_provider_strategy).
    ///
    /// <p>The capacity provider strategy to set as the default for the cluster. After a default capacity provider strategy is set for a cluster, when you call the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> APIs with no capacity provider strategy or launch type specified, the default capacity provider strategy for the cluster is used.</p>
    /// <p>If a default capacity provider strategy isn't defined for a cluster when it was created, it can be defined later with the <code>PutClusterCapacityProviders</code> API operation.</p>
    pub fn default_capacity_provider_strategy(mut self, input: crate::types::CapacityProviderStrategyItem) -> Self {
        self.inner = self.inner.default_capacity_provider_strategy(input);
        self
    }
    /// <p>The capacity provider strategy to set as the default for the cluster. After a default capacity provider strategy is set for a cluster, when you call the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> APIs with no capacity provider strategy or launch type specified, the default capacity provider strategy for the cluster is used.</p>
    /// <p>If a default capacity provider strategy isn't defined for a cluster when it was created, it can be defined later with the <code>PutClusterCapacityProviders</code> API operation.</p>
    pub fn set_default_capacity_provider_strategy(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CapacityProviderStrategyItem>>,
    ) -> Self {
        self.inner = self.inner.set_default_capacity_provider_strategy(input);
        self
    }
    /// <p>The capacity provider strategy to set as the default for the cluster. After a default capacity provider strategy is set for a cluster, when you call the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> APIs with no capacity provider strategy or launch type specified, the default capacity provider strategy for the cluster is used.</p>
    /// <p>If a default capacity provider strategy isn't defined for a cluster when it was created, it can be defined later with the <code>PutClusterCapacityProviders</code> API operation.</p>
    pub fn get_default_capacity_provider_strategy(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CapacityProviderStrategyItem>> {
        self.inner.get_default_capacity_provider_strategy()
    }
    /// <p>Use this parameter to set a default Service Connect namespace. After you set a default Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as client services in the namespace. This setting only applies to new services that set the <code>enabled</code> parameter to <code>true</code> in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each service individually in the <code>ServiceConnectConfiguration</code> to override this default parameter.</p>
    /// <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn service_connect_defaults(mut self, input: crate::types::ClusterServiceConnectDefaultsRequest) -> Self {
        self.inner = self.inner.service_connect_defaults(input);
        self
    }
    /// <p>Use this parameter to set a default Service Connect namespace. After you set a default Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as client services in the namespace. This setting only applies to new services that set the <code>enabled</code> parameter to <code>true</code> in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each service individually in the <code>ServiceConnectConfiguration</code> to override this default parameter.</p>
    /// <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn set_service_connect_defaults(mut self, input: ::std::option::Option<crate::types::ClusterServiceConnectDefaultsRequest>) -> Self {
        self.inner = self.inner.set_service_connect_defaults(input);
        self
    }
    /// <p>Use this parameter to set a default Service Connect namespace. After you set a default Service Connect namespace, any new services with Service Connect turned on that are created in the cluster are added as client services in the namespace. This setting only applies to new services that set the <code>enabled</code> parameter to <code>true</code> in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each service individually in the <code>ServiceConnectConfiguration</code> to override this default parameter.</p>
    /// <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn get_service_connect_defaults(&self) -> &::std::option::Option<crate::types::ClusterServiceConnectDefaultsRequest> {
        self.inner.get_service_connect_defaults()
    }
}
