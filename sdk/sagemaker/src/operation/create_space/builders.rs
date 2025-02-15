// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_space::_create_space_output::CreateSpaceOutputBuilder;

pub use crate::operation::create_space::_create_space_input::CreateSpaceInputBuilder;

impl CreateSpaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_space::CreateSpaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_space::CreateSpaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_space();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSpace`.
///
/// <p>Creates a space used for real time collaboration in a Domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSpaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_space::builders::CreateSpaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_space::CreateSpaceOutput,
        crate::operation::create_space::CreateSpaceError,
    > for CreateSpaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_space::CreateSpaceOutput,
            crate::operation::create_space::CreateSpaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSpaceFluentBuilder {
    /// Creates a new `CreateSpace`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSpace as a reference.
    pub fn as_input(&self) -> &crate::operation::create_space::builders::CreateSpaceInputBuilder {
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
        crate::operation::create_space::CreateSpaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_space::CreateSpaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_space::CreateSpace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_space::CreateSpace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_space::CreateSpaceOutput,
        crate::operation::create_space::CreateSpaceError,
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
    /// <p>The ID of the associated Domain.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_id()
    }
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_name()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to associated with the space. Each tag consists of a key and an optional value. Tag keys must be unique for each resource. Tags are searchable using the <code>Search</code> API.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Tags to associated with the space. Each tag consists of a key and an optional value. Tag keys must be unique for each resource. Tags are searchable using the <code>Search</code> API.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags to associated with the space. Each tag consists of a key and an optional value. Tag keys must be unique for each resource. Tags are searchable using the <code>Search</code> API.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>A collection of space settings.</p>
    pub fn space_settings(mut self, input: crate::types::SpaceSettings) -> Self {
        self.inner = self.inner.space_settings(input);
        self
    }
    /// <p>A collection of space settings.</p>
    pub fn set_space_settings(mut self, input: ::std::option::Option<crate::types::SpaceSettings>) -> Self {
        self.inner = self.inner.set_space_settings(input);
        self
    }
    /// <p>A collection of space settings.</p>
    pub fn get_space_settings(&self) -> &::std::option::Option<crate::types::SpaceSettings> {
        self.inner.get_space_settings()
    }
}
