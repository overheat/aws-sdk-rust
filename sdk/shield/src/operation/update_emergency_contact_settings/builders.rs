// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_emergency_contact_settings::_update_emergency_contact_settings_output::UpdateEmergencyContactSettingsOutputBuilder;

pub use crate::operation::update_emergency_contact_settings::_update_emergency_contact_settings_input::UpdateEmergencyContactSettingsInputBuilder;

impl UpdateEmergencyContactSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_emergency_contact_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateEmergencyContactSettings`.
///
/// <p>Updates the details of the list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEmergencyContactSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_emergency_contact_settings::builders::UpdateEmergencyContactSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsOutput,
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsError,
    > for UpdateEmergencyContactSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsOutput,
            crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateEmergencyContactSettingsFluentBuilder {
    /// Creates a new `UpdateEmergencyContactSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateEmergencyContactSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_emergency_contact_settings::builders::UpdateEmergencyContactSettingsInputBuilder {
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
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsOutput,
        crate::operation::update_emergency_contact_settings::UpdateEmergencyContactSettingsError,
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
    /// Appends an item to `EmergencyContactList`.
    ///
    /// To override the contents of this collection use [`set_emergency_contact_list`](Self::set_emergency_contact_list).
    ///
    /// <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
    /// <p>If you have proactive engagement enabled, the contact list must include at least one phone number.</p>
    pub fn emergency_contact_list(mut self, input: crate::types::EmergencyContact) -> Self {
        self.inner = self.inner.emergency_contact_list(input);
        self
    }
    /// <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
    /// <p>If you have proactive engagement enabled, the contact list must include at least one phone number.</p>
    pub fn set_emergency_contact_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EmergencyContact>>) -> Self {
        self.inner = self.inner.set_emergency_contact_list(input);
        self
    }
    /// <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
    /// <p>If you have proactive engagement enabled, the contact list must include at least one phone number.</p>
    pub fn get_emergency_contact_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EmergencyContact>> {
        self.inner.get_emergency_contact_list()
    }
}
