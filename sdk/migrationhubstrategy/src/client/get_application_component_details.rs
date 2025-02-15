// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetApplicationComponentDetails`](crate::operation::get_application_component_details::builders::GetApplicationComponentDetailsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_component_id(impl Into<String>)`](crate::operation::get_application_component_details::builders::GetApplicationComponentDetailsFluentBuilder::application_component_id) / [`set_application_component_id(Option<String>)`](crate::operation::get_application_component_details::builders::GetApplicationComponentDetailsFluentBuilder::set_application_component_id):<br>required: **true**<br><p> The ID of the application component. The ID is unique within an AWS account.</p><br>
    /// - On success, responds with [`GetApplicationComponentDetailsOutput`](crate::operation::get_application_component_details::GetApplicationComponentDetailsOutput) with field(s):
    ///   - [`application_component_detail(Option<ApplicationComponentDetail>)`](crate::operation::get_application_component_details::GetApplicationComponentDetailsOutput::application_component_detail): <p> Detailed information about an application component. </p>
    ///   - [`associated_applications(Option<Vec::<AssociatedApplication>>)`](crate::operation::get_application_component_details::GetApplicationComponentDetailsOutput::associated_applications): <p> The associated application group as defined in AWS Application Discovery Service. </p>
    ///   - [`more_application_resource(Option<bool>)`](crate::operation::get_application_component_details::GetApplicationComponentDetailsOutput::more_application_resource): <p> Set to true if the application component belongs to more than one application group. </p>
    ///   - [`associated_server_ids(Option<Vec::<String>>)`](crate::operation::get_application_component_details::GetApplicationComponentDetailsOutput::associated_server_ids): <p> A list of the IDs of the servers on which the application component is running. </p>
    /// - On failure, responds with [`SdkError<GetApplicationComponentDetailsError>`](crate::operation::get_application_component_details::GetApplicationComponentDetailsError)
    pub fn get_application_component_details(
        &self,
    ) -> crate::operation::get_application_component_details::builders::GetApplicationComponentDetailsFluentBuilder {
        crate::operation::get_application_component_details::builders::GetApplicationComponentDetailsFluentBuilder::new(self.handle.clone())
    }
}
