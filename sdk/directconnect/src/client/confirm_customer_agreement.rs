// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConfirmCustomerAgreement`](crate::operation::confirm_customer_agreement::builders::ConfirmCustomerAgreementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agreement_name(impl Into<String>)`](crate::operation::confirm_customer_agreement::builders::ConfirmCustomerAgreementFluentBuilder::agreement_name) / [`set_agreement_name(Option<String>)`](crate::operation::confirm_customer_agreement::builders::ConfirmCustomerAgreementFluentBuilder::set_agreement_name):<br>required: **false**<br><p> The name of the customer agreement. </p><br>
    /// - On success, responds with [`ConfirmCustomerAgreementOutput`](crate::operation::confirm_customer_agreement::ConfirmCustomerAgreementOutput) with field(s):
    ///   - [`status(Option<String>)`](crate::operation::confirm_customer_agreement::ConfirmCustomerAgreementOutput::status): <p> The status of the customer agreement when the connection was created. This will be either <code>signed</code> or <code>unsigned</code>. </p>
    /// - On failure, responds with [`SdkError<ConfirmCustomerAgreementError>`](crate::operation::confirm_customer_agreement::ConfirmCustomerAgreementError)
    pub fn confirm_customer_agreement(&self) -> crate::operation::confirm_customer_agreement::builders::ConfirmCustomerAgreementFluentBuilder {
        crate::operation::confirm_customer_agreement::builders::ConfirmCustomerAgreementFluentBuilder::new(self.handle.clone())
    }
}
