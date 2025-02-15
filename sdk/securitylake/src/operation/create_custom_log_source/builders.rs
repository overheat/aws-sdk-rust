// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_custom_log_source::_create_custom_log_source_output::CreateCustomLogSourceOutputBuilder;

pub use crate::operation::create_custom_log_source::_create_custom_log_source_input::CreateCustomLogSourceInputBuilder;

impl CreateCustomLogSourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_custom_log_source::CreateCustomLogSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_custom_log_source::CreateCustomLogSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_custom_log_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCustomLogSource`.
///
/// <p>Adds a third-party custom source in Amazon Security Lake, from the Amazon Web Services Region where you want to create a custom source. Security Lake can collect logs and events from third-party custom sources. After creating the appropriate IAM role to invoke Glue crawler, use this API to add a custom source name in Security Lake. This operation creates a partition in the Amazon S3 bucket for Security Lake as the target location for log files from the custom source. In addition, this operation also creates an associated Glue table and an Glue crawler.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCustomLogSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_custom_log_source::builders::CreateCustomLogSourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_custom_log_source::CreateCustomLogSourceOutput,
        crate::operation::create_custom_log_source::CreateCustomLogSourceError,
    > for CreateCustomLogSourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_custom_log_source::CreateCustomLogSourceOutput,
            crate::operation::create_custom_log_source::CreateCustomLogSourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCustomLogSourceFluentBuilder {
    /// Creates a new `CreateCustomLogSource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCustomLogSource as a reference.
    pub fn as_input(&self) -> &crate::operation::create_custom_log_source::builders::CreateCustomLogSourceInputBuilder {
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
        crate::operation::create_custom_log_source::CreateCustomLogSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_custom_log_source::CreateCustomLogSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_custom_log_source::CreateCustomLogSource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_custom_log_source::CreateCustomLogSource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_custom_log_source::CreateCustomLogSourceOutput,
        crate::operation::create_custom_log_source::CreateCustomLogSourceError,
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
    /// <p>Specify the name for a third-party custom source. This must be a Regionally unique value.</p>
    pub fn source_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_name(input.into());
        self
    }
    /// <p>Specify the name for a third-party custom source. This must be a Regionally unique value.</p>
    pub fn set_source_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_name(input);
        self
    }
    /// <p>Specify the name for a third-party custom source. This must be a Regionally unique value.</p>
    pub fn get_source_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_name()
    }
    /// <p>Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.</p>
    pub fn source_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_version(input.into());
        self
    }
    /// <p>Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.</p>
    pub fn set_source_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_version(input);
        self
    }
    /// <p>Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.</p>
    pub fn get_source_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_version()
    }
    /// Appends an item to `eventClasses`.
    ///
    /// To override the contents of this collection use [`set_event_classes`](Self::set_event_classes).
    ///
    /// <p>The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake. The supported event classes are:</p>
    /// <ul>
    /// <li> <p> <code>ACCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FILE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_EXTENSION</code> </p> </li>
    /// <li> <p> <code>MEMORY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>MODULE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>PROCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_KEY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_VALUE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RESOURCE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SCHEDULED_JOB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SECURITY_FINDING</code> </p> </li>
    /// <li> <p> <code>ACCOUNT_CHANGE</code> </p> </li>
    /// <li> <p> <code>AUTHENTICATION</code> </p> </li>
    /// <li> <p> <code>AUTHORIZATION</code> </p> </li>
    /// <li> <p> <code>ENTITY_MANAGEMENT_AUDIT</code> </p> </li>
    /// <li> <p> <code>DHCP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>NETWORK_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>DNS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>HTTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RDP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SMB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SSH_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CONFIG_STATE</code> </p> </li>
    /// <li> <p> <code>INVENTORY_INFO</code> </p> </li>
    /// <li> <p> <code>EMAIL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>API_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CLOUD_API</code> </p> </li>
    /// </ul>
    pub fn event_classes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_classes(input.into());
        self
    }
    /// <p>The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake. The supported event classes are:</p>
    /// <ul>
    /// <li> <p> <code>ACCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FILE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_EXTENSION</code> </p> </li>
    /// <li> <p> <code>MEMORY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>MODULE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>PROCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_KEY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_VALUE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RESOURCE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SCHEDULED_JOB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SECURITY_FINDING</code> </p> </li>
    /// <li> <p> <code>ACCOUNT_CHANGE</code> </p> </li>
    /// <li> <p> <code>AUTHENTICATION</code> </p> </li>
    /// <li> <p> <code>AUTHORIZATION</code> </p> </li>
    /// <li> <p> <code>ENTITY_MANAGEMENT_AUDIT</code> </p> </li>
    /// <li> <p> <code>DHCP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>NETWORK_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>DNS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>HTTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RDP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SMB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SSH_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CONFIG_STATE</code> </p> </li>
    /// <li> <p> <code>INVENTORY_INFO</code> </p> </li>
    /// <li> <p> <code>EMAIL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>API_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CLOUD_API</code> </p> </li>
    /// </ul>
    pub fn set_event_classes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_event_classes(input);
        self
    }
    /// <p>The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake. The supported event classes are:</p>
    /// <ul>
    /// <li> <p> <code>ACCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FILE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>KERNEL_EXTENSION</code> </p> </li>
    /// <li> <p> <code>MEMORY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>MODULE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>PROCESS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_KEY_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>REGISTRY_VALUE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RESOURCE_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SCHEDULED_JOB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SECURITY_FINDING</code> </p> </li>
    /// <li> <p> <code>ACCOUNT_CHANGE</code> </p> </li>
    /// <li> <p> <code>AUTHENTICATION</code> </p> </li>
    /// <li> <p> <code>AUTHORIZATION</code> </p> </li>
    /// <li> <p> <code>ENTITY_MANAGEMENT_AUDIT</code> </p> </li>
    /// <li> <p> <code>DHCP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>NETWORK_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>DNS_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>FTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>HTTP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>RDP_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SMB_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>SSH_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CONFIG_STATE</code> </p> </li>
    /// <li> <p> <code>INVENTORY_INFO</code> </p> </li>
    /// <li> <p> <code>EMAIL_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>API_ACTIVITY</code> </p> </li>
    /// <li> <p> <code>CLOUD_API</code> </p> </li>
    /// </ul>
    pub fn get_event_classes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_event_classes()
    }
    /// <p>The configuration for the third-party custom source.</p>
    pub fn configuration(mut self, input: crate::types::CustomLogSourceConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>The configuration for the third-party custom source.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::CustomLogSourceConfiguration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>The configuration for the third-party custom source.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::CustomLogSourceConfiguration> {
        self.inner.get_configuration()
    }
}
