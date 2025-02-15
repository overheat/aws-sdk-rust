// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBillingGroups`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`billing_period(impl Into<String>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::billing_period) / [`set_billing_period(Option<String>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::set_billing_period):<br>required: **false**<br><p>The preferred billing period to get billing groups. </p><br>
    ///   - [`max_results(i32)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of billing groups to retrieve. </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token that's used on subsequent calls to get billing groups. </p><br>
    ///   - [`filters(ListBillingGroupsFilter)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::filters) / [`set_filters(Option<ListBillingGroupsFilter>)`](crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::set_filters):<br>required: **false**<br><p>A <code>ListBillingGroupsFilter</code> that specifies the billing group and pricing plan to retrieve billing group information. </p><br>
    /// - On success, responds with [`ListBillingGroupsOutput`](crate::operation::list_billing_groups::ListBillingGroupsOutput) with field(s):
    ///   - [`billing_groups(Option<Vec::<BillingGroupListElement>>)`](crate::operation::list_billing_groups::ListBillingGroupsOutput::billing_groups): <p>A list of <code>BillingGroupListElement</code> retrieved. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_billing_groups::ListBillingGroupsOutput::next_token): <p>The pagination token that's used on subsequent calls to get billing groups. </p>
    /// - On failure, responds with [`SdkError<ListBillingGroupsError>`](crate::operation::list_billing_groups::ListBillingGroupsError)
    pub fn list_billing_groups(&self) -> crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder {
        crate::operation::list_billing_groups::builders::ListBillingGroupsFluentBuilder::new(self.handle.clone())
    }
}
