// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deprecate_domain::_deprecate_domain_output::DeprecateDomainOutputBuilder;

pub use crate::operation::deprecate_domain::_deprecate_domain_input::DeprecateDomainInputBuilder;

impl DeprecateDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deprecate_domain::DeprecateDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deprecate_domain::DeprecateDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deprecate_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeprecateDomain`.
///
/// <p>Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating a domain also deprecates all activity and workflow types registered in the domain. Executions that were started before the domain was deprecated continues to run.</p> <note>
/// <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p>
/// </note>
/// <p> <b>Access Control</b> </p>
/// <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p>
/// <ul>
/// <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li>
/// <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li>
/// <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li>
/// </ul>
/// <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeprecateDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deprecate_domain::builders::DeprecateDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deprecate_domain::DeprecateDomainOutput,
        crate::operation::deprecate_domain::DeprecateDomainError,
    > for DeprecateDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deprecate_domain::DeprecateDomainOutput,
            crate::operation::deprecate_domain::DeprecateDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeprecateDomainFluentBuilder {
    /// Creates a new `DeprecateDomain`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeprecateDomain as a reference.
    pub fn as_input(&self) -> &crate::operation::deprecate_domain::builders::DeprecateDomainInputBuilder {
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
        crate::operation::deprecate_domain::DeprecateDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deprecate_domain::DeprecateDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deprecate_domain::DeprecateDomain::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deprecate_domain::DeprecateDomain::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deprecate_domain::DeprecateDomainOutput,
        crate::operation::deprecate_domain::DeprecateDomainError,
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
    /// <p>The name of the domain to deprecate.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the domain to deprecate.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the domain to deprecate.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
