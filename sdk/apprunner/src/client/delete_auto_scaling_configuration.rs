// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAutoScalingConfiguration`](crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_scaling_configuration_arn(impl Into<String>)`](crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder::auto_scaling_configuration_arn) / [`set_auto_scaling_configuration_arn(Option<String>)`](crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder::set_auto_scaling_configuration_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the App Runner auto scaling configuration that you want to delete.</p>  <p>The ARN can be a full auto scaling configuration ARN, or a partial ARN ending with either <code>.../<i>name</i> </code> or <code>.../<i>name</i>/<i>revision</i> </code>. If a revision isn't specified, the latest active revision is deleted.</p><br>
    ///   - [`delete_all_revisions(bool)`](crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder::delete_all_revisions) / [`set_delete_all_revisions(Option<bool>)`](crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder::set_delete_all_revisions):<br>required: **false**<br><p>Set to <code>true</code> to delete all of the revisions associated with the <code>AutoScalingConfigurationArn</code> parameter value.</p>  <p>When <code>DeleteAllRevisions</code> is set to <code>true</code>, the only valid value for the Amazon Resource Name (ARN) is a partial ARN ending with: <code>.../name</code>.</p><br>
    /// - On success, responds with [`DeleteAutoScalingConfigurationOutput`](crate::operation::delete_auto_scaling_configuration::DeleteAutoScalingConfigurationOutput) with field(s):
    ///   - [`auto_scaling_configuration(Option<AutoScalingConfiguration>)`](crate::operation::delete_auto_scaling_configuration::DeleteAutoScalingConfigurationOutput::auto_scaling_configuration): <p>A description of the App Runner auto scaling configuration that this request just deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteAutoScalingConfigurationError>`](crate::operation::delete_auto_scaling_configuration::DeleteAutoScalingConfigurationError)
    pub fn delete_auto_scaling_configuration(
        &self,
    ) -> crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder {
        crate::operation::delete_auto_scaling_configuration::builders::DeleteAutoScalingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
