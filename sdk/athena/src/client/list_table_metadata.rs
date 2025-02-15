// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTableMetadata`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_name(impl Into<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::catalog_name) / [`set_catalog_name(Option<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::set_catalog_name):<br>required: **true**<br><p>The name of the data catalog for which table metadata should be returned.</p><br>
    ///   - [`database_name(impl Into<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::set_database_name):<br>required: **true**<br><p>The name of the database for which table metadata should be returned.</p><br>
    ///   - [`expression(impl Into<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::expression) / [`set_expression(Option<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::set_expression):<br>required: **false**<br><p>A regex filter that pattern-matches table names. If no expression is supplied, metadata for all tables are listed.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::set_next_token):<br>required: **false**<br><p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::set_max_results):<br>required: **false**<br><p>Specifies the maximum number of results to return.</p><br>
    /// - On success, responds with [`ListTableMetadataOutput`](crate::operation::list_table_metadata::ListTableMetadataOutput) with field(s):
    ///   - [`table_metadata_list(Option<Vec::<TableMetadata>>)`](crate::operation::list_table_metadata::ListTableMetadataOutput::table_metadata_list): <p>A list of table metadata.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_table_metadata::ListTableMetadataOutput::next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    /// - On failure, responds with [`SdkError<ListTableMetadataError>`](crate::operation::list_table_metadata::ListTableMetadataError)
    pub fn list_table_metadata(&self) -> crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder {
        crate::operation::list_table_metadata::builders::ListTableMetadataFluentBuilder::new(self.handle.clone())
    }
}
