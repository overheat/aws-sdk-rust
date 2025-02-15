// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dataset::_create_dataset_output::CreateDatasetOutputBuilder;

pub use crate::operation::create_dataset::_create_dataset_input::CreateDatasetInputBuilder;

impl CreateDatasetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_dataset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDataset`.
///
/// <p>Used to create a dataset. A dataset stores data retrieved from a data store by applying a <code>queryAction</code> (a SQL query) or a <code>containerAction</code> (executing a containerized application). This operation creates the skeleton of a dataset. The dataset can be populated manually by calling <code>CreateDatasetContent</code> or automatically according to a trigger you specify.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatasetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dataset::builders::CreateDatasetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
    > for CreateDatasetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_dataset::CreateDatasetOutput,
            crate::operation::create_dataset::CreateDatasetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDatasetFluentBuilder {
    /// Creates a new `CreateDataset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDataset as a reference.
    pub fn as_input(&self) -> &crate::operation::create_dataset::builders::CreateDatasetInputBuilder {
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
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dataset::CreateDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_dataset::CreateDataset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_dataset::CreateDataset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_dataset::CreateDatasetOutput,
        crate::operation::create_dataset::CreateDatasetError,
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
    /// <p>The name of the dataset.</p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_name(input.into());
        self
    }
    /// <p>The name of the dataset.</p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_name(input);
        self
    }
    /// <p>The name of the dataset.</p>
    pub fn get_dataset_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_name()
    }
    /// Appends an item to `actions`.
    ///
    /// To override the contents of this collection use [`set_actions`](Self::set_actions).
    ///
    /// <p>A list of actions that create the dataset contents.</p>
    pub fn actions(mut self, input: crate::types::DatasetAction) -> Self {
        self.inner = self.inner.actions(input);
        self
    }
    /// <p>A list of actions that create the dataset contents.</p>
    pub fn set_actions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DatasetAction>>) -> Self {
        self.inner = self.inner.set_actions(input);
        self
    }
    /// <p>A list of actions that create the dataset contents.</p>
    pub fn get_actions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DatasetAction>> {
        self.inner.get_actions()
    }
    /// Appends an item to `triggers`.
    ///
    /// To override the contents of this collection use [`set_triggers`](Self::set_triggers).
    ///
    /// <p>A list of triggers. A trigger causes dataset contents to be populated at a specified time interval or when another dataset's contents are created. The list of triggers can be empty or contain up to five <code>DataSetTrigger</code> objects.</p>
    pub fn triggers(mut self, input: crate::types::DatasetTrigger) -> Self {
        self.inner = self.inner.triggers(input);
        self
    }
    /// <p>A list of triggers. A trigger causes dataset contents to be populated at a specified time interval or when another dataset's contents are created. The list of triggers can be empty or contain up to five <code>DataSetTrigger</code> objects.</p>
    pub fn set_triggers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DatasetTrigger>>) -> Self {
        self.inner = self.inner.set_triggers(input);
        self
    }
    /// <p>A list of triggers. A trigger causes dataset contents to be populated at a specified time interval or when another dataset's contents are created. The list of triggers can be empty or contain up to five <code>DataSetTrigger</code> objects.</p>
    pub fn get_triggers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DatasetTrigger>> {
        self.inner.get_triggers()
    }
    /// Appends an item to `contentDeliveryRules`.
    ///
    /// To override the contents of this collection use [`set_content_delivery_rules`](Self::set_content_delivery_rules).
    ///
    /// <p>When dataset contents are created, they are delivered to destinations specified here.</p>
    pub fn content_delivery_rules(mut self, input: crate::types::DatasetContentDeliveryRule) -> Self {
        self.inner = self.inner.content_delivery_rules(input);
        self
    }
    /// <p>When dataset contents are created, they are delivered to destinations specified here.</p>
    pub fn set_content_delivery_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DatasetContentDeliveryRule>>) -> Self {
        self.inner = self.inner.set_content_delivery_rules(input);
        self
    }
    /// <p>When dataset contents are created, they are delivered to destinations specified here.</p>
    pub fn get_content_delivery_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DatasetContentDeliveryRule>> {
        self.inner.get_content_delivery_rules()
    }
    /// <p>Optional. How long, in days, versions of dataset contents are kept for the dataset. If not specified or set to <code>null</code>, versions of dataset contents are retained for at most 90 days. The number of versions of dataset contents retained is determined by the <code>versioningConfiguration</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions"> Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn retention_period(mut self, input: crate::types::RetentionPeriod) -> Self {
        self.inner = self.inner.retention_period(input);
        self
    }
    /// <p>Optional. How long, in days, versions of dataset contents are kept for the dataset. If not specified or set to <code>null</code>, versions of dataset contents are retained for at most 90 days. The number of versions of dataset contents retained is determined by the <code>versioningConfiguration</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions"> Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn set_retention_period(mut self, input: ::std::option::Option<crate::types::RetentionPeriod>) -> Self {
        self.inner = self.inner.set_retention_period(input);
        self
    }
    /// <p>Optional. How long, in days, versions of dataset contents are kept for the dataset. If not specified or set to <code>null</code>, versions of dataset contents are retained for at most 90 days. The number of versions of dataset contents retained is determined by the <code>versioningConfiguration</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions"> Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn get_retention_period(&self) -> &::std::option::Option<crate::types::RetentionPeriod> {
        self.inner.get_retention_period()
    }
    /// <p>Optional. How many versions of dataset contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the <code>retentionPeriod</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions">Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn versioning_configuration(mut self, input: crate::types::VersioningConfiguration) -> Self {
        self.inner = self.inner.versioning_configuration(input);
        self
    }
    /// <p>Optional. How many versions of dataset contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the <code>retentionPeriod</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions">Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn set_versioning_configuration(mut self, input: ::std::option::Option<crate::types::VersioningConfiguration>) -> Self {
        self.inner = self.inner.set_versioning_configuration(input);
        self
    }
    /// <p>Optional. How many versions of dataset contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the <code>retentionPeriod</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions">Keeping Multiple Versions of IoT Analytics datasets</a> in the <i>IoT Analytics User Guide</i>.</p>
    pub fn get_versioning_configuration(&self) -> &::std::option::Option<crate::types::VersioningConfiguration> {
        self.inner.get_versioning_configuration()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata which can be used to manage the dataset.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the dataset.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the dataset.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// Appends an item to `lateDataRules`.
    ///
    /// To override the contents of this collection use [`set_late_data_rules`](Self::set_late_data_rules).
    ///
    /// <p>A list of data rules that send notifications to CloudWatch, when data arrives late. To specify <code>lateDataRules</code>, the dataset must use a <a href="https://docs.aws.amazon.com/iotanalytics/latest/APIReference/API_DeltaTime.html">DeltaTimer</a> filter.</p>
    pub fn late_data_rules(mut self, input: crate::types::LateDataRule) -> Self {
        self.inner = self.inner.late_data_rules(input);
        self
    }
    /// <p>A list of data rules that send notifications to CloudWatch, when data arrives late. To specify <code>lateDataRules</code>, the dataset must use a <a href="https://docs.aws.amazon.com/iotanalytics/latest/APIReference/API_DeltaTime.html">DeltaTimer</a> filter.</p>
    pub fn set_late_data_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LateDataRule>>) -> Self {
        self.inner = self.inner.set_late_data_rules(input);
        self
    }
    /// <p>A list of data rules that send notifications to CloudWatch, when data arrives late. To specify <code>lateDataRules</code>, the dataset must use a <a href="https://docs.aws.amazon.com/iotanalytics/latest/APIReference/API_DeltaTime.html">DeltaTimer</a> filter.</p>
    pub fn get_late_data_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LateDataRule>> {
        self.inner.get_late_data_rules()
    }
}
