// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_index_fields::_describe_index_fields_output::DescribeIndexFieldsOutputBuilder;

pub use crate::operation::describe_index_fields::_describe_index_fields_input::DescribeIndexFieldsInputBuilder;

impl DescribeIndexFieldsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_index_fields::DescribeIndexFieldsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_index_fields::DescribeIndexFieldsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_index_fields();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeIndexFields`.
///
/// <p>Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-domain-info.html" target="_blank">Getting Domain Information</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeIndexFieldsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_index_fields::builders::DescribeIndexFieldsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_index_fields::DescribeIndexFieldsOutput,
        crate::operation::describe_index_fields::DescribeIndexFieldsError,
    > for DescribeIndexFieldsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_index_fields::DescribeIndexFieldsOutput,
            crate::operation::describe_index_fields::DescribeIndexFieldsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeIndexFieldsFluentBuilder {
    /// Creates a new `DescribeIndexFields`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeIndexFields as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_index_fields::builders::DescribeIndexFieldsInputBuilder {
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
        crate::operation::describe_index_fields::DescribeIndexFieldsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_index_fields::DescribeIndexFieldsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_index_fields::DescribeIndexFields::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_index_fields::DescribeIndexFields::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_index_fields::DescribeIndexFieldsOutput,
        crate::operation::describe_index_fields::DescribeIndexFieldsError,
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
    /// <p>The name of the domain you want to describe.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// Appends an item to `FieldNames`.
    ///
    /// To override the contents of this collection use [`set_field_names`](Self::set_field_names).
    ///
    /// <p>A list of the index fields you want to describe. If not specified, information is returned for all configured index fields.</p>
    pub fn field_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.field_names(input.into());
        self
    }
    /// <p>A list of the index fields you want to describe. If not specified, information is returned for all configured index fields.</p>
    pub fn set_field_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_field_names(input);
        self
    }
    /// <p>A list of the index fields you want to describe. If not specified, information is returned for all configured index fields.</p>
    pub fn get_field_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_field_names()
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn deployed(mut self, input: bool) -> Self {
        self.inner = self.inner.deployed(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn set_deployed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deployed(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn get_deployed(&self) -> &::std::option::Option<bool> {
        self.inner.get_deployed()
    }
}
