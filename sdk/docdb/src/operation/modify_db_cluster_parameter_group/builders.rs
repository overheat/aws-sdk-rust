// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_db_cluster_parameter_group::_modify_db_cluster_parameter_group_output::ModifyDbClusterParameterGroupOutputBuilder;

pub use crate::operation::modify_db_cluster_parameter_group::_modify_db_cluster_parameter_group_input::ModifyDbClusterParameterGroupInputBuilder;

impl ModifyDbClusterParameterGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_db_cluster_parameter_group::ModifyDbClusterParameterGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_db_cluster_parameter_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyDBClusterParameterGroup`.
///
/// <p> Modifies the parameters of a cluster parameter group. To modify more than one parameter, submit a list of the following: <code>ParameterName</code>, <code>ParameterValue</code>, and <code>ApplyMethod</code>. A maximum of 20 parameters can be modified in a single request. </p> <note>
/// <p>Changes to dynamic parameters are applied immediately. Changes to static parameters require a reboot or maintenance window before the change can take effect.</p>
/// </note> <important>
/// <p>After you create a cluster parameter group, you should wait at least 5 minutes before creating your first cluster that uses that cluster parameter group as the default parameter group. This allows Amazon DocumentDB to fully complete the create action before the parameter group is used as the default for a new cluster. This step is especially important for parameters that are critical when creating the default database for a cluster, such as the character set for the default database defined by the <code>character_set_database</code> parameter.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyDBClusterParameterGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_db_cluster_parameter_group::builders::ModifyDbClusterParameterGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_db_cluster_parameter_group::ModifyDbClusterParameterGroupOutput,
        crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroupError,
    > for ModifyDBClusterParameterGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_db_cluster_parameter_group::ModifyDbClusterParameterGroupOutput,
            crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyDBClusterParameterGroupFluentBuilder {
    /// Creates a new `ModifyDBClusterParameterGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyDBClusterParameterGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_db_cluster_parameter_group::builders::ModifyDbClusterParameterGroupInputBuilder {
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
        crate::operation::modify_db_cluster_parameter_group::ModifyDbClusterParameterGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_db_cluster_parameter_group::ModifyDbClusterParameterGroupOutput,
        crate::operation::modify_db_cluster_parameter_group::ModifyDBClusterParameterGroupError,
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
    /// <p>The name of the cluster parameter group to modify.</p>
    pub fn db_cluster_parameter_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_parameter_group_name(input.into());
        self
    }
    /// <p>The name of the cluster parameter group to modify.</p>
    pub fn set_db_cluster_parameter_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_cluster_parameter_group_name(input);
        self
    }
    /// <p>The name of the cluster parameter group to modify.</p>
    pub fn get_db_cluster_parameter_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_cluster_parameter_group_name()
    }
    /// Appends an item to `Parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>A list of parameters in the cluster parameter group to modify.</p>
    pub fn parameters(mut self, input: crate::types::Parameter) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>A list of parameters in the cluster parameter group to modify.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Parameter>>) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>A list of parameters in the cluster parameter group to modify.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Parameter>> {
        self.inner.get_parameters()
    }
}
