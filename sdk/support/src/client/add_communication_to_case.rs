// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddCommunicationToCase`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`case_id(impl Into<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::case_id) / [`set_case_id(Option<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::set_case_id):<br>required: **false**<br><p>The support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p><br>
    ///   - [`communication_body(impl Into<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::communication_body) / [`set_communication_body(Option<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::set_communication_body):<br>required: **true**<br><p>The body of an email communication to add to the support case.</p><br>
    ///   - [`cc_email_addresses(impl Into<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::cc_email_addresses) / [`set_cc_email_addresses(Option<Vec::<String>>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::set_cc_email_addresses):<br>required: **false**<br><p>The email addresses in the CC line of an email to be added to the support case.</p><br>
    ///   - [`attachment_set_id(impl Into<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::attachment_set_id) / [`set_attachment_set_id(Option<String>)`](crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::set_attachment_set_id):<br>required: **false**<br><p>The ID of a set of one or more attachments for the communication to add to the case. Create the set by calling <code>AddAttachmentsToSet</code> </p><br>
    /// - On success, responds with [`AddCommunicationToCaseOutput`](crate::operation::add_communication_to_case::AddCommunicationToCaseOutput) with field(s):
    ///   - [`result(bool)`](crate::operation::add_communication_to_case::AddCommunicationToCaseOutput::result): <p>True if <code>AddCommunicationToCase</code> succeeds. Otherwise, returns an error.</p>
    /// - On failure, responds with [`SdkError<AddCommunicationToCaseError>`](crate::operation::add_communication_to_case::AddCommunicationToCaseError)
    pub fn add_communication_to_case(&self) -> crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder {
        crate::operation::add_communication_to_case::builders::AddCommunicationToCaseFluentBuilder::new(self.handle.clone())
    }
}
