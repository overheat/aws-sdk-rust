// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetUpgradeHistory`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::set_domain_name):<br>required: **true**<br><p>The name of an existing domain.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::set_next_token):<br>required: **false**<br><p>If your initial <code>GetUpgradeHistory</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>GetUpgradeHistory</code> operations, which returns results in the next page.</p><br>
    /// - On success, responds with [`GetUpgradeHistoryOutput`](crate::operation::get_upgrade_history::GetUpgradeHistoryOutput) with field(s):
    ///   - [`upgrade_histories(Option<Vec::<UpgradeHistory>>)`](crate::operation::get_upgrade_history::GetUpgradeHistoryOutput::upgrade_histories): <p>A list of objects corresponding to each upgrade or upgrade eligibility check performed on a domain.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_upgrade_history::GetUpgradeHistoryOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<GetUpgradeHistoryError>`](crate::operation::get_upgrade_history::GetUpgradeHistoryError)
    pub fn get_upgrade_history(&self) -> crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder {
        crate::operation::get_upgrade_history::builders::GetUpgradeHistoryFluentBuilder::new(self.handle.clone())
    }
}
