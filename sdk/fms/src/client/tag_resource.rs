// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the resource to return tags for. The Firewall Manager resources that support tagging are policies, applications lists, and protocols lists. </p><br>
    ///   - [`tag_list(Tag)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tag_list) / [`set_tag_list(Option<Vec::<Tag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tag_list):<br>required: **true**<br><p>The tags to add to the resource.</p><br>
    /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(&self) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.handle.clone())
    }
}
