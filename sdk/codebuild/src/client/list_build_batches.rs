// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBuildBatches`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filter(BuildBatchFilter)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::filter) / [`set_filter(Option<BuildBatchFilter>)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::set_filter):<br>required: **false**<br><p>A <code>BuildBatchFilter</code> object that specifies the filters for the search.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return.</p><br>
    ///   - [`sort_order(SortOrderType)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::sort_order) / [`set_sort_order(Option<SortOrderType>)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::set_sort_order):<br>required: **false**<br><p>Specifies the sort order of the returned items. Valid values include:</p>  <ul>   <li> <p> <code>ASCENDING</code>: List the batch build identifiers in ascending order by identifier.</p> </li>   <li> <p> <code>DESCENDING</code>: List the batch build identifiers in descending order by identifier.</p> </li>  </ul><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>nextToken</code> value returned from a previous call to <code>ListBuildBatches</code>. This specifies the next item to return. To return the beginning of the list, exclude this parameter.</p><br>
    /// - On success, responds with [`ListBuildBatchesOutput`](crate::operation::list_build_batches::ListBuildBatchesOutput) with field(s):
    ///   - [`ids(Option<Vec::<String>>)`](crate::operation::list_build_batches::ListBuildBatchesOutput::ids): <p>An array of strings that contains the batch build identifiers.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_build_batches::ListBuildBatchesOutput::next_token): <p>If there are more items to return, this contains a token that is passed to a subsequent call to <code>ListBuildBatches</code> to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<ListBuildBatchesError>`](crate::operation::list_build_batches::ListBuildBatchesError)
    pub fn list_build_batches(&self) -> crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder {
        crate::operation::list_build_batches::builders::ListBuildBatchesFluentBuilder::new(self.handle.clone())
    }
}
