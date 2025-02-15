// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_backups::_describe_backups_output::DescribeBackupsOutputBuilder;

pub use crate::operation::describe_backups::_describe_backups_input::DescribeBackupsInputBuilder;

impl DescribeBackupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_backups::DescribeBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_backups::DescribeBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_backups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeBackups`.
///
/// <p> Describes backups. The results are ordered by time, with newest backups first. If you do not specify a BackupId or ServerName, the command returns all backups. </p>
/// <p> This operation is synchronous. </p>
/// <p> A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeBackupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_backups::builders::DescribeBackupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_backups::DescribeBackupsOutput,
        crate::operation::describe_backups::DescribeBackupsError,
    > for DescribeBackupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_backups::DescribeBackupsOutput,
            crate::operation::describe_backups::DescribeBackupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeBackupsFluentBuilder {
    /// Creates a new `DescribeBackups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeBackups as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_backups::builders::DescribeBackupsInputBuilder {
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
        crate::operation::describe_backups::DescribeBackupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_backups::DescribeBackupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_backups::DescribeBackups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_backups::DescribeBackups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_backups::DescribeBackupsOutput,
        crate::operation::describe_backups::DescribeBackupsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_backups::paginator::DescribeBackupsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_backups::paginator::DescribeBackupsPaginator {
        crate::operation::describe_backups::paginator::DescribeBackupsPaginator::new(self.handle, self.inner)
    }
    /// <p>Describes a single backup. </p>
    pub fn backup_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_id(input.into());
        self
    }
    /// <p>Describes a single backup. </p>
    pub fn set_backup_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_id(input);
        self
    }
    /// <p>Describes a single backup. </p>
    pub fn get_backup_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_id()
    }
    /// <p>Returns backups for the server with the specified ServerName. </p>
    pub fn server_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.server_name(input.into());
        self
    }
    /// <p>Returns backups for the server with the specified ServerName. </p>
    pub fn set_server_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_server_name(input);
        self
    }
    /// <p>Returns backups for the server with the specified ServerName. </p>
    pub fn get_server_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_server_name()
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
