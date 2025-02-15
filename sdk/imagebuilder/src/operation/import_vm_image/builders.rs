// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_vm_image::_import_vm_image_output::ImportVmImageOutputBuilder;

pub use crate::operation::import_vm_image::_import_vm_image_input::ImportVmImageInputBuilder;

impl ImportVmImageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_vm_image::ImportVmImageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_vm_image::ImportVmImageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_vm_image();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportVmImage`.
///
/// <p>When you export your virtual machine (VM) from its virtualization environment, that process creates a set of one or more disk container files that act as snapshots of your VM’s environment, settings, and data. The Amazon EC2 API <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ImportImage.html">ImportImage</a> action uses those files to import your VM and create an AMI. To import using the CLI command, see <a href="https://docs.aws.amazon.com/cli/latest/reference/ec2/import-image.html">import-image</a> </p>
/// <p>You can reference the task ID from the VM import to pull in the AMI that the import created as the base image for your Image Builder recipe.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportVmImageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_vm_image::builders::ImportVmImageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_vm_image::ImportVmImageOutput,
        crate::operation::import_vm_image::ImportVmImageError,
    > for ImportVmImageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_vm_image::ImportVmImageOutput,
            crate::operation::import_vm_image::ImportVmImageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportVmImageFluentBuilder {
    /// Creates a new `ImportVmImage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportVmImage as a reference.
    pub fn as_input(&self) -> &crate::operation::import_vm_image::builders::ImportVmImageInputBuilder {
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
        crate::operation::import_vm_image::ImportVmImageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_vm_image::ImportVmImageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_vm_image::ImportVmImage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_vm_image::ImportVmImage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_vm_image::ImportVmImageOutput,
        crate::operation::import_vm_image::ImportVmImageError,
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
    /// <p>The name of the base image that is created by the import process.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the base image that is created by the import process.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the base image that is created by the import process.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The semantic version to attach to the base image that was created during the import process. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the build number to the fourth node.</p>
    /// <p> <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or a date, such as 2021.01.01.</p>
    /// </note>
    pub fn semantic_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.semantic_version(input.into());
        self
    }
    /// <p>The semantic version to attach to the base image that was created during the import process. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the build number to the fourth node.</p>
    /// <p> <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or a date, such as 2021.01.01.</p>
    /// </note>
    pub fn set_semantic_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_semantic_version(input);
        self
    }
    /// <p>The semantic version to attach to the base image that was created during the import process. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the build number to the fourth node.</p>
    /// <p> <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or a date, such as 2021.01.01.</p>
    /// </note>
    pub fn get_semantic_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_semantic_version()
    }
    /// <p>The description for the base image that is created by the import process.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description for the base image that is created by the import process.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description for the base image that is created by the import process.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The operating system platform for the imported VM.</p>
    pub fn platform(mut self, input: crate::types::Platform) -> Self {
        self.inner = self.inner.platform(input);
        self
    }
    /// <p>The operating system platform for the imported VM.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<crate::types::Platform>) -> Self {
        self.inner = self.inner.set_platform(input);
        self
    }
    /// <p>The operating system platform for the imported VM.</p>
    pub fn get_platform(&self) -> &::std::option::Option<crate::types::Platform> {
        self.inner.get_platform()
    }
    /// <p>The operating system version for the imported VM.</p>
    pub fn os_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.os_version(input.into());
        self
    }
    /// <p>The operating system version for the imported VM.</p>
    pub fn set_os_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_os_version(input);
        self
    }
    /// <p>The operating system version for the imported VM.</p>
    pub fn get_os_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_os_version()
    }
    /// <p>The <code>importTaskId</code> (API) or <code>ImportTaskId</code> (CLI) from the Amazon EC2 VM import process. Image Builder retrieves information from the import process to pull in the AMI that is created from the VM source as the base image for your recipe.</p>
    pub fn vm_import_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vm_import_task_id(input.into());
        self
    }
    /// <p>The <code>importTaskId</code> (API) or <code>ImportTaskId</code> (CLI) from the Amazon EC2 VM import process. Image Builder retrieves information from the import process to pull in the AMI that is created from the VM source as the base image for your recipe.</p>
    pub fn set_vm_import_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vm_import_task_id(input);
        self
    }
    /// <p>The <code>importTaskId</code> (API) or <code>ImportTaskId</code> (CLI) from the Amazon EC2 VM import process. Image Builder retrieves information from the import process to pull in the AMI that is created from the VM source as the base image for your recipe.</p>
    pub fn get_vm_import_task_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vm_import_task_id()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags that are attached to the import resources.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Tags that are attached to the import resources.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags that are attached to the import resources.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
