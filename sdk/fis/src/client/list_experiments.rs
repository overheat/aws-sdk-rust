// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListExperiments`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    /// - On success, responds with [`ListExperimentsOutput`](crate::operation::list_experiments::ListExperimentsOutput) with field(s):
    ///   - [`experiments(Option<Vec::<ExperimentSummary>>)`](crate::operation::list_experiments::ListExperimentsOutput::experiments): <p>The experiments.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_experiments::ListExperimentsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListExperimentsError>`](crate::operation::list_experiments::ListExperimentsError)
    pub fn list_experiments(&self) -> crate::operation::list_experiments::builders::ListExperimentsFluentBuilder {
        crate::operation::list_experiments::builders::ListExperimentsFluentBuilder::new(self.handle.clone())
    }
}
