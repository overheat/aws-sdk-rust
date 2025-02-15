// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connection_alias_permission::_update_connection_alias_permission_output::UpdateConnectionAliasPermissionOutputBuilder;

pub use crate::operation::update_connection_alias_permission::_update_connection_alias_permission_input::UpdateConnectionAliasPermissionInputBuilder;

impl UpdateConnectionAliasPermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_connection_alias_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConnectionAliasPermission`.
///
/// <p>Shares or unshares a connection alias with one account by specifying whether that account has permission to associate the connection alias with a directory. If the association permission is granted, the connection alias is shared with that account. If the association permission is revoked, the connection alias is unshared with the account. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/cross-region-redirection.html"> Cross-Region Redirection for Amazon WorkSpaces</a>.</p> <note>
/// <ul>
/// <li> <p>Before performing this operation, call <a href="https://docs.aws.amazon.com/workspaces/latest/api/API_DescribeConnectionAliases.html"> DescribeConnectionAliases</a> to make sure that the current state of the connection alias is <code>CREATED</code>.</p> </li>
/// <li> <p>To delete a connection alias that has been shared, the shared account must first disassociate the connection alias from any directories it has been associated with. Then you must unshare the connection alias from the account it has been shared with. You can delete a connection alias only after it is no longer shared with any accounts or associated with any directories.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConnectionAliasPermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connection_alias_permission::builders::UpdateConnectionAliasPermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionOutput,
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionError,
    > for UpdateConnectionAliasPermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionOutput,
            crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConnectionAliasPermissionFluentBuilder {
    /// Creates a new `UpdateConnectionAliasPermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConnectionAliasPermission as a reference.
    pub fn as_input(&self) -> &crate::operation::update_connection_alias_permission::builders::UpdateConnectionAliasPermissionInputBuilder {
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
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionOutput,
        crate::operation::update_connection_alias_permission::UpdateConnectionAliasPermissionError,
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
    /// <p>The identifier of the connection alias that you want to update permissions for.</p>
    pub fn alias_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias_id(input.into());
        self
    }
    /// <p>The identifier of the connection alias that you want to update permissions for.</p>
    pub fn set_alias_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias_id(input);
        self
    }
    /// <p>The identifier of the connection alias that you want to update permissions for.</p>
    pub fn get_alias_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alias_id()
    }
    /// <p>Indicates whether to share or unshare the connection alias with the specified Amazon Web Services account.</p>
    pub fn connection_alias_permission(mut self, input: crate::types::ConnectionAliasPermission) -> Self {
        self.inner = self.inner.connection_alias_permission(input);
        self
    }
    /// <p>Indicates whether to share or unshare the connection alias with the specified Amazon Web Services account.</p>
    pub fn set_connection_alias_permission(mut self, input: ::std::option::Option<crate::types::ConnectionAliasPermission>) -> Self {
        self.inner = self.inner.set_connection_alias_permission(input);
        self
    }
    /// <p>Indicates whether to share or unshare the connection alias with the specified Amazon Web Services account.</p>
    pub fn get_connection_alias_permission(&self) -> &::std::option::Option<crate::types::ConnectionAliasPermission> {
        self.inner.get_connection_alias_permission()
    }
}
