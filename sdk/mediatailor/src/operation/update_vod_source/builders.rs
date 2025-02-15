// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_vod_source::_update_vod_source_output::UpdateVodSourceOutputBuilder;

pub use crate::operation::update_vod_source::_update_vod_source_input::UpdateVodSourceInputBuilder;

impl UpdateVodSourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_vod_source::UpdateVodSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_vod_source::UpdateVodSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_vod_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateVodSource`.
///
/// <p>Updates a VOD source's configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateVodSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_vod_source::builders::UpdateVodSourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_vod_source::UpdateVodSourceOutput,
        crate::operation::update_vod_source::UpdateVodSourceError,
    > for UpdateVodSourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_vod_source::UpdateVodSourceOutput,
            crate::operation::update_vod_source::UpdateVodSourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateVodSourceFluentBuilder {
    /// Creates a new `UpdateVodSource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateVodSource as a reference.
    pub fn as_input(&self) -> &crate::operation::update_vod_source::builders::UpdateVodSourceInputBuilder {
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
        crate::operation::update_vod_source::UpdateVodSourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_vod_source::UpdateVodSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_vod_source::UpdateVodSource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_vod_source::UpdateVodSource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_vod_source::UpdateVodSourceOutput,
        crate::operation::update_vod_source::UpdateVodSourceError,
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
    /// Appends an item to `HttpPackageConfigurations`.
    ///
    /// To override the contents of this collection use [`set_http_package_configurations`](Self::set_http_package_configurations).
    ///
    /// <p>A list of HTTP package configurations for the VOD source on this account.</p>
    pub fn http_package_configurations(mut self, input: crate::types::HttpPackageConfiguration) -> Self {
        self.inner = self.inner.http_package_configurations(input);
        self
    }
    /// <p>A list of HTTP package configurations for the VOD source on this account.</p>
    pub fn set_http_package_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::HttpPackageConfiguration>>) -> Self {
        self.inner = self.inner.set_http_package_configurations(input);
        self
    }
    /// <p>A list of HTTP package configurations for the VOD source on this account.</p>
    pub fn get_http_package_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::HttpPackageConfiguration>> {
        self.inner.get_http_package_configurations()
    }
    /// <p>The name of the source location associated with this VOD Source.</p>
    pub fn source_location_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_location_name(input.into());
        self
    }
    /// <p>The name of the source location associated with this VOD Source.</p>
    pub fn set_source_location_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_location_name(input);
        self
    }
    /// <p>The name of the source location associated with this VOD Source.</p>
    pub fn get_source_location_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_location_name()
    }
    /// <p>The name of the VOD source.</p>
    pub fn vod_source_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vod_source_name(input.into());
        self
    }
    /// <p>The name of the VOD source.</p>
    pub fn set_vod_source_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vod_source_name(input);
        self
    }
    /// <p>The name of the VOD source.</p>
    pub fn get_vod_source_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vod_source_name()
    }
}
