// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_directory::_create_directory_output::CreateDirectoryOutputBuilder;

pub use crate::operation::create_directory::_create_directory_input::CreateDirectoryInputBuilder;

impl CreateDirectoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_directory::CreateDirectoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_directory::CreateDirectoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_directory();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDirectory`.
///
/// <p>Creates a Simple AD directory. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_simple_ad.html">Simple Active Directory</a> in the <i>Directory Service Admin Guide</i>.</p>
/// <p>Before you call <code>CreateDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>CreateDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDirectoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_directory::builders::CreateDirectoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_directory::CreateDirectoryOutput,
        crate::operation::create_directory::CreateDirectoryError,
    > for CreateDirectoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_directory::CreateDirectoryOutput,
            crate::operation::create_directory::CreateDirectoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDirectoryFluentBuilder {
    /// Creates a new `CreateDirectory`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDirectory as a reference.
    pub fn as_input(&self) -> &crate::operation::create_directory::builders::CreateDirectoryInputBuilder {
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
        crate::operation::create_directory::CreateDirectoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_directory::CreateDirectoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_directory::CreateDirectory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_directory::CreateDirectory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_directory::CreateDirectoryOutput,
        crate::operation::create_directory::CreateDirectoryError,
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
    /// <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The NetBIOS name of the directory, such as <code>CORP</code>.</p>
    pub fn short_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.short_name(input.into());
        self
    }
    /// <p>The NetBIOS name of the directory, such as <code>CORP</code>.</p>
    pub fn set_short_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_short_name(input);
        self
    }
    /// <p>The NetBIOS name of the directory, such as <code>CORP</code>.</p>
    pub fn get_short_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_short_name()
    }
    /// <p>The password for the directory administrator. The directory creation process creates a directory administrator account with the user name <code>Administrator</code> and this password.</p>
    /// <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    /// <p>The regex pattern for this string is made up of the following conditions:</p>
    /// <ul>
    /// <li> <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p> </li>
    /// </ul>
    /// <p>AND any 3 of the following password complexity rules required by Active Directory:</p>
    /// <ul>
    /// <li> <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and special characters and lower case (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p> </li>
    /// <li> <p>Special characters and upper case and lower case (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and upper case and special characters (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p> </li>
    /// </ul>
    /// <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>The password for the directory administrator. The directory creation process creates a directory administrator account with the user name <code>Administrator</code> and this password.</p>
    /// <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    /// <p>The regex pattern for this string is made up of the following conditions:</p>
    /// <ul>
    /// <li> <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p> </li>
    /// </ul>
    /// <p>AND any 3 of the following password complexity rules required by Active Directory:</p>
    /// <ul>
    /// <li> <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and special characters and lower case (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p> </li>
    /// <li> <p>Special characters and upper case and lower case (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and upper case and special characters (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p> </li>
    /// </ul>
    /// <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
    /// <p>The password for the directory administrator. The directory creation process creates a directory administrator account with the user name <code>Administrator</code> and this password.</p>
    /// <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    /// <p>The regex pattern for this string is made up of the following conditions:</p>
    /// <ul>
    /// <li> <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p> </li>
    /// </ul>
    /// <p>AND any 3 of the following password complexity rules required by Active Directory:</p>
    /// <ul>
    /// <li> <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and special characters and lower case (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p> </li>
    /// <li> <p>Special characters and upper case and lower case (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p> </li>
    /// <li> <p>Numbers and upper case and special characters (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p> </li>
    /// </ul>
    /// <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p>
    pub fn get_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_password()
    }
    /// <p>A description for the directory.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the directory.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the directory.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The size of the directory.</p>
    pub fn size(mut self, input: crate::types::DirectorySize) -> Self {
        self.inner = self.inner.size(input);
        self
    }
    /// <p>The size of the directory.</p>
    pub fn set_size(mut self, input: ::std::option::Option<crate::types::DirectorySize>) -> Self {
        self.inner = self.inner.set_size(input);
        self
    }
    /// <p>The size of the directory.</p>
    pub fn get_size(&self) -> &::std::option::Option<crate::types::DirectorySize> {
        self.inner.get_size()
    }
    /// <p>A <code>DirectoryVpcSettings</code> object that contains additional information for the operation.</p>
    pub fn vpc_settings(mut self, input: crate::types::DirectoryVpcSettings) -> Self {
        self.inner = self.inner.vpc_settings(input);
        self
    }
    /// <p>A <code>DirectoryVpcSettings</code> object that contains additional information for the operation.</p>
    pub fn set_vpc_settings(mut self, input: ::std::option::Option<crate::types::DirectoryVpcSettings>) -> Self {
        self.inner = self.inner.set_vpc_settings(input);
        self
    }
    /// <p>A <code>DirectoryVpcSettings</code> object that contains additional information for the operation.</p>
    pub fn get_vpc_settings(&self) -> &::std::option::Option<crate::types::DirectoryVpcSettings> {
        self.inner.get_vpc_settings()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be assigned to the Simple AD directory.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to be assigned to the Simple AD directory.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to be assigned to the Simple AD directory.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
