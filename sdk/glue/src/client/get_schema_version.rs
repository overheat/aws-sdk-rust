// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSchemaVersion`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_id(SchemaId)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::schema_id) / [`set_schema_id(Option<SchemaId>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_schema_id):<br>required: **false**<br><p>This is a wrapper structure to contain schema identity fields. The structure contains:</p>  <ul>   <li> <p>SchemaId$SchemaArn: The Amazon Resource Name (ARN) of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>   <li> <p>SchemaId$SchemaName: The name of the schema. Either <code>SchemaArn</code> or <code>SchemaName</code> and <code>RegistryName</code> has to be provided.</p> </li>  </ul><br>
    ///   - [`schema_version_id(impl Into<String>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::schema_version_id) / [`set_schema_version_id(Option<String>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_schema_version_id):<br>required: **false**<br><p>The <code>SchemaVersionId</code> of the schema version. This field is required for fetching by schema ID. Either this or the <code>SchemaId</code> wrapper has to be provided.</p><br>
    ///   - [`schema_version_number(SchemaVersionNumber)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::schema_version_number) / [`set_schema_version_number(Option<SchemaVersionNumber>)`](crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::set_schema_version_number):<br>required: **false**<br><p>The version number of the schema.</p><br>
    /// - On success, responds with [`GetSchemaVersionOutput`](crate::operation::get_schema_version::GetSchemaVersionOutput) with field(s):
    ///   - [`schema_version_id(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::schema_version_id): <p>The <code>SchemaVersionId</code> of the schema version.</p>
    ///   - [`schema_definition(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::schema_definition): <p>The schema definition for the schema ID.</p>
    ///   - [`data_format(Option<DataFormat>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::data_format): <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    ///   - [`schema_arn(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::schema_arn): <p>The Amazon Resource Name (ARN) of the schema.</p>
    ///   - [`version_number(Option<i64>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::version_number): <p>The version number of the schema.</p>
    ///   - [`status(Option<SchemaVersionStatus>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::status): <p>The status of the schema version. </p>
    ///   - [`created_time(Option<String>)`](crate::operation::get_schema_version::GetSchemaVersionOutput::created_time): <p>The date and time the schema version was created.</p>
    /// - On failure, responds with [`SdkError<GetSchemaVersionError>`](crate::operation::get_schema_version::GetSchemaVersionError)
    pub fn get_schema_version(&self) -> crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder {
        crate::operation::get_schema_version::builders::GetSchemaVersionFluentBuilder::new(self.handle.clone())
    }
}
