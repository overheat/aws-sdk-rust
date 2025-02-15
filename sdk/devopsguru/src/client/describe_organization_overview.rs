// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeOrganizationOverview`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`from_time(DateTime)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::from_time) / [`set_from_time(Option<DateTime>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::set_from_time):<br>required: **true**<br><p> The start of the time range passed in. The start time granularity is at the day level. The floor of the start time is used. Returned information occurred after this day. </p><br>
    ///   - [`to_time(DateTime)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::to_time) / [`set_to_time(Option<DateTime>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::set_to_time):<br>required: **false**<br><p> The end of the time range passed in. The start time granularity is at the day level. The floor of the start time is used. Returned information occurred before this day. If this is not specified, then the current day is used. </p><br>
    ///   - [`account_ids(impl Into<String>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::account_ids) / [`set_account_ids(Option<Vec::<String>>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::set_account_ids):<br>required: **false**<br><p>The ID of the Amazon Web Services account.</p><br>
    ///   - [`organizational_unit_ids(impl Into<String>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::organizational_unit_ids) / [`set_organizational_unit_ids(Option<Vec::<String>>)`](crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::set_organizational_unit_ids):<br>required: **false**<br><p>The ID of the organizational unit.</p><br>
    /// - On success, responds with [`DescribeOrganizationOverviewOutput`](crate::operation::describe_organization_overview::DescribeOrganizationOverviewOutput) with field(s):
    ///   - [`reactive_insights(i32)`](crate::operation::describe_organization_overview::DescribeOrganizationOverviewOutput::reactive_insights): <p>An integer that specifies the number of open reactive insights in your Amazon Web Services account.</p>
    ///   - [`proactive_insights(i32)`](crate::operation::describe_organization_overview::DescribeOrganizationOverviewOutput::proactive_insights): <p>An integer that specifies the number of open proactive insights in your Amazon Web Services account.</p>
    /// - On failure, responds with [`SdkError<DescribeOrganizationOverviewError>`](crate::operation::describe_organization_overview::DescribeOrganizationOverviewError)
    pub fn describe_organization_overview(
        &self,
    ) -> crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder {
        crate::operation::describe_organization_overview::builders::DescribeOrganizationOverviewFluentBuilder::new(self.handle.clone())
    }
}
