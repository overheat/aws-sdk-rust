// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_region_settings::_update_region_settings_output::UpdateRegionSettingsOutputBuilder;

pub use crate::operation::update_region_settings::_update_region_settings_input::UpdateRegionSettingsInputBuilder;

impl UpdateRegionSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_region_settings::UpdateRegionSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_region_settings::UpdateRegionSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_region_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRegionSettings`.
///
/// <p>Updates the current service opt-in settings for the Region. If service-opt-in is enabled for a service, Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. Otherwise, Backup does not try to protect that service's resources in this Region. Use the <code>DescribeRegionSettings</code> API to determine the resource types that are supported.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRegionSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_region_settings::builders::UpdateRegionSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_region_settings::UpdateRegionSettingsOutput,
        crate::operation::update_region_settings::UpdateRegionSettingsError,
    > for UpdateRegionSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_region_settings::UpdateRegionSettingsOutput,
            crate::operation::update_region_settings::UpdateRegionSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRegionSettingsFluentBuilder {
    /// Creates a new `UpdateRegionSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRegionSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_region_settings::builders::UpdateRegionSettingsInputBuilder {
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
        crate::operation::update_region_settings::UpdateRegionSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_region_settings::UpdateRegionSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_region_settings::UpdateRegionSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_region_settings::UpdateRegionSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_region_settings::UpdateRegionSettingsOutput,
        crate::operation::update_region_settings::UpdateRegionSettingsError,
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
    /// Adds a key-value pair to `ResourceTypeOptInPreference`.
    ///
    /// To override the contents of this collection use [`set_resource_type_opt_in_preference`](Self::set_resource_type_opt_in_preference).
    ///
    /// <p>Updates the list of services along with the opt-in preferences for the Region.</p>
    pub fn resource_type_opt_in_preference(mut self, k: impl ::std::convert::Into<::std::string::String>, v: bool) -> Self {
        self.inner = self.inner.resource_type_opt_in_preference(k.into(), v);
        self
    }
    /// <p>Updates the list of services along with the opt-in preferences for the Region.</p>
    pub fn set_resource_type_opt_in_preference(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, bool>>,
    ) -> Self {
        self.inner = self.inner.set_resource_type_opt_in_preference(input);
        self
    }
    /// <p>Updates the list of services along with the opt-in preferences for the Region.</p>
    pub fn get_resource_type_opt_in_preference(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, bool>> {
        self.inner.get_resource_type_opt_in_preference()
    }
    /// Adds a key-value pair to `ResourceTypeManagementPreference`.
    ///
    /// To override the contents of this collection use [`set_resource_type_management_preference`](Self::set_resource_type_management_preference).
    ///
    /// <p>Enables or disables full Backup management of backups for a resource type. To enable full Backup management for DynamoDB along with <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html"> Backup's advanced DynamoDB backup features</a>, follow the procedure to <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html#advanced-ddb-backup-enable-cli"> enable advanced DynamoDB backup programmatically</a>.</p>
    pub fn resource_type_management_preference(mut self, k: impl ::std::convert::Into<::std::string::String>, v: bool) -> Self {
        self.inner = self.inner.resource_type_management_preference(k.into(), v);
        self
    }
    /// <p>Enables or disables full Backup management of backups for a resource type. To enable full Backup management for DynamoDB along with <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html"> Backup's advanced DynamoDB backup features</a>, follow the procedure to <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html#advanced-ddb-backup-enable-cli"> enable advanced DynamoDB backup programmatically</a>.</p>
    pub fn set_resource_type_management_preference(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, bool>>,
    ) -> Self {
        self.inner = self.inner.set_resource_type_management_preference(input);
        self
    }
    /// <p>Enables or disables full Backup management of backups for a resource type. To enable full Backup management for DynamoDB along with <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html"> Backup's advanced DynamoDB backup features</a>, follow the procedure to <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html#advanced-ddb-backup-enable-cli"> enable advanced DynamoDB backup programmatically</a>.</p>
    pub fn get_resource_type_management_preference(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, bool>> {
        self.inner.get_resource_type_management_preference()
    }
}
