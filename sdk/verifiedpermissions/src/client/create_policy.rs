// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePolicy`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_client_token):<br>required: **false**<br><p>Specifies a unique, case-sensitive ID that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>  <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>  <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p><br>
    ///   - [`policy_store_id(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::policy_store_id) / [`set_policy_store_id(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_policy_store_id):<br>required: **true**<br><p>Specifies the <code>PolicyStoreId</code> of the policy store you want to store the policy in.</p><br>
    ///   - [`definition(PolicyDefinition)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::definition) / [`set_definition(Option<PolicyDefinition>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_definition):<br>required: **true**<br><p>A structure that specifies the policy type and content to use for the new policy. You must include either a static or a templateLinked element. The policy content must be written in the Cedar policy language.</p><br>
    /// - On success, responds with [`CreatePolicyOutput`](crate::operation::create_policy::CreatePolicyOutput) with field(s):
    ///   - [`policy_store_id(String)`](crate::operation::create_policy::CreatePolicyOutput::policy_store_id): <p>The ID of the policy store that contains the new policy.</p>
    ///   - [`policy_id(String)`](crate::operation::create_policy::CreatePolicyOutput::policy_id): <p>The unique ID of the new policy.</p>
    ///   - [`policy_type(PolicyType)`](crate::operation::create_policy::CreatePolicyOutput::policy_type): <p>The policy type of the new policy.</p>
    ///   - [`principal(Option<EntityIdentifier>)`](crate::operation::create_policy::CreatePolicyOutput::principal): <p>The principal specified in the new policy's scope. This response element isn't present when <code>principal</code> isn't specified in the policy content.</p>
    ///   - [`resource(Option<EntityIdentifier>)`](crate::operation::create_policy::CreatePolicyOutput::resource): <p>The resource specified in the new policy's scope. This response element isn't present when the <code>resource</code> isn't specified in the policy content.</p>
    ///   - [`created_date(DateTime)`](crate::operation::create_policy::CreatePolicyOutput::created_date): <p>The date and time the policy was originally created.</p>
    ///   - [`last_updated_date(DateTime)`](crate::operation::create_policy::CreatePolicyOutput::last_updated_date): <p>The date and time the policy was last updated.</p>
    /// - On failure, responds with [`SdkError<CreatePolicyError>`](crate::operation::create_policy::CreatePolicyError)
    pub fn create_policy(&self) -> crate::operation::create_policy::builders::CreatePolicyFluentBuilder {
        crate::operation::create_policy::builders::CreatePolicyFluentBuilder::new(self.handle.clone())
    }
}
