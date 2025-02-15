// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListParts`](crate::operation::list_parts::builders::ListPartsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_parts::builders::ListPartsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::set_account_id):<br>required: **true**<br><p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p><br>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::set_vault_name):<br>required: **true**<br><p>The name of the vault.</p><br>
    ///   - [`upload_id(impl Into<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::set_upload_id):<br>required: **true**<br><p>The upload ID of the multipart upload.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::set_marker):<br>required: **false**<br><p>An opaque string used for pagination. This value specifies the part at which the listing of parts should begin. Get the marker value from the response of a previous List Parts response. You need only include the marker if you are continuing the pagination of results started in a previous List Parts request.</p><br>
    ///   - [`limit(i32)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_parts::builders::ListPartsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of parts to be returned. The default limit is 50. The number of parts returned might be fewer than the specified limit, but the number of returned parts never exceeds the limit.</p><br>
    /// - On success, responds with [`ListPartsOutput`](crate::operation::list_parts::ListPartsOutput) with field(s):
    ///   - [`multipart_upload_id(Option<String>)`](crate::operation::list_parts::ListPartsOutput::multipart_upload_id): <p>The ID of the upload to which the parts are associated.</p>
    ///   - [`vault_arn(Option<String>)`](crate::operation::list_parts::ListPartsOutput::vault_arn): <p>The Amazon Resource Name (ARN) of the vault to which the multipart upload was initiated.</p>
    ///   - [`archive_description(Option<String>)`](crate::operation::list_parts::ListPartsOutput::archive_description): <p>The description of the archive that was specified in the Initiate Multipart Upload request.</p>
    ///   - [`part_size_in_bytes(i64)`](crate::operation::list_parts::ListPartsOutput::part_size_in_bytes): <p>The part size in bytes. This is the same value that you specified in the Initiate Multipart Upload request.</p>
    ///   - [`creation_date(Option<String>)`](crate::operation::list_parts::ListPartsOutput::creation_date): <p>The UTC time at which the multipart upload was initiated.</p>
    ///   - [`parts(Option<Vec::<PartListElement>>)`](crate::operation::list_parts::ListPartsOutput::parts): <p>A list of the part sizes of the multipart upload. Each object in the array contains a <code>RangeBytes</code> and <code>sha256-tree-hash</code> name/value pair.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_parts::ListPartsOutput::marker): <p>An opaque string that represents where to continue pagination of the results. You use the marker in a new List Parts request to obtain more jobs in the list. If there are no more parts, this value is <code>null</code>.</p>
    /// - On failure, responds with [`SdkError<ListPartsError>`](crate::operation::list_parts::ListPartsError)
    pub fn list_parts(&self) -> crate::operation::list_parts::builders::ListPartsFluentBuilder {
        crate::operation::list_parts::builders::ListPartsFluentBuilder::new(self.handle.clone())
    }
}
