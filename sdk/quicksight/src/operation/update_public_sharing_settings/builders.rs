// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_public_sharing_settings::_update_public_sharing_settings_output::UpdatePublicSharingSettingsOutputBuilder;

pub use crate::operation::update_public_sharing_settings::_update_public_sharing_settings_input::UpdatePublicSharingSettingsInputBuilder;

impl UpdatePublicSharingSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_public_sharing_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdatePublicSharingSettings`.
///
/// <p>Use the <code>UpdatePublicSharingSettings</code> operation to turn on or turn off the public sharing settings of an Amazon QuickSight dashboard.</p>
/// <p>To use this operation, turn on session capacity pricing for your Amazon QuickSight account.</p>
/// <p>Before you can turn on public sharing on your account, make sure to give public sharing permissions to an administrative user in the Identity and Access Management (IAM) console. For more information on using IAM with Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/security_iam_service-with-iam.html">Using Amazon QuickSight with IAM</a> in the <i>Amazon QuickSight User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePublicSharingSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_public_sharing_settings::builders::UpdatePublicSharingSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsOutput,
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsError,
    > for UpdatePublicSharingSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsOutput,
            crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdatePublicSharingSettingsFluentBuilder {
    /// Creates a new `UpdatePublicSharingSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdatePublicSharingSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_public_sharing_settings::builders::UpdatePublicSharingSettingsInputBuilder {
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
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_public_sharing_settings::UpdatePublicSharingSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsOutput,
        crate::operation::update_public_sharing_settings::UpdatePublicSharingSettingsError,
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
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight subscription.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight subscription.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID associated with your Amazon QuickSight subscription.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>A Boolean value that indicates whether public sharing is turned on for an Amazon QuickSight account.</p>
    pub fn public_sharing_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.public_sharing_enabled(input);
        self
    }
    /// <p>A Boolean value that indicates whether public sharing is turned on for an Amazon QuickSight account.</p>
    pub fn set_public_sharing_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_public_sharing_enabled(input);
        self
    }
    /// <p>A Boolean value that indicates whether public sharing is turned on for an Amazon QuickSight account.</p>
    pub fn get_public_sharing_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_public_sharing_enabled()
    }
}
