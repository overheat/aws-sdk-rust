// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_app::_update_app_output::UpdateAppOutputBuilder;

pub use crate::operation::update_app::_update_app_input::UpdateAppInputBuilder;

impl UpdateAppInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_app::UpdateAppOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_app::UpdateAppError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_app();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateApp`.
///
/// <p>Updates a specified app.</p>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAppFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_app::builders::UpdateAppInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::update_app::UpdateAppOutput, crate::operation::update_app::UpdateAppError>
    for UpdateAppFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::update_app::UpdateAppOutput, crate::operation::update_app::UpdateAppError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAppFluentBuilder {
    /// Creates a new `UpdateApp`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateApp as a reference.
    pub fn as_input(&self) -> &crate::operation::update_app::builders::UpdateAppInputBuilder {
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
        crate::operation::update_app::UpdateAppOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_app::UpdateAppError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_app::UpdateApp::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_app::UpdateApp::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_app::UpdateAppOutput,
        crate::operation::update_app::UpdateAppError,
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
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The app ID.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// <p>The app name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The app name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The app name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description of the app.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the app.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the app.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `DataSources`.
    ///
    /// To override the contents of this collection use [`set_data_sources`](Self::set_data_sources).
    ///
    /// <p>The app's data sources.</p>
    pub fn data_sources(mut self, input: crate::types::DataSource) -> Self {
        self.inner = self.inner.data_sources(input);
        self
    }
    /// <p>The app's data sources.</p>
    pub fn set_data_sources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DataSource>>) -> Self {
        self.inner = self.inner.set_data_sources(input);
        self
    }
    /// <p>The app's data sources.</p>
    pub fn get_data_sources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DataSource>> {
        self.inner.get_data_sources()
    }
    /// <p>The app type.</p>
    pub fn r#type(mut self, input: crate::types::AppType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The app type.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::AppType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The app type.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::AppType> {
        self.inner.get_type()
    }
    /// <p>A <code>Source</code> object that specifies the app repository.</p>
    pub fn app_source(mut self, input: crate::types::Source) -> Self {
        self.inner = self.inner.app_source(input);
        self
    }
    /// <p>A <code>Source</code> object that specifies the app repository.</p>
    pub fn set_app_source(mut self, input: ::std::option::Option<crate::types::Source>) -> Self {
        self.inner = self.inner.set_app_source(input);
        self
    }
    /// <p>A <code>Source</code> object that specifies the app repository.</p>
    pub fn get_app_source(&self) -> &::std::option::Option<crate::types::Source> {
        self.inner.get_app_source()
    }
    /// Appends an item to `Domains`.
    ///
    /// To override the contents of this collection use [`set_domains`](Self::set_domains).
    ///
    /// <p>The app's virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    pub fn domains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domains(input.into());
        self
    }
    /// <p>The app's virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    pub fn set_domains(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_domains(input);
        self
    }
    /// <p>The app's virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    pub fn get_domains(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_domains()
    }
    /// <p>Whether SSL is enabled for the app.</p>
    pub fn enable_ssl(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_ssl(input);
        self
    }
    /// <p>Whether SSL is enabled for the app.</p>
    pub fn set_enable_ssl(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_ssl(input);
        self
    }
    /// <p>Whether SSL is enabled for the app.</p>
    pub fn get_enable_ssl(&self) -> &::std::option::Option<bool> {
        self.inner.get_enable_ssl()
    }
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    pub fn ssl_configuration(mut self, input: crate::types::SslConfiguration) -> Self {
        self.inner = self.inner.ssl_configuration(input);
        self
    }
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    pub fn set_ssl_configuration(mut self, input: ::std::option::Option<crate::types::SslConfiguration>) -> Self {
        self.inner = self.inner.set_ssl_configuration(input);
        self
    }
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    pub fn get_ssl_configuration(&self) -> &::std::option::Option<crate::types::SslConfiguration> {
        self.inner.get_ssl_configuration()
    }
    /// Adds a key-value pair to `Attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    pub fn attributes(mut self, k: crate::types::AppAttributesKeys, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attributes(k, v.into());
        self
    }
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::AppAttributesKeys, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<crate::types::AppAttributesKeys, ::std::string::String>> {
        self.inner.get_attributes()
    }
    /// Appends an item to `Environment`.
    ///
    /// To override the contents of this collection use [`set_environment`](Self::set_environment).
    ///
    /// <p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances.For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>.</p>
    /// <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, "Environment: is too large (maximum is 20 KB)."</p> <note>
    /// <p>If you have specified one or more environment variables, you cannot modify the stack's Chef version.</p>
    /// </note>
    pub fn environment(mut self, input: crate::types::EnvironmentVariable) -> Self {
        self.inner = self.inner.environment(input);
        self
    }
    /// <p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances.For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>.</p>
    /// <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, "Environment: is too large (maximum is 20 KB)."</p> <note>
    /// <p>If you have specified one or more environment variables, you cannot modify the stack's Chef version.</p>
    /// </note>
    pub fn set_environment(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentVariable>>) -> Self {
        self.inner = self.inner.set_environment(input);
        self
    }
    /// <p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances.For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>.</p>
    /// <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, "Environment: is too large (maximum is 20 KB)."</p> <note>
    /// <p>If you have specified one or more environment variables, you cannot modify the stack's Chef version.</p>
    /// </note>
    pub fn get_environment(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EnvironmentVariable>> {
        self.inner.get_environment()
    }
}
