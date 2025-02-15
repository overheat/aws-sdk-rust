// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_option_groups::_describe_option_groups_output::DescribeOptionGroupsOutputBuilder;

pub use crate::operation::describe_option_groups::_describe_option_groups_input::DescribeOptionGroupsInputBuilder;

impl DescribeOptionGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_option_groups::DescribeOptionGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_option_groups::DescribeOptionGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_option_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeOptionGroups`.
///
/// <p>Describes the available option groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeOptionGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_option_groups::builders::DescribeOptionGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_option_groups::DescribeOptionGroupsOutput,
        crate::operation::describe_option_groups::DescribeOptionGroupsError,
    > for DescribeOptionGroupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_option_groups::DescribeOptionGroupsOutput,
            crate::operation::describe_option_groups::DescribeOptionGroupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeOptionGroupsFluentBuilder {
    /// Creates a new `DescribeOptionGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeOptionGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_option_groups::builders::DescribeOptionGroupsInputBuilder {
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
        crate::operation::describe_option_groups::DescribeOptionGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_option_groups::DescribeOptionGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_option_groups::DescribeOptionGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_option_groups::DescribeOptionGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_option_groups::DescribeOptionGroupsOutput,
        crate::operation::describe_option_groups::DescribeOptionGroupsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_option_groups::paginator::DescribeOptionGroupsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_option_groups::paginator::DescribeOptionGroupsPaginator {
        crate::operation::describe_option_groups::paginator::DescribeOptionGroupsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the option group to describe. Can't be supplied together with EngineName or MajorEngineVersion.</p>
    pub fn option_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.option_group_name(input.into());
        self
    }
    /// <p>The name of the option group to describe. Can't be supplied together with EngineName or MajorEngineVersion.</p>
    pub fn set_option_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_option_group_name(input);
        self
    }
    /// <p>The name of the option group to describe. Can't be supplied together with EngineName or MajorEngineVersion.</p>
    pub fn get_option_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_option_group_name()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>This parameter isn't currently supported.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>This parameter isn't currently supported.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>This parameter isn't currently supported.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>An optional pagination token provided by a previous DescribeOptionGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous DescribeOptionGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional pagination token provided by a previous DescribeOptionGroups request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine.</p>
    /// <p>Valid Values:</p>
    /// <ul>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-ee-cdb</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>oracle-se2-cdb</code> </p> </li>
    /// <li> <p> <code>postgres</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn engine_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_name(input.into());
        self
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine.</p>
    /// <p>Valid Values:</p>
    /// <ul>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-ee-cdb</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>oracle-se2-cdb</code> </p> </li>
    /// <li> <p> <code>postgres</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn set_engine_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_name(input);
        self
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine.</p>
    /// <p>Valid Values:</p>
    /// <ul>
    /// <li> <p> <code>mariadb</code> </p> </li>
    /// <li> <p> <code>mysql</code> </p> </li>
    /// <li> <p> <code>oracle-ee</code> </p> </li>
    /// <li> <p> <code>oracle-ee-cdb</code> </p> </li>
    /// <li> <p> <code>oracle-se2</code> </p> </li>
    /// <li> <p> <code>oracle-se2-cdb</code> </p> </li>
    /// <li> <p> <code>postgres</code> </p> </li>
    /// <li> <p> <code>sqlserver-ee</code> </p> </li>
    /// <li> <p> <code>sqlserver-se</code> </p> </li>
    /// <li> <p> <code>sqlserver-ex</code> </p> </li>
    /// <li> <p> <code>sqlserver-web</code> </p> </li>
    /// </ul>
    pub fn get_engine_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_name()
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine version. If specified, then EngineName must also be specified.</p>
    pub fn major_engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.major_engine_version(input.into());
        self
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine version. If specified, then EngineName must also be specified.</p>
    pub fn set_major_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_major_engine_version(input);
        self
    }
    /// <p>Filters the list of option groups to only include groups associated with a specific database engine version. If specified, then EngineName must also be specified.</p>
    pub fn get_major_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_major_engine_version()
    }
}
