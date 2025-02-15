// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListReplicationSets`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results per page. </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token to continue to the next page of results.</p><br>
    /// - On success, responds with [`ListReplicationSetsOutput`](crate::operation::list_replication_sets::ListReplicationSetsOutput) with field(s):
    ///   - [`replication_set_arns(Vec::<String>)`](crate::operation::list_replication_sets::ListReplicationSetsOutput::replication_set_arns): <p>The Amazon Resource Name (ARN) of the list replication set.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_replication_sets::ListReplicationSetsOutput::next_token): <p>The pagination token to continue to the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListReplicationSetsError>`](crate::operation::list_replication_sets::ListReplicationSetsError)
    pub fn list_replication_sets(&self) -> crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder {
        crate::operation::list_replication_sets::builders::ListReplicationSetsFluentBuilder::new(self.handle.clone())
    }
}
