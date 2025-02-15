// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_deployment::_create_deployment_output::CreateDeploymentOutputBuilder;

pub use crate::operation::create_deployment::_create_deployment_input::CreateDeploymentInputBuilder;

impl CreateDeploymentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_deployment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDeployment`.
///
/// <p>Runs deployment or stack commands. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html">Deploying Apps</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html">Run Stack Commands</a>.</p>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDeploymentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_deployment::builders::CreateDeploymentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_deployment::CreateDeploymentOutput,
        crate::operation::create_deployment::CreateDeploymentError,
    > for CreateDeploymentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_deployment::CreateDeploymentOutput,
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDeploymentFluentBuilder {
    /// Creates a new `CreateDeployment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDeployment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_deployment::builders::CreateDeploymentInputBuilder {
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
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_deployment::CreateDeployment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_deployment::CreateDeployment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_deployment::CreateDeploymentOutput,
        crate::operation::create_deployment::CreateDeploymentError,
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
    /// <p>The stack ID.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_id(input.into());
        self
    }
    /// <p>The stack ID.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_id(input);
        self
    }
    /// <p>The stack ID.</p>
    pub fn get_stack_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stack_id()
    }
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    pub fn get_app_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_id()
    }
    /// Appends an item to `InstanceIds`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>The instance IDs for the deployment targets.</p>
    pub fn instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_ids(input.into());
        self
    }
    /// <p>The instance IDs for the deployment targets.</p>
    pub fn set_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_instance_ids(input);
        self
    }
    /// <p>The instance IDs for the deployment targets.</p>
    pub fn get_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_instance_ids()
    }
    /// Appends an item to `LayerIds`.
    ///
    /// To override the contents of this collection use [`set_layer_ids`](Self::set_layer_ids).
    ///
    /// <p>The layer IDs for the deployment targets.</p>
    pub fn layer_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.layer_ids(input.into());
        self
    }
    /// <p>The layer IDs for the deployment targets.</p>
    pub fn set_layer_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_layer_ids(input);
        self
    }
    /// <p>The layer IDs for the deployment targets.</p>
    pub fn get_layer_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_layer_ids()
    }
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    pub fn command(mut self, input: crate::types::DeploymentCommand) -> Self {
        self.inner = self.inner.command(input);
        self
    }
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    pub fn set_command(mut self, input: ::std::option::Option<crate::types::DeploymentCommand>) -> Self {
        self.inner = self.inner.set_command(input);
        self
    }
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    pub fn get_command(&self) -> &::std::option::Option<crate::types::DeploymentCommand> {
        self.inner.get_command()
    }
    /// <p>A user-defined comment.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>A user-defined comment.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>A user-defined comment.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment()
    }
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p>
    /// <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p>
    /// <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    pub fn custom_json(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.custom_json(input.into());
        self
    }
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p>
    /// <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p>
    /// <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    pub fn set_custom_json(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_custom_json(input);
        self
    }
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p>
    /// <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p>
    /// <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    pub fn get_custom_json(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_custom_json()
    }
}
