// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeUserGroups`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_group_id(impl Into<String>)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::user_group_id) / [`set_user_group_id(Option<String>)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::set_user_group_id):<br>required: **false**<br><p>The ID of the user group.</p><br>
    ///   - [`max_records(i32)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::set_max_records):<br>required: **false**<br><p>The maximum number of records to include in the response. If more records exist than the specified MaxRecords value, a marker is included in the response so that the remaining results can be retrieved. </p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::set_marker):<br>required: **false**<br><p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p><br>
    /// - On success, responds with [`DescribeUserGroupsOutput`](crate::operation::describe_user_groups::DescribeUserGroupsOutput) with field(s):
    ///   - [`user_groups(Option<Vec::<UserGroup>>)`](crate::operation::describe_user_groups::DescribeUserGroupsOutput::user_groups): <p>Returns a list of user groups.</p>
    ///   - [`marker(Option<String>)`](crate::operation::describe_user_groups::DescribeUserGroupsOutput::marker): <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    /// - On failure, responds with [`SdkError<DescribeUserGroupsError>`](crate::operation::describe_user_groups::DescribeUserGroupsError)
    pub fn describe_user_groups(&self) -> crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder {
        crate::operation::describe_user_groups::builders::DescribeUserGroupsFluentBuilder::new(self.handle.clone())
    }
}
