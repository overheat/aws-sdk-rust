// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAggregationAuthorization`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`authorized_account_id(impl Into<String>)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::authorized_account_id) / [`set_authorized_account_id(Option<String>)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::set_authorized_account_id):<br>required: **true**<br><p>The 12-digit account ID of the account authorized to aggregate data.</p><br>
    ///   - [`authorized_aws_region(impl Into<String>)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::authorized_aws_region) / [`set_authorized_aws_region(Option<String>)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::set_authorized_aws_region):<br>required: **true**<br><p>The region authorized to collect aggregated data.</p><br>
    ///   - [`tags(Tag)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::set_tags):<br>required: **false**<br><p>An array of tag object.</p><br>
    /// - On success, responds with [`PutAggregationAuthorizationOutput`](crate::operation::put_aggregation_authorization::PutAggregationAuthorizationOutput) with field(s):
    ///   - [`aggregation_authorization(Option<AggregationAuthorization>)`](crate::operation::put_aggregation_authorization::PutAggregationAuthorizationOutput::aggregation_authorization): <p>Returns an AggregationAuthorization object. </p>
    /// - On failure, responds with [`SdkError<PutAggregationAuthorizationError>`](crate::operation::put_aggregation_authorization::PutAggregationAuthorizationError)
    pub fn put_aggregation_authorization(
        &self,
    ) -> crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder {
        crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationFluentBuilder::new(self.handle.clone())
    }
}
