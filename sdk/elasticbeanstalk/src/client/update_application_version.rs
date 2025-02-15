// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateApplicationVersion`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::application_name) / [`set_application_name(Option<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::set_application_name):<br>required: **true**<br><p>The name of the application associated with this version.</p>  <p> If no application is found with this name, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error.</p><br>
    ///   - [`version_label(impl Into<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::version_label) / [`set_version_label(Option<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::set_version_label):<br>required: **true**<br><p>The name of the version to update.</p>  <p>If no application version is found with this label, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error. </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::set_description):<br>required: **false**<br><p>A new description for this version.</p><br>
    /// - On success, responds with [`UpdateApplicationVersionOutput`](crate::operation::update_application_version::UpdateApplicationVersionOutput) with field(s):
    ///   - [`application_version(Option<ApplicationVersionDescription>)`](crate::operation::update_application_version::UpdateApplicationVersionOutput::application_version): <p> The <code>ApplicationVersionDescription</code> of the application version. </p>
    /// - On failure, responds with [`SdkError<UpdateApplicationVersionError>`](crate::operation::update_application_version::UpdateApplicationVersionError)
    pub fn update_application_version(&self) -> crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder {
        crate::operation::update_application_version::builders::UpdateApplicationVersionFluentBuilder::new(self.handle.clone())
    }
}
