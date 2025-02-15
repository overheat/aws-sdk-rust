// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateProject`](crate::operation::update_project::builders::UpdateProjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contents(Blob)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::contents) / [`set_contents(Option<Blob>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::set_contents):<br>required: **false**<br><p> ZIP or YAML file which contains project configuration to be updated. This should be the contents of the file downloaded from the URL provided in an export project operation. </p><br>
    ///   - [`project_id(impl Into<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::project_id) / [`set_project_id(Option<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::set_project_id):<br>required: **true**<br><p> Unique project identifier. </p><br>
    /// - On success, responds with [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput) with field(s):
    ///   - [`details(Option<ProjectDetails>)`](crate::operation::update_project::UpdateProjectOutput::details): <p> Detailed information about the updated AWS Mobile Hub project. </p>
    /// - On failure, responds with [`SdkError<UpdateProjectError>`](crate::operation::update_project::UpdateProjectError)
    pub fn update_project(&self) -> crate::operation::update_project::builders::UpdateProjectFluentBuilder {
        crate::operation::update_project::builders::UpdateProjectFluentBuilder::new(self.handle.clone())
    }
}
