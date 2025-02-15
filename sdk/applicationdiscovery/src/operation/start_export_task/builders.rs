// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_export_task::_start_export_task_output::StartExportTaskOutputBuilder;

pub use crate::operation::start_export_task::_start_export_task_input::StartExportTaskInputBuilder;

impl StartExportTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_export_task::StartExportTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_export_task::StartExportTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_export_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartExportTask`.
///
/// <p>Begins the export of a discovered data report to an Amazon S3 bucket managed by Amazon Web Services.</p> <note>
/// <p>Exports might provide an estimate of fees and savings based on certain information that you provide. Fee estimates do not include any taxes that might apply. Your actual fees and savings depend on a variety of factors, including your actual usage of Amazon Web Services services, which might vary from the estimates provided in this report.</p>
/// </note>
/// <p>If you do not specify <code>preferences</code> or <code>agentIds</code> in the filter, a summary of all servers, applications, tags, and performance is generated. This data is an aggregation of all server data collected through on-premises tooling, file import, application grouping and applying tags.</p>
/// <p>If you specify <code>agentIds</code> in a filter, the task exports up to 72 hours of detailed data collected by the identified Application Discovery Agent, including network, process, and performance details. A time range for exported agent data may be set by using <code>startTime</code> and <code>endTime</code>. Export of detailed agent data is limited to five concurrently running exports. Export of detailed agent data is limited to two exports per day.</p>
/// <p>If you enable <code>ec2RecommendationsPreferences</code> in <code>preferences</code> , an Amazon EC2 instance matching the characteristics of each server in Application Discovery Service is generated. Changing the attributes of the <code>ec2RecommendationsPreferences</code> changes the criteria of the recommendation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartExportTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_export_task::builders::StartExportTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_export_task::StartExportTaskOutput,
        crate::operation::start_export_task::StartExportTaskError,
    > for StartExportTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_export_task::StartExportTaskOutput,
            crate::operation::start_export_task::StartExportTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartExportTaskFluentBuilder {
    /// Creates a new `StartExportTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartExportTask as a reference.
    pub fn as_input(&self) -> &crate::operation::start_export_task::builders::StartExportTaskInputBuilder {
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
        crate::operation::start_export_task::StartExportTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_export_task::StartExportTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_export_task::StartExportTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_export_task::StartExportTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_export_task::StartExportTaskOutput,
        crate::operation::start_export_task::StartExportTaskError,
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
    /// Appends an item to `exportDataFormat`.
    ///
    /// To override the contents of this collection use [`set_export_data_format`](Self::set_export_data_format).
    ///
    /// <p>The file format for the returned export data. Default value is <code>CSV</code>. <b>Note:</b> <i>The</i> <code>GRAPHML</code> <i>option has been deprecated.</i> </p>
    pub fn export_data_format(mut self, input: crate::types::ExportDataFormat) -> Self {
        self.inner = self.inner.export_data_format(input);
        self
    }
    /// <p>The file format for the returned export data. Default value is <code>CSV</code>. <b>Note:</b> <i>The</i> <code>GRAPHML</code> <i>option has been deprecated.</i> </p>
    pub fn set_export_data_format(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ExportDataFormat>>) -> Self {
        self.inner = self.inner.set_export_data_format(input);
        self
    }
    /// <p>The file format for the returned export data. Default value is <code>CSV</code>. <b>Note:</b> <i>The</i> <code>GRAPHML</code> <i>option has been deprecated.</i> </p>
    pub fn get_export_data_format(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ExportDataFormat>> {
        self.inner.get_export_data_format()
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>If a filter is present, it selects the single <code>agentId</code> of the Application Discovery Agent for which data is exported. The <code>agentId</code> can be found in the results of the <code>DescribeAgents</code> API or CLI. If no filter is present, <code>startTime</code> and <code>endTime</code> are ignored and exported data includes both Amazon Web Services Application Discovery Service Agentless Collector collectors data and summary data from Application Discovery Agent agents. </p>
    pub fn filters(mut self, input: crate::types::ExportFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>If a filter is present, it selects the single <code>agentId</code> of the Application Discovery Agent for which data is exported. The <code>agentId</code> can be found in the results of the <code>DescribeAgents</code> API or CLI. If no filter is present, <code>startTime</code> and <code>endTime</code> are ignored and exported data includes both Amazon Web Services Application Discovery Service Agentless Collector collectors data and summary data from Application Discovery Agent agents. </p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ExportFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>If a filter is present, it selects the single <code>agentId</code> of the Application Discovery Agent for which data is exported. The <code>agentId</code> can be found in the results of the <code>DescribeAgents</code> API or CLI. If no filter is present, <code>startTime</code> and <code>endTime</code> are ignored and exported data includes both Amazon Web Services Application Discovery Service Agentless Collector collectors data and summary data from Application Discovery Agent agents. </p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ExportFilter>> {
        self.inner.get_filters()
    }
    /// <p>The start timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, data is exported starting from the first data collected by the agent.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, data is exported starting from the first data collected by the agent.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The start timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, data is exported starting from the first data collected by the agent.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The end timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, exported data includes the most recent data collected by the agent.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, exported data includes the most recent data collected by the agent.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The end timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, exported data includes the most recent data collected by the agent.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p> Indicates the type of data that needs to be exported. Only one <a href="https://docs.aws.amazon.com/application-discovery/latest/APIReference/API_ExportPreferences.html">ExportPreferences</a> can be enabled at any time. </p>
    pub fn preferences(mut self, input: crate::types::ExportPreferences) -> Self {
        self.inner = self.inner.preferences(input);
        self
    }
    /// <p> Indicates the type of data that needs to be exported. Only one <a href="https://docs.aws.amazon.com/application-discovery/latest/APIReference/API_ExportPreferences.html">ExportPreferences</a> can be enabled at any time. </p>
    pub fn set_preferences(mut self, input: ::std::option::Option<crate::types::ExportPreferences>) -> Self {
        self.inner = self.inner.set_preferences(input);
        self
    }
    /// <p> Indicates the type of data that needs to be exported. Only one <a href="https://docs.aws.amazon.com/application-discovery/latest/APIReference/API_ExportPreferences.html">ExportPreferences</a> can be enabled at any time. </p>
    pub fn get_preferences(&self) -> &::std::option::Option<crate::types::ExportPreferences> {
        self.inner.get_preferences()
    }
}
