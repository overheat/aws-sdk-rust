// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSchema`](crate::operation::get_schema::builders::GetSchemaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_store_id(impl Into<String>)`](crate::operation::get_schema::builders::GetSchemaFluentBuilder::policy_store_id) / [`set_policy_store_id(Option<String>)`](crate::operation::get_schema::builders::GetSchemaFluentBuilder::set_policy_store_id):<br>required: **true**<br><p>Specifies the ID of the policy store that contains the schema.</p><br>
    /// - On success, responds with [`GetSchemaOutput`](crate::operation::get_schema::GetSchemaOutput) with field(s):
    ///   - [`policy_store_id(String)`](crate::operation::get_schema::GetSchemaOutput::policy_store_id): <p>The ID of the policy store that contains the schema.</p>
    ///   - [`schema(String)`](crate::operation::get_schema::GetSchemaOutput::schema): <p>The body of the schema, written in Cedar schema JSON.</p>
    ///   - [`created_date(DateTime)`](crate::operation::get_schema::GetSchemaOutput::created_date): <p>The date and time that the schema was originally created.</p>
    ///   - [`last_updated_date(DateTime)`](crate::operation::get_schema::GetSchemaOutput::last_updated_date): <p>The date and time that the schema was most recently updated.</p>
    /// - On failure, responds with [`SdkError<GetSchemaError>`](crate::operation::get_schema::GetSchemaError)
    pub fn get_schema(&self) -> crate::operation::get_schema::builders::GetSchemaFluentBuilder {
        crate::operation::get_schema::builders::GetSchemaFluentBuilder::new(self.handle.clone())
    }
}
