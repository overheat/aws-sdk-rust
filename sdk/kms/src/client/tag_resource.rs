// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a customer managed key in the account and Region.</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
    ///   - [`tags(Tag)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags):<br>required: **true**<br><p>One or more tags. Each tag consists of a tag key and a tag value. The tag value can be an empty (null) string. </p> <important>   <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>  </important>  <p>You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p><br>
    /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(&self) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.handle.clone())
    }
}
