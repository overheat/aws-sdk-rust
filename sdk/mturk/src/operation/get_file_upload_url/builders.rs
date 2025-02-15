// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_file_upload_url::_get_file_upload_url_output::GetFileUploadUrlOutputBuilder;

pub use crate::operation::get_file_upload_url::_get_file_upload_url_input::GetFileUploadUrlInputBuilder;

impl GetFileUploadUrlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_file_upload_url::GetFileUploadUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_file_upload_url::GetFileUploadURLError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_file_upload_url();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFileUploadURL`.
///
/// <p> The <code>GetFileUploadURL</code> operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is generated the instant the GetFileUploadURL operation is called, and is valid for 60 seconds. You can get a temporary file upload URL any time until the HIT is disposed. After the HIT is disposed, any uploaded files are deleted, and cannot be retrieved. Pending Deprecation on December 12, 2017. The Answer Specification structure will no longer support the <code>FileUploadAnswer</code> element to be used for the QuestionForm data structure. Instead, we recommend that Requesters who want to create HITs asking Workers to upload files to use Amazon S3. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFileUploadURLFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_file_upload_url::builders::GetFileUploadUrlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_file_upload_url::GetFileUploadUrlOutput,
        crate::operation::get_file_upload_url::GetFileUploadURLError,
    > for GetFileUploadURLFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_file_upload_url::GetFileUploadUrlOutput,
            crate::operation::get_file_upload_url::GetFileUploadURLError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFileUploadURLFluentBuilder {
    /// Creates a new `GetFileUploadURL`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFileUploadURL as a reference.
    pub fn as_input(&self) -> &crate::operation::get_file_upload_url::builders::GetFileUploadUrlInputBuilder {
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
        crate::operation::get_file_upload_url::GetFileUploadUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_file_upload_url::GetFileUploadURLError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_file_upload_url::GetFileUploadURL::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_file_upload_url::GetFileUploadURL::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_file_upload_url::GetFileUploadUrlOutput,
        crate::operation::get_file_upload_url::GetFileUploadURLError,
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
    /// <p>The ID of the assignment that contains the question with a FileUploadAnswer.</p>
    pub fn assignment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assignment_id(input.into());
        self
    }
    /// <p>The ID of the assignment that contains the question with a FileUploadAnswer.</p>
    pub fn set_assignment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assignment_id(input);
        self
    }
    /// <p>The ID of the assignment that contains the question with a FileUploadAnswer.</p>
    pub fn get_assignment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assignment_id()
    }
    /// <p>The identifier of the question with a FileUploadAnswer, as specified in the QuestionForm of the HIT.</p>
    pub fn question_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.question_identifier(input.into());
        self
    }
    /// <p>The identifier of the question with a FileUploadAnswer, as specified in the QuestionForm of the HIT.</p>
    pub fn set_question_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_question_identifier(input);
        self
    }
    /// <p>The identifier of the question with a FileUploadAnswer, as specified in the QuestionForm of the HIT.</p>
    pub fn get_question_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_question_identifier()
    }
}
