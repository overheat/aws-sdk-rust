// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_file_system_aliases::_disassociate_file_system_aliases_output::DisassociateFileSystemAliasesOutputBuilder;

pub use crate::operation::disassociate_file_system_aliases::_disassociate_file_system_aliases_input::DisassociateFileSystemAliasesInputBuilder;

impl DisassociateFileSystemAliasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_file_system_aliases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateFileSystemAliases`.
///
/// <p>Use this action to disassociate, or remove, one or more Domain Name Service (DNS) aliases from an Amazon FSx for Windows File Server file system. If you attempt to disassociate a DNS alias that is not associated with the file system, Amazon FSx responds with a 400 Bad Request. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-dns-aliases.html">Working with DNS Aliases</a>.</p>
/// <p>The system generated response showing the DNS aliases that Amazon FSx is attempting to disassociate from the file system. Use the API operation to monitor the status of the aliases Amazon FSx is disassociating with the file system.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateFileSystemAliasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_file_system_aliases::builders::DisassociateFileSystemAliasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesOutput,
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesError,
    > for DisassociateFileSystemAliasesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesOutput,
            crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateFileSystemAliasesFluentBuilder {
    /// Creates a new `DisassociateFileSystemAliases`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateFileSystemAliases as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_file_system_aliases::builders::DisassociateFileSystemAliasesInputBuilder {
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
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesOutput,
        crate::operation::disassociate_file_system_aliases::DisassociateFileSystemAliasesError,
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
    /// <p>(Optional) An idempotency token for resource creation, in a string of up to 63 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>(Optional) An idempotency token for resource creation, in a string of up to 63 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>(Optional) An idempotency token for resource creation, in a string of up to 63 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>Specifies the file system from which to disassociate the DNS aliases.</p>
    pub fn file_system_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_system_id(input.into());
        self
    }
    /// <p>Specifies the file system from which to disassociate the DNS aliases.</p>
    pub fn set_file_system_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_file_system_id(input);
        self
    }
    /// <p>Specifies the file system from which to disassociate the DNS aliases.</p>
    pub fn get_file_system_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_file_system_id()
    }
    /// Appends an item to `Aliases`.
    ///
    /// To override the contents of this collection use [`set_aliases`](Self::set_aliases).
    ///
    /// <p>An array of one or more DNS alias names to disassociate, or remove, from the file system.</p>
    pub fn aliases(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aliases(input.into());
        self
    }
    /// <p>An array of one or more DNS alias names to disassociate, or remove, from the file system.</p>
    pub fn set_aliases(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_aliases(input);
        self
    }
    /// <p>An array of one or more DNS alias names to disassociate, or remove, from the file system.</p>
    pub fn get_aliases(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_aliases()
    }
}
