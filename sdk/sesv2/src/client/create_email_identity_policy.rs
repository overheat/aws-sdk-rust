// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateEmailIdentityPolicy`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`email_identity(impl Into<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::email_identity) / [`set_email_identity(Option<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::set_email_identity):<br>required: **true**<br><p>The email identity.</p><br>
    ///   - [`policy_name(impl Into<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::set_policy_name):<br>required: **true**<br><p>The name of the policy.</p>  <p>The policy name cannot exceed 64 characters and can only include alphanumeric characters, dashes, and underscores.</p><br>
    ///   - [`policy(impl Into<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::set_policy):<br>required: **true**<br><p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p>  <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer Guide</a>.</p><br>
    /// - On success, responds with [`CreateEmailIdentityPolicyOutput`](crate::operation::create_email_identity_policy::CreateEmailIdentityPolicyOutput)
    /// - On failure, responds with [`SdkError<CreateEmailIdentityPolicyError>`](crate::operation::create_email_identity_policy::CreateEmailIdentityPolicyError)
    pub fn create_email_identity_policy(&self) -> crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder {
        crate::operation::create_email_identity_policy::builders::CreateEmailIdentityPolicyFluentBuilder::new(self.handle.clone())
    }
}
