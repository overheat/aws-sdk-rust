// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutLaunchAction`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_resource_id):<br>required: **true**<br><p>Launch configuration template Id or Source Server Id</p><br>
    ///   - [`action_code(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::action_code) / [`set_action_code(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_action_code):<br>required: **true**<br><p>Launch action code.</p><br>
    ///   - [`order(i32)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::order) / [`set_order(Option<i32>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_order):<br>required: **true**<br><p>Launch action order.</p><br>
    ///   - [`action_id(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::action_id) / [`set_action_id(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_action_id):<br>required: **true**<br><p>Launch action Id.</p><br>
    ///   - [`optional(bool)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::optional) / [`set_optional(Option<bool>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_optional):<br>required: **true**<br><p>Whether the launch will not be marked as failed if this action fails.</p><br>
    ///   - [`active(bool)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::active) / [`set_active(Option<bool>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_active):<br>required: **true**<br><p>Whether the launch action is active.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_name):<br>required: **true**<br><p>Launch action name.</p><br>
    ///   - [`action_version(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::action_version) / [`set_action_version(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_action_version):<br>required: **true**<br><p>Launch action version.</p><br>
    ///   - [`category(LaunchActionCategory)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::category) / [`set_category(Option<LaunchActionCategory>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_category):<br>required: **true**<br><p>Launch action category.</p><br>
    ///   - [`parameters(impl Into<String>, LaunchActionParameter)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::parameters) / [`set_parameters(Option<HashMap::<String, LaunchActionParameter>>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_parameters):<br>required: **false**<br><p>Launch action parameters.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::set_description):<br>required: **true**<br><p>Launch action description.</p><br>
    /// - On success, responds with [`PutLaunchActionOutput`](crate::operation::put_launch_action::PutLaunchActionOutput) with field(s):
    ///   - [`resource_id(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::resource_id): <p>Launch configuration template Id or Source Server Id</p>
    ///   - [`action_id(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::action_id): <p>Launch action Id.</p>
    ///   - [`action_code(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::action_code): <p>Launch action code.</p>
    ///   - [`r#type(Option<LaunchActionType>)`](crate::operation::put_launch_action::PutLaunchActionOutput::type): <p>Launch action type.</p>
    ///   - [`name(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::name): <p>Launch action name.</p>
    ///   - [`active(Option<bool>)`](crate::operation::put_launch_action::PutLaunchActionOutput::active): <p>Whether the launch action is active.</p>
    ///   - [`order(i32)`](crate::operation::put_launch_action::PutLaunchActionOutput::order): <p>Launch action order.</p>
    ///   - [`action_version(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::action_version): <p>Launch action version.</p>
    ///   - [`optional(Option<bool>)`](crate::operation::put_launch_action::PutLaunchActionOutput::optional): <p>Whether the launch will not be marked as failed if this action fails.</p>
    ///   - [`parameters(Option<HashMap::<String, LaunchActionParameter>>)`](crate::operation::put_launch_action::PutLaunchActionOutput::parameters): <p>Launch action parameters.</p>
    ///   - [`description(Option<String>)`](crate::operation::put_launch_action::PutLaunchActionOutput::description): <p>Launch action description.</p>
    ///   - [`category(Option<LaunchActionCategory>)`](crate::operation::put_launch_action::PutLaunchActionOutput::category): <p>Launch action category.</p>
    /// - On failure, responds with [`SdkError<PutLaunchActionError>`](crate::operation::put_launch_action::PutLaunchActionError)
    pub fn put_launch_action(&self) -> crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder {
        crate::operation::put_launch_action::builders::PutLaunchActionFluentBuilder::new(self.handle.clone())
    }
}
