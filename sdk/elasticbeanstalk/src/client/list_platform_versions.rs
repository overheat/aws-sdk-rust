// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPlatformVersions`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(PlatformFilter)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::filters) / [`set_filters(Option<Vec::<PlatformFilter>>)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::set_filters):<br>required: **false**<br><p>Criteria for restricting the resulting list of platform versions. The filter is interpreted as a logical conjunction (AND) of the separate <code>PlatformFilter</code> terms.</p><br>
    ///   - [`max_records(i32)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::set_max_records):<br>required: **false**<br><p>The maximum number of platform version values returned in one call.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::set_next_token):<br>required: **false**<br><p>For a paginated request. Specify a token from a previous response page to retrieve the next response page. All other parameter values must be identical to the ones specified in the initial request.</p>  <p>If no <code>NextToken</code> is specified, the first page is retrieved.</p><br>
    /// - On success, responds with [`ListPlatformVersionsOutput`](crate::operation::list_platform_versions::ListPlatformVersionsOutput) with field(s):
    ///   - [`platform_summary_list(Option<Vec::<PlatformSummary>>)`](crate::operation::list_platform_versions::ListPlatformVersionsOutput::platform_summary_list): <p>Summary information about the platform versions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_platform_versions::ListPlatformVersionsOutput::next_token): <p>In a paginated request, if this value isn't <code>null</code>, it's the token that you can pass in a subsequent request to get the next response page.</p>
    /// - On failure, responds with [`SdkError<ListPlatformVersionsError>`](crate::operation::list_platform_versions::ListPlatformVersionsError)
    pub fn list_platform_versions(&self) -> crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder {
        crate::operation::list_platform_versions::builders::ListPlatformVersionsFluentBuilder::new(self.handle.clone())
    }
}
