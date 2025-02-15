// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateLifecyclePolicy`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_id(impl Into<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::policy_id) / [`set_policy_id(Option<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::set_policy_id):<br>required: **true**<br><p>The identifier of the lifecycle policy.</p><br>
    ///   - [`execution_role_arn(impl Into<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::set_execution_role_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p><br>
    ///   - [`state(SettablePolicyStateValues)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::state) / [`set_state(Option<SettablePolicyStateValues>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::set_state):<br>required: **false**<br><p>The desired activation state of the lifecycle policy after creation.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::set_description):<br>required: **false**<br><p>A description of the lifecycle policy.</p><br>
    ///   - [`policy_details(PolicyDetails)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::policy_details) / [`set_policy_details(Option<PolicyDetails>)`](crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::set_policy_details):<br>required: **false**<br><p>The configuration of the lifecycle policy. You cannot update the policy type or the resource type.</p><br>
    /// - On success, responds with [`UpdateLifecyclePolicyOutput`](crate::operation::update_lifecycle_policy::UpdateLifecyclePolicyOutput)
    /// - On failure, responds with [`SdkError<UpdateLifecyclePolicyError>`](crate::operation::update_lifecycle_policy::UpdateLifecyclePolicyError)
    pub fn update_lifecycle_policy(&self) -> crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder {
        crate::operation::update_lifecycle_policy::builders::UpdateLifecyclePolicyFluentBuilder::new(self.handle.clone())
    }
}
