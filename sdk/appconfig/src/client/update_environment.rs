// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateEnvironment`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_application_id):<br>required: **true**<br><p>The application ID.</p><br>
    ///   - [`environment_id(impl Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::environment_id) / [`set_environment_id(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_environment_id):<br>required: **true**<br><p>The environment ID.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_name):<br>required: **false**<br><p>The name of the environment.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_description):<br>required: **false**<br><p>A description of the environment.</p><br>
    ///   - [`monitors(Monitor)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::monitors) / [`set_monitors(Option<Vec::<Monitor>>)`](crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::set_monitors):<br>required: **false**<br><p>Amazon CloudWatch alarms to monitor during the deployment process.</p><br>
    /// - On success, responds with [`UpdateEnvironmentOutput`](crate::operation::update_environment::UpdateEnvironmentOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::operation::update_environment::UpdateEnvironmentOutput::application_id): <p>The application ID.</p>
    ///   - [`id(Option<String>)`](crate::operation::update_environment::UpdateEnvironmentOutput::id): <p>The environment ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_environment::UpdateEnvironmentOutput::name): <p>The name of the environment.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_environment::UpdateEnvironmentOutput::description): <p>The description of the environment.</p>
    ///   - [`state(Option<EnvironmentState>)`](crate::operation::update_environment::UpdateEnvironmentOutput::state): <p>The state of the environment. An environment can be in one of the following states: <code>READY_FOR_DEPLOYMENT</code>, <code>DEPLOYING</code>, <code>ROLLING_BACK</code>, or <code>ROLLED_BACK</code> </p>
    ///   - [`monitors(Option<Vec::<Monitor>>)`](crate::operation::update_environment::UpdateEnvironmentOutput::monitors): <p>Amazon CloudWatch alarms monitored during the deployment.</p>
    /// - On failure, responds with [`SdkError<UpdateEnvironmentError>`](crate::operation::update_environment::UpdateEnvironmentError)
    pub fn update_environment(&self) -> crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder {
        crate::operation::update_environment::builders::UpdateEnvironmentFluentBuilder::new(self.handle.clone())
    }
}
