// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetActionType`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`category(ActionCategory)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::category) / [`set_category(Option<ActionCategory>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::set_category):<br>required: **true**<br><p>Defines what kind of action can be taken in the stage. The following are the valid values:</p>  <ul>   <li> <p> <code>Source</code> </p> </li>   <li> <p> <code>Build</code> </p> </li>   <li> <p> <code>Test</code> </p> </li>   <li> <p> <code>Deploy</code> </p> </li>   <li> <p> <code>Approval</code> </p> </li>   <li> <p> <code>Invoke</code> </p> </li>  </ul><br>
    ///   - [`owner(impl Into<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::owner) / [`set_owner(Option<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::set_owner):<br>required: **true**<br><p>The creator of an action type that was created with any supported integration model. There are two valid values: <code>AWS</code> and <code>ThirdParty</code>.</p><br>
    ///   - [`provider(impl Into<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::provider) / [`set_provider(Option<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::set_provider):<br>required: **true**<br><p>The provider of the action type being called. The provider name is specified when the action type is created.</p><br>
    ///   - [`version(impl Into<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::version) / [`set_version(Option<String>)`](crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::set_version):<br>required: **true**<br><p>A string that describes the action type version.</p><br>
    /// - On success, responds with [`GetActionTypeOutput`](crate::operation::get_action_type::GetActionTypeOutput) with field(s):
    ///   - [`action_type(Option<ActionTypeDeclaration>)`](crate::operation::get_action_type::GetActionTypeOutput::action_type): <p>The action type information for the requested action type, such as the action type ID.</p>
    /// - On failure, responds with [`SdkError<GetActionTypeError>`](crate::operation::get_action_type::GetActionTypeError)
    pub fn get_action_type(&self) -> crate::operation::get_action_type::builders::GetActionTypeFluentBuilder {
        crate::operation::get_action_type::builders::GetActionTypeFluentBuilder::new(self.handle.clone())
    }
}
