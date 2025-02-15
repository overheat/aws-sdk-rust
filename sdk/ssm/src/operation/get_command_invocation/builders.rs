// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_command_invocation::_get_command_invocation_output::GetCommandInvocationOutputBuilder;

pub use crate::operation::get_command_invocation::_get_command_invocation_input::GetCommandInvocationInputBuilder;

impl GetCommandInvocationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_command_invocation::GetCommandInvocationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_command_invocation::GetCommandInvocationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_command_invocation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetCommandInvocation`.
///
/// <p>Returns detailed information about command execution for an invocation or plugin.</p>
/// <p> <code>GetCommandInvocation</code> only gives the execution status of a plugin in a document. To get the command execution status on a specific managed node, use <code>ListCommandInvocations</code>. To get the command execution status across managed nodes, use <code>ListCommands</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCommandInvocationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_command_invocation::builders::GetCommandInvocationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_command_invocation::GetCommandInvocationOutput,
        crate::operation::get_command_invocation::GetCommandInvocationError,
    > for GetCommandInvocationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_command_invocation::GetCommandInvocationOutput,
            crate::operation::get_command_invocation::GetCommandInvocationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetCommandInvocationFluentBuilder {
    /// Creates a new `GetCommandInvocation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetCommandInvocation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_command_invocation::builders::GetCommandInvocationInputBuilder {
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
        crate::operation::get_command_invocation::GetCommandInvocationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_command_invocation::GetCommandInvocationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_command_invocation::GetCommandInvocation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_command_invocation::GetCommandInvocation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_command_invocation::GetCommandInvocationOutput,
        crate::operation::get_command_invocation::GetCommandInvocationError,
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
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn command_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.command_id(input.into());
        self
    }
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn set_command_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_command_id(input);
        self
    }
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn get_command_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_command_id()
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn plugin_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plugin_name(input.into());
        self
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn set_plugin_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plugin_name(input);
        self
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn get_plugin_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plugin_name()
    }
}
