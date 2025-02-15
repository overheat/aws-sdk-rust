// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterWebhookWithThirdParty`](crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`webhook_name(impl Into<String>)`](crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyFluentBuilder::webhook_name) / [`set_webhook_name(Option<String>)`](crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyFluentBuilder::set_webhook_name):<br>required: **false**<br><p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p><br>
    /// - On success, responds with [`RegisterWebhookWithThirdPartyOutput`](crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput)
    /// - On failure, responds with [`SdkError<RegisterWebhookWithThirdPartyError>`](crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError)
    pub fn register_webhook_with_third_party(
        &self,
    ) -> crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyFluentBuilder {
        crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyFluentBuilder::new(self.handle.clone())
    }
}
