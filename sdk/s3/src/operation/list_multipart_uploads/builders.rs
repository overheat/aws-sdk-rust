// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_multipart_uploads::_list_multipart_uploads_output::ListMultipartUploadsOutputBuilder;

pub use crate::operation::list_multipart_uploads::_list_multipart_uploads_input::ListMultipartUploadsInputBuilder;

impl ListMultipartUploadsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_multipart_uploads::ListMultipartUploadsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_multipart_uploads::ListMultipartUploadsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_multipart_uploads();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListMultipartUploads`.
///
/// <p>This action lists in-progress multipart uploads. An in-progress multipart upload is a multipart upload that has been initiated using the Initiate Multipart Upload request, but has not yet been completed or aborted.</p>
/// <p>This action returns at most 1,000 multipart uploads in the response. 1,000 multipart uploads is the maximum number of uploads a response can include, which is also the default value. You can further limit the number of uploads in a response by specifying the <code>max-uploads</code> parameter in the response. If additional multipart uploads satisfy the list criteria, the response will contain an <code>IsTruncated</code> element with the value true. To list the additional multipart uploads, use the <code>key-marker</code> and <code>upload-id-marker</code> request parameters.</p>
/// <p>In the response, the uploads are sorted by key. If your application has initiated more than one multipart upload using the same object key, then uploads in the response are first sorted by key. Additionally, uploads are sorted in ascending order within each key by the upload initiation time.</p>
/// <p>For more information on multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart Upload</a>.</p>
/// <p>For information on permissions required to use the multipart upload API, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and Permissions</a>.</p>
/// <p>The following operations are related to <code>ListMultipartUploads</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMultipartUploadsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_multipart_uploads::builders::ListMultipartUploadsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_multipart_uploads::ListMultipartUploadsOutput,
        crate::operation::list_multipart_uploads::ListMultipartUploadsError,
    > for ListMultipartUploadsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_multipart_uploads::ListMultipartUploadsOutput,
            crate::operation::list_multipart_uploads::ListMultipartUploadsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListMultipartUploadsFluentBuilder {
    /// Creates a new `ListMultipartUploads`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListMultipartUploads as a reference.
    pub fn as_input(&self) -> &crate::operation::list_multipart_uploads::builders::ListMultipartUploadsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_multipart_uploads::ListMultipartUploadsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_multipart_uploads::ListMultipartUploadsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_multipart_uploads::ListMultipartUploads::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_multipart_uploads::ListMultipartUploads::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_multipart_uploads::ListMultipartUploadsOutput,
        crate::operation::list_multipart_uploads::ListMultipartUploadsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>Character you use to group keys.</p>
    /// <p>All keys that contain the same string between the prefix, if specified, and the first occurrence of the delimiter after the prefix are grouped under a single result element, <code>CommonPrefixes</code>. If you don't specify the prefix parameter, then the substring starts at the beginning of the key. The keys that are grouped under <code>CommonPrefixes</code> result element are not returned elsewhere in the response.</p>
    pub fn delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.delimiter(input.into());
        self
    }
    /// <p>Character you use to group keys.</p>
    /// <p>All keys that contain the same string between the prefix, if specified, and the first occurrence of the delimiter after the prefix are grouped under a single result element, <code>CommonPrefixes</code>. If you don't specify the prefix parameter, then the substring starts at the beginning of the key. The keys that are grouped under <code>CommonPrefixes</code> result element are not returned elsewhere in the response.</p>
    pub fn set_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_delimiter(input);
        self
    }
    /// <p>Character you use to group keys.</p>
    /// <p>All keys that contain the same string between the prefix, if specified, and the first occurrence of the delimiter after the prefix are grouped under a single result element, <code>CommonPrefixes</code>. If you don't specify the prefix parameter, then the substring starts at the beginning of the key. The keys that are grouped under <code>CommonPrefixes</code> result element are not returned elsewhere in the response.</p>
    pub fn get_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_delimiter()
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn encoding_type(mut self, input: crate::types::EncodingType) -> Self {
        self.inner = self.inner.encoding_type(input);
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn set_encoding_type(mut self, input: ::std::option::Option<crate::types::EncodingType>) -> Self {
        self.inner = self.inner.set_encoding_type(input);
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn get_encoding_type(&self) -> &::std::option::Option<crate::types::EncodingType> {
        self.inner.get_encoding_type()
    }
    /// <p>Together with <code>upload-id-marker</code>, this parameter specifies the multipart upload after which listing should begin.</p>
    /// <p>If <code>upload-id-marker</code> is not specified, only the keys lexicographically greater than the specified <code>key-marker</code> will be included in the list.</p>
    /// <p>If <code>upload-id-marker</code> is specified, any multipart uploads for a key equal to the <code>key-marker</code> might also be included, provided those multipart uploads have upload IDs lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_marker(input.into());
        self
    }
    /// <p>Together with <code>upload-id-marker</code>, this parameter specifies the multipart upload after which listing should begin.</p>
    /// <p>If <code>upload-id-marker</code> is not specified, only the keys lexicographically greater than the specified <code>key-marker</code> will be included in the list.</p>
    /// <p>If <code>upload-id-marker</code> is specified, any multipart uploads for a key equal to the <code>key-marker</code> might also be included, provided those multipart uploads have upload IDs lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn set_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_marker(input);
        self
    }
    /// <p>Together with <code>upload-id-marker</code>, this parameter specifies the multipart upload after which listing should begin.</p>
    /// <p>If <code>upload-id-marker</code> is not specified, only the keys lexicographically greater than the specified <code>key-marker</code> will be included in the list.</p>
    /// <p>If <code>upload-id-marker</code> is specified, any multipart uploads for a key equal to the <code>key-marker</code> might also be included, provided those multipart uploads have upload IDs lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn get_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_marker()
    }
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub fn max_uploads(mut self, input: i32) -> Self {
        self.inner = self.inner.max_uploads(input);
        self
    }
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub fn set_max_uploads(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_uploads(input);
        self
    }
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub fn get_max_uploads(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_uploads()
    }
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different grouping of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.)</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.prefix(input.into());
        self
    }
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different grouping of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.)</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_prefix(input);
        self
    }
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different grouping of keys. (You can think of using <code>prefix</code> to make groups in the same way that you'd use a folder in a file system.)</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_prefix()
    }
    /// <p>Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored. Otherwise, any multipart uploads for a key equal to the key-marker might be included in the list only if they have an upload ID lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn upload_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id_marker(input.into());
        self
    }
    /// <p>Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored. Otherwise, any multipart uploads for a key equal to the key-marker might be included in the list only if they have an upload ID lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn set_upload_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id_marker(input);
        self
    }
    /// <p>Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored. Otherwise, any multipart uploads for a key equal to the key-marker might be included in the list only if they have an upload ID lexicographically greater than the specified <code>upload-id-marker</code>.</p>
    pub fn get_upload_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_upload_id_marker()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination Amazon S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.inner = self.inner.request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination Amazon S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_request_payer(mut self, input: ::std::option::Option<crate::types::RequestPayer>) -> Self {
        self.inner = self.inner.set_request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. If either the source or destination Amazon S3 bucket has Requester Pays enabled, the requester will pay for corresponding charges to copy the object. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_request_payer(&self) -> &::std::option::Option<crate::types::RequestPayer> {
        self.inner.get_request_payer()
    }
}
