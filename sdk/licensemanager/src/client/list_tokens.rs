// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTokens`](crate::operation::list_tokens::builders::ListTokensFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`token_ids(impl Into<String>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::token_ids) / [`set_token_ids(Option<Vec::<String>>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::set_token_ids):<br>required: **false**<br><p>Token IDs.</p><br>
    ///   - [`filters(Filter)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::set_filters):<br>required: **false**<br><p>Filters to scope the results. The following filter is supported:</p>  <ul>   <li> <p> <code>LicenseArns</code> </p> </li>  </ul><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::set_next_token):<br>required: **false**<br><p>Token for the next set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tokens::builders::ListTokensFluentBuilder::set_max_results):<br>required: **false**<br><p>Maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListTokensOutput`](crate::operation::list_tokens::ListTokensOutput) with field(s):
    ///   - [`tokens(Option<Vec::<TokenData>>)`](crate::operation::list_tokens::ListTokensOutput::tokens): <p>Received token details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tokens::ListTokensOutput::next_token): <p>Token for the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListTokensError>`](crate::operation::list_tokens::ListTokensError)
    pub fn list_tokens(&self) -> crate::operation::list_tokens::builders::ListTokensFluentBuilder {
        crate::operation::list_tokens::builders::ListTokensFluentBuilder::new(self.handle.clone())
    }
}
