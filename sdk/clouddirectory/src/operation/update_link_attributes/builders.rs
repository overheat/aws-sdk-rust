// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_link_attributes::_update_link_attributes_output::UpdateLinkAttributesOutputBuilder;

pub use crate::operation::update_link_attributes::_update_link_attributes_input::UpdateLinkAttributesInputBuilder;

impl UpdateLinkAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_link_attributes::UpdateLinkAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_link_attributes::UpdateLinkAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_link_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLinkAttributes`.
///
/// <p>Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its <code>IdentityAttributeOrder</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLinkAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_link_attributes::builders::UpdateLinkAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_link_attributes::UpdateLinkAttributesOutput,
        crate::operation::update_link_attributes::UpdateLinkAttributesError,
    > for UpdateLinkAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_link_attributes::UpdateLinkAttributesOutput,
            crate::operation::update_link_attributes::UpdateLinkAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLinkAttributesFluentBuilder {
    /// Creates a new `UpdateLinkAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLinkAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::update_link_attributes::builders::UpdateLinkAttributesInputBuilder {
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
        crate::operation::update_link_attributes::UpdateLinkAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_link_attributes::UpdateLinkAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_link_attributes::UpdateLinkAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_link_attributes::UpdateLinkAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_link_attributes::UpdateLinkAttributesOutput,
        crate::operation::update_link_attributes::UpdateLinkAttributesError,
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
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn directory_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn set_directory_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <code>arns</code> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn get_directory_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_arn()
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn typed_link_specifier(mut self, input: crate::types::TypedLinkSpecifier) -> Self {
        self.inner = self.inner.typed_link_specifier(input);
        self
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn set_typed_link_specifier(mut self, input: ::std::option::Option<crate::types::TypedLinkSpecifier>) -> Self {
        self.inner = self.inner.set_typed_link_specifier(input);
        self
    }
    /// <p>Allows a typed link specifier to be accepted as input.</p>
    pub fn get_typed_link_specifier(&self) -> &::std::option::Option<crate::types::TypedLinkSpecifier> {
        self.inner.get_typed_link_specifier()
    }
    /// Appends an item to `AttributeUpdates`.
    ///
    /// To override the contents of this collection use [`set_attribute_updates`](Self::set_attribute_updates).
    ///
    /// <p>The attributes update structure.</p>
    pub fn attribute_updates(mut self, input: crate::types::LinkAttributeUpdate) -> Self {
        self.inner = self.inner.attribute_updates(input);
        self
    }
    /// <p>The attributes update structure.</p>
    pub fn set_attribute_updates(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LinkAttributeUpdate>>) -> Self {
        self.inner = self.inner.set_attribute_updates(input);
        self
    }
    /// <p>The attributes update structure.</p>
    pub fn get_attribute_updates(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LinkAttributeUpdate>> {
        self.inner.get_attribute_updates()
    }
}
