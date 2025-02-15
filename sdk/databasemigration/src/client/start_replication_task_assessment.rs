// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartReplicationTaskAssessment`](crate::operation::start_replication_task_assessment::builders::StartReplicationTaskAssessmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_task_arn(impl Into<String>)`](crate::operation::start_replication_task_assessment::builders::StartReplicationTaskAssessmentFluentBuilder::replication_task_arn) / [`set_replication_task_arn(Option<String>)`](crate::operation::start_replication_task_assessment::builders::StartReplicationTaskAssessmentFluentBuilder::set_replication_task_arn):<br>required: **true**<br><p> The Amazon Resource Name (ARN) of the replication task. </p><br>
    /// - On success, responds with [`StartReplicationTaskAssessmentOutput`](crate::operation::start_replication_task_assessment::StartReplicationTaskAssessmentOutput) with field(s):
    ///   - [`replication_task(Option<ReplicationTask>)`](crate::operation::start_replication_task_assessment::StartReplicationTaskAssessmentOutput::replication_task): <p> The assessed replication task. </p>
    /// - On failure, responds with [`SdkError<StartReplicationTaskAssessmentError>`](crate::operation::start_replication_task_assessment::StartReplicationTaskAssessmentError)
    pub fn start_replication_task_assessment(
        &self,
    ) -> crate::operation::start_replication_task_assessment::builders::StartReplicationTaskAssessmentFluentBuilder {
        crate::operation::start_replication_task_assessment::builders::StartReplicationTaskAssessmentFluentBuilder::new(self.handle.clone())
    }
}
