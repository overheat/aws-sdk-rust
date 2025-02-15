// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTags`](crate::operation::list_tags::builders::ListTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The stack or layer's Amazon Resource Number (ARN).</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::set_max_results):<br>required: **false**<br><p>Do not use. A validation exception occurs if you add a <code>MaxResults</code> parameter to a <code>ListTagsRequest</code> call. </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::set_next_token):<br>required: **false**<br><p>Do not use. A validation exception occurs if you add a <code>NextToken</code> parameter to a <code>ListTagsRequest</code> call. </p><br>
    /// - On success, responds with [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput) with field(s):
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::list_tags::ListTagsOutput::tags): <p>A set of key-value pairs that contain tag keys and tag values that are attached to a stack or layer.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags::ListTagsOutput::next_token): <p>If a paginated request does not return all of the remaining results, this parameter is set to a token that you can assign to the request object's <code>NextToken</code> parameter to get the next set of results. If the previous paginated request returned all of the remaining results, this parameter is set to <code>null</code>. </p>
    /// - On failure, responds with [`SdkError<ListTagsError>`](crate::operation::list_tags::ListTagsError)
    pub fn list_tags(&self) -> crate::operation::list_tags::builders::ListTagsFluentBuilder {
        crate::operation::list_tags::builders::ListTagsFluentBuilder::new(self.handle.clone())
    }
}
