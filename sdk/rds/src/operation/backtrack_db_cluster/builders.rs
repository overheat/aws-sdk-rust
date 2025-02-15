// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::backtrack_db_cluster::_backtrack_db_cluster_output::BacktrackDbClusterOutputBuilder;

pub use crate::operation::backtrack_db_cluster::_backtrack_db_cluster_input::BacktrackDbClusterInputBuilder;

impl BacktrackDbClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::backtrack_db_cluster::BacktrackDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::backtrack_db_cluster::BacktrackDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.backtrack_db_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BacktrackDBCluster`.
///
/// <p>Backtracks a DB cluster to a specific time, without creating a new DB cluster.</p>
/// <p>For more information on backtracking, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Managing.Backtrack.html"> Backtracking an Aurora DB Cluster</a> in the <i>Amazon Aurora User Guide</i>.</p> <note>
/// <p>This action applies only to Aurora MySQL DB clusters.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BacktrackDBClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::backtrack_db_cluster::builders::BacktrackDbClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::backtrack_db_cluster::BacktrackDbClusterOutput,
        crate::operation::backtrack_db_cluster::BacktrackDBClusterError,
    > for BacktrackDBClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::backtrack_db_cluster::BacktrackDbClusterOutput,
            crate::operation::backtrack_db_cluster::BacktrackDBClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BacktrackDBClusterFluentBuilder {
    /// Creates a new `BacktrackDBCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BacktrackDBCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::backtrack_db_cluster::builders::BacktrackDbClusterInputBuilder {
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
        crate::operation::backtrack_db_cluster::BacktrackDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::backtrack_db_cluster::BacktrackDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::backtrack_db_cluster::BacktrackDBCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::backtrack_db_cluster::BacktrackDBCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::backtrack_db_cluster::BacktrackDbClusterOutput,
        crate::operation::backtrack_db_cluster::BacktrackDBClusterError,
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
    /// <p>The DB cluster identifier of the DB cluster to be backtracked. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens.</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster1</code> </p>
    pub fn db_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_identifier(input.into());
        self
    }
    /// <p>The DB cluster identifier of the DB cluster to be backtracked. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens.</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster1</code> </p>
    pub fn set_db_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_cluster_identifier(input);
        self
    }
    /// <p>The DB cluster identifier of the DB cluster to be backtracked. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens.</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster1</code> </p>
    pub fn get_db_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_cluster_identifier()
    }
    /// <p>The timestamp of the time to backtrack the DB cluster to, specified in ISO 8601 format. For more information about ISO 8601, see the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <note>
    /// <p>If the specified time isn't a consistent time for the DB cluster, Aurora automatically chooses the nearest possible consistent time for the DB cluster.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain a valid ISO 8601 timestamp.</p> </li>
    /// <li> <p>Can't contain a timestamp set in the future.</p> </li>
    /// </ul>
    /// <p>Example: <code>2017-07-08T18:00Z</code> </p>
    pub fn backtrack_to(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.backtrack_to(input);
        self
    }
    /// <p>The timestamp of the time to backtrack the DB cluster to, specified in ISO 8601 format. For more information about ISO 8601, see the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <note>
    /// <p>If the specified time isn't a consistent time for the DB cluster, Aurora automatically chooses the nearest possible consistent time for the DB cluster.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain a valid ISO 8601 timestamp.</p> </li>
    /// <li> <p>Can't contain a timestamp set in the future.</p> </li>
    /// </ul>
    /// <p>Example: <code>2017-07-08T18:00Z</code> </p>
    pub fn set_backtrack_to(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_backtrack_to(input);
        self
    }
    /// <p>The timestamp of the time to backtrack the DB cluster to, specified in ISO 8601 format. For more information about ISO 8601, see the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p> <note>
    /// <p>If the specified time isn't a consistent time for the DB cluster, Aurora automatically chooses the nearest possible consistent time for the DB cluster.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain a valid ISO 8601 timestamp.</p> </li>
    /// <li> <p>Can't contain a timestamp set in the future.</p> </li>
    /// </ul>
    /// <p>Example: <code>2017-07-08T18:00Z</code> </p>
    pub fn get_backtrack_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_backtrack_to()
    }
    /// <p>Specifies whether to force the DB cluster to backtrack when binary logging is enabled. Otherwise, an error occurs when binary logging is enabled.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>Specifies whether to force the DB cluster to backtrack when binary logging is enabled. Otherwise, an error occurs when binary logging is enabled.</p>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>Specifies whether to force the DB cluster to backtrack when binary logging is enabled. Otherwise, an error occurs when binary logging is enabled.</p>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        self.inner.get_force()
    }
    /// <p>Specifies whether to backtrack the DB cluster to the earliest possible backtrack time when <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time. When this parameter is disabled and <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time, an error occurs.</p>
    pub fn use_earliest_time_on_point_in_time_unavailable(mut self, input: bool) -> Self {
        self.inner = self.inner.use_earliest_time_on_point_in_time_unavailable(input);
        self
    }
    /// <p>Specifies whether to backtrack the DB cluster to the earliest possible backtrack time when <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time. When this parameter is disabled and <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time, an error occurs.</p>
    pub fn set_use_earliest_time_on_point_in_time_unavailable(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_earliest_time_on_point_in_time_unavailable(input);
        self
    }
    /// <p>Specifies whether to backtrack the DB cluster to the earliest possible backtrack time when <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time. When this parameter is disabled and <i>BacktrackTo</i> is set to a timestamp earlier than the earliest backtrack time, an error occurs.</p>
    pub fn get_use_earliest_time_on_point_in_time_unavailable(&self) -> &::std::option::Option<bool> {
        self.inner.get_use_earliest_time_on_point_in_time_unavailable()
    }
}
