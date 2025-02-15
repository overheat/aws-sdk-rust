// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_fast_launch::_disable_fast_launch_output::DisableFastLaunchOutputBuilder;

pub use crate::operation::disable_fast_launch::_disable_fast_launch_input::DisableFastLaunchInputBuilder;

impl DisableFastLaunchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_fast_launch::DisableFastLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_fast_launch::DisableFastLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_fast_launch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableFastLaunch`.
///
/// <p>Discontinue faster launching for a Windows AMI, and clean up existing pre-provisioned snapshots. When you disable faster launching, the AMI uses the standard launch process for each instance. All pre-provisioned snapshots must be removed before you can enable faster launching again.</p> <note>
/// <p>To change these settings, you must own the AMI.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableFastLaunchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_fast_launch::builders::DisableFastLaunchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_fast_launch::DisableFastLaunchOutput,
        crate::operation::disable_fast_launch::DisableFastLaunchError,
    > for DisableFastLaunchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_fast_launch::DisableFastLaunchOutput,
            crate::operation::disable_fast_launch::DisableFastLaunchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableFastLaunchFluentBuilder {
    /// Creates a new `DisableFastLaunch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableFastLaunch as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_fast_launch::builders::DisableFastLaunchInputBuilder {
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
        crate::operation::disable_fast_launch::DisableFastLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_fast_launch::DisableFastLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_fast_launch::DisableFastLaunch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_fast_launch::DisableFastLaunch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_fast_launch::DisableFastLaunchOutput,
        crate::operation::disable_fast_launch::DisableFastLaunchError,
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
    /// <p>The ID of the image for which you’re turning off faster launching, and removing pre-provisioned snapshots.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the image for which you’re turning off faster launching, and removing pre-provisioned snapshots.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>The ID of the image for which you’re turning off faster launching, and removing pre-provisioned snapshots.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_id()
    }
    /// <p>Forces the image settings to turn off faster launching for your Windows AMI. This parameter overrides any errors that are encountered while cleaning up resources in your account.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>Forces the image settings to turn off faster launching for your Windows AMI. This parameter overrides any errors that are encountered while cleaning up resources in your account.</p>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>Forces the image settings to turn off faster launching for your Windows AMI. This parameter overrides any errors that are encountered while cleaning up resources in your account.</p>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        self.inner.get_force()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
