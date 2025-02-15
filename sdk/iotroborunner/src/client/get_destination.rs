// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDestination`](crate::operation::get_destination::builders::GetDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::get_destination::builders::GetDestinationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_destination::builders::GetDestinationFluentBuilder::set_id):<br>required: **true**<br>Destination ARN.<br>
    /// - On success, responds with [`GetDestinationOutput`](crate::operation::get_destination::GetDestinationOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::get_destination::GetDestinationOutput::arn): Destination ARN.
    ///   - [`id(String)`](crate::operation::get_destination::GetDestinationOutput::id): Filters access by the destination's identifier
    ///   - [`name(String)`](crate::operation::get_destination::GetDestinationOutput::name): Human friendly name of the resource.
    ///   - [`site(String)`](crate::operation::get_destination::GetDestinationOutput::site): Site ARN.
    ///   - [`created_at(DateTime)`](crate::operation::get_destination::GetDestinationOutput::created_at): Timestamp at which the resource was created.
    ///   - [`updated_at(DateTime)`](crate::operation::get_destination::GetDestinationOutput::updated_at): Timestamp at which the resource was last updated.
    ///   - [`state(DestinationState)`](crate::operation::get_destination::GetDestinationOutput::state): State of the destination.
    ///   - [`additional_fixed_properties(Option<String>)`](crate::operation::get_destination::GetDestinationOutput::additional_fixed_properties): JSON document containing additional fixed properties regarding the destination
    /// - On failure, responds with [`SdkError<GetDestinationError>`](crate::operation::get_destination::GetDestinationError)
    pub fn get_destination(&self) -> crate::operation::get_destination::builders::GetDestinationFluentBuilder {
        crate::operation::get_destination::builders::GetDestinationFluentBuilder::new(self.handle.clone())
    }
}
