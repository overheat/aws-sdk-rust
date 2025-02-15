// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutStoredQuery`](crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stored_query(StoredQuery)`](crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder::stored_query) / [`set_stored_query(Option<StoredQuery>)`](crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder::set_stored_query):<br>required: **true**<br><p>A list of <code>StoredQuery</code> objects. The mandatory fields are <code>QueryName</code> and <code>Expression</code>.</p> <note>   <p>When you are creating a query, you must provide a query name and an expression. When you are updating a query, you must provide a query name but updating the description is optional.</p>  </note><br>
    ///   - [`tags(Tag)`](crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder::set_tags):<br>required: **false**<br><p>A list of <code>Tags</code> object.</p><br>
    /// - On success, responds with [`PutStoredQueryOutput`](crate::operation::put_stored_query::PutStoredQueryOutput) with field(s):
    ///   - [`query_arn(Option<String>)`](crate::operation::put_stored_query::PutStoredQueryOutput::query_arn): <p>Amazon Resource Name (ARN) of the query. For example, arn:partition:service:region:account-id:resource-type/resource-name/resource-id.</p>
    /// - On failure, responds with [`SdkError<PutStoredQueryError>`](crate::operation::put_stored_query::PutStoredQueryError)
    pub fn put_stored_query(&self) -> crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder {
        crate::operation::put_stored_query::builders::PutStoredQueryFluentBuilder::new(self.handle.clone())
    }
}
