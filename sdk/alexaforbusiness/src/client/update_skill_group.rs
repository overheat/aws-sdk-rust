// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSkillGroup`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`skill_group_arn(impl Into<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::skill_group_arn) / [`set_skill_group_arn(Option<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::set_skill_group_arn):<br>required: **false**<br><p>The ARN of the skill group to update. </p><br>
    ///   - [`skill_group_name(impl Into<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::skill_group_name) / [`set_skill_group_name(Option<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::set_skill_group_name):<br>required: **false**<br><p>The updated name for the skill group.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::set_description):<br>required: **false**<br><p>The updated description for the skill group.</p><br>
    /// - On success, responds with [`UpdateSkillGroupOutput`](crate::operation::update_skill_group::UpdateSkillGroupOutput)
    /// - On failure, responds with [`SdkError<UpdateSkillGroupError>`](crate::operation::update_skill_group::UpdateSkillGroupError)
    #[deprecated(note = "Alexa For Business is no longer supported")]
    pub fn update_skill_group(&self) -> crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder {
        crate::operation::update_skill_group::builders::UpdateSkillGroupFluentBuilder::new(self.handle.clone())
    }
}
