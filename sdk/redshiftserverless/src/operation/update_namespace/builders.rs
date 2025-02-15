// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_namespace::_update_namespace_output::UpdateNamespaceOutputBuilder;

pub use crate::operation::update_namespace::_update_namespace_input::UpdateNamespaceInputBuilder;

impl UpdateNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_namespace::UpdateNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_namespace::UpdateNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateNamespace`.
///
/// <p>Updates a namespace with the specified settings. Unless required, you can't update multiple parameters in one request. For example, you must specify both <code>adminUsername</code> and <code>adminUserPassword</code> to update either field, but you can't update both <code>kmsKeyId</code> and <code>logExports</code> in a single request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_namespace::builders::UpdateNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_namespace::UpdateNamespaceOutput,
        crate::operation::update_namespace::UpdateNamespaceError,
    > for UpdateNamespaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_namespace::UpdateNamespaceOutput,
            crate::operation::update_namespace::UpdateNamespaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateNamespaceFluentBuilder {
    /// Creates a new `UpdateNamespace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::update_namespace::builders::UpdateNamespaceInputBuilder {
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
        crate::operation::update_namespace::UpdateNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_namespace::UpdateNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_namespace::UpdateNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_namespace::UpdateNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_namespace::UpdateNamespaceOutput,
        crate::operation::update_namespace::UpdateNamespaceError,
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
    /// <p>The name of the namespace to update. You can't update the name of a namespace once it is created.</p>
    pub fn namespace_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace_name(input.into());
        self
    }
    /// <p>The name of the namespace to update. You can't update the name of a namespace once it is created.</p>
    pub fn set_namespace_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_name(input);
        self
    }
    /// <p>The name of the namespace to update. You can't update the name of a namespace once it is created.</p>
    pub fn get_namespace_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace_name()
    }
    /// <p>The password of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUsername</code>.</p>
    /// <p>You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. </p>
    pub fn admin_user_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.admin_user_password(input.into());
        self
    }
    /// <p>The password of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUsername</code>.</p>
    /// <p>You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. </p>
    pub fn set_admin_user_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_admin_user_password(input);
        self
    }
    /// <p>The password of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUsername</code>.</p>
    /// <p>You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. </p>
    pub fn get_admin_user_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_admin_user_password()
    }
    /// <p>The username of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUserPassword</code>.</p>
    pub fn admin_username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.admin_username(input.into());
        self
    }
    /// <p>The username of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUserPassword</code>.</p>
    pub fn set_admin_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_admin_username(input);
        self
    }
    /// <p>The username of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUserPassword</code>.</p>
    pub fn get_admin_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_admin_username()
    }
    /// <p>The ID of the Amazon Web Services Key Management Service key used to encrypt your data.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Key Management Service key used to encrypt your data.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services Key Management Service key used to encrypt your data.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. This parameter must be updated together with <code>iamRoles</code>.</p>
    pub fn default_iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.default_iam_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. This parameter must be updated together with <code>iamRoles</code>.</p>
    pub fn set_default_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_default_iam_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. This parameter must be updated together with <code>iamRoles</code>.</p>
    pub fn get_default_iam_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_default_iam_role_arn()
    }
    /// Appends an item to `iamRoles`.
    ///
    /// To override the contents of this collection use [`set_iam_roles`](Self::set_iam_roles).
    ///
    /// <p>A list of IAM roles to associate with the namespace. This parameter must be updated together with <code>defaultIamRoleArn</code>.</p>
    pub fn iam_roles(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.iam_roles(input.into());
        self
    }
    /// <p>A list of IAM roles to associate with the namespace. This parameter must be updated together with <code>defaultIamRoleArn</code>.</p>
    pub fn set_iam_roles(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_iam_roles(input);
        self
    }
    /// <p>A list of IAM roles to associate with the namespace. This parameter must be updated together with <code>defaultIamRoleArn</code>.</p>
    pub fn get_iam_roles(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_iam_roles()
    }
    /// Appends an item to `logExports`.
    ///
    /// To override the contents of this collection use [`set_log_exports`](Self::set_log_exports).
    ///
    /// <p>The types of logs the namespace can export. The export types are <code>userlog</code>, <code>connectionlog</code>, and <code>useractivitylog</code>.</p>
    pub fn log_exports(mut self, input: crate::types::LogExport) -> Self {
        self.inner = self.inner.log_exports(input);
        self
    }
    /// <p>The types of logs the namespace can export. The export types are <code>userlog</code>, <code>connectionlog</code>, and <code>useractivitylog</code>.</p>
    pub fn set_log_exports(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LogExport>>) -> Self {
        self.inner = self.inner.set_log_exports(input);
        self
    }
    /// <p>The types of logs the namespace can export. The export types are <code>userlog</code>, <code>connectionlog</code>, and <code>useractivitylog</code>.</p>
    pub fn get_log_exports(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LogExport>> {
        self.inner.get_log_exports()
    }
    /// <p>If <code>true</code>, Amazon Redshift uses Secrets Manager to manage the namespace's admin credentials. You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. If <code>manageAdminPassword</code> is false or not set, Amazon Redshift uses <code>adminUserPassword</code> for the admin user account's password. </p>
    pub fn manage_admin_password(mut self, input: bool) -> Self {
        self.inner = self.inner.manage_admin_password(input);
        self
    }
    /// <p>If <code>true</code>, Amazon Redshift uses Secrets Manager to manage the namespace's admin credentials. You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. If <code>manageAdminPassword</code> is false or not set, Amazon Redshift uses <code>adminUserPassword</code> for the admin user account's password. </p>
    pub fn set_manage_admin_password(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_manage_admin_password(input);
        self
    }
    /// <p>If <code>true</code>, Amazon Redshift uses Secrets Manager to manage the namespace's admin credentials. You can't use <code>adminUserPassword</code> if <code>manageAdminPassword</code> is true. If <code>manageAdminPassword</code> is false or not set, Amazon Redshift uses <code>adminUserPassword</code> for the admin user account's password. </p>
    pub fn get_manage_admin_password(&self) -> &::std::option::Option<bool> {
        self.inner.get_manage_admin_password()
    }
    /// <p>The ID of the Key Management Service (KMS) key used to encrypt and store the namespace's admin credentials secret. You can only use this parameter if <code>manageAdminPassword</code> is true.</p>
    pub fn admin_password_secret_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.admin_password_secret_kms_key_id(input.into());
        self
    }
    /// <p>The ID of the Key Management Service (KMS) key used to encrypt and store the namespace's admin credentials secret. You can only use this parameter if <code>manageAdminPassword</code> is true.</p>
    pub fn set_admin_password_secret_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_admin_password_secret_kms_key_id(input);
        self
    }
    /// <p>The ID of the Key Management Service (KMS) key used to encrypt and store the namespace's admin credentials secret. You can only use this parameter if <code>manageAdminPassword</code> is true.</p>
    pub fn get_admin_password_secret_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_admin_password_secret_kms_key_id()
    }
}
