// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSchedule`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::set_name):<br>required: **true**<br><p>The name of the schedule to delete.</p><br>
    ///   - [`group_name(impl Into<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::set_group_name):<br>required: **false**<br><p>The name of the schedule group associated with this schedule. If you omit this, the default schedule group is used.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::set_client_token):<br>required: **false**<br><p> Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, EventBridge Scheduler uses a randomly generated token for the request to ensure idempotency. </p><br>
    /// - On success, responds with [`DeleteScheduleOutput`](crate::operation::delete_schedule::DeleteScheduleOutput)
    /// - On failure, responds with [`SdkError<DeleteScheduleError>`](crate::operation::delete_schedule::DeleteScheduleError)
    pub fn delete_schedule(&self) -> crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder {
        crate::operation::delete_schedule::builders::DeleteScheduleFluentBuilder::new(self.handle.clone())
    }
}
