// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateLoggingConfiguration`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::set_workspace_id):<br>required: **true**<br>The ID of the workspace to vend logs to.<br>
    ///   - [`log_group_arn(impl Into<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::log_group_arn) / [`set_log_group_arn(Option<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::set_log_group_arn):<br>required: **true**<br>The ARN of the CW log group to which the vended log data will be published.<br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::set_client_token):<br>required: **false**<br>Optional, unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.<br>
    /// - On success, responds with [`UpdateLoggingConfigurationOutput`](crate::operation::update_logging_configuration::UpdateLoggingConfigurationOutput) with field(s):
    ///   - [`status(Option<LoggingConfigurationStatus>)`](crate::operation::update_logging_configuration::UpdateLoggingConfigurationOutput::status): The status of the logging configuration.
    /// - On failure, responds with [`SdkError<UpdateLoggingConfigurationError>`](crate::operation::update_logging_configuration::UpdateLoggingConfigurationError)
    pub fn update_logging_configuration(&self) -> crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder {
        crate::operation::update_logging_configuration::builders::UpdateLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
