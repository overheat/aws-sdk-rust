// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PrepareQuery`](crate::operation::prepare_query::builders::PrepareQueryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`query_string(impl Into<String>)`](crate::operation::prepare_query::builders::PrepareQueryFluentBuilder::query_string) / [`set_query_string(Option<String>)`](crate::operation::prepare_query::builders::PrepareQueryFluentBuilder::set_query_string):<br>required: **true**<br><p>The Timestream query string that you want to use as a prepared statement. Parameter names can be specified in the query string <code>@</code> character followed by an identifier. </p><br>
    ///   - [`validate_only(bool)`](crate::operation::prepare_query::builders::PrepareQueryFluentBuilder::validate_only) / [`set_validate_only(Option<bool>)`](crate::operation::prepare_query::builders::PrepareQueryFluentBuilder::set_validate_only):<br>required: **false**<br><p>By setting this value to <code>true</code>, Timestream will only validate that the query string is a valid Timestream query, and not store the prepared query for later use.</p><br>
    /// - On success, responds with [`PrepareQueryOutput`](crate::operation::prepare_query::PrepareQueryOutput) with field(s):
    ///   - [`query_string(String)`](crate::operation::prepare_query::PrepareQueryOutput::query_string): <p>The query string that you want prepare.</p>
    ///   - [`columns(Vec::<SelectColumn>)`](crate::operation::prepare_query::PrepareQueryOutput::columns): <p>A list of SELECT clause columns of the submitted query string. </p>
    ///   - [`parameters(Vec::<ParameterMapping>)`](crate::operation::prepare_query::PrepareQueryOutput::parameters): <p>A list of parameters used in the submitted query string. </p>
    /// - On failure, responds with [`SdkError<PrepareQueryError>`](crate::operation::prepare_query::PrepareQueryError)
    pub fn prepare_query(&self) -> crate::operation::prepare_query::builders::PrepareQueryFluentBuilder {
        crate::operation::prepare_query::builders::PrepareQueryFluentBuilder::new(self.handle.clone())
    }
}
