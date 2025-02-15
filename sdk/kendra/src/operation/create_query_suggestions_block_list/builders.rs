// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_query_suggestions_block_list::_create_query_suggestions_block_list_output::CreateQuerySuggestionsBlockListOutputBuilder;

pub use crate::operation::create_query_suggestions_block_list::_create_query_suggestions_block_list_input::CreateQuerySuggestionsBlockListInputBuilder;

impl CreateQuerySuggestionsBlockListInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_query_suggestions_block_list();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateQuerySuggestionsBlockList`.
///
/// <p>Creates a block list to exlcude certain queries from suggestions.</p>
/// <p>Any query that contains words or phrases specified in the block list is blocked or filtered out from being shown as a suggestion.</p>
/// <p>You need to provide the file location of your block list text file in your S3 bucket. In your text file, enter each block word or phrase on a separate line.</p>
/// <p>For information on the current quota limits for block lists, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas for Amazon Kendra</a>.</p>
/// <p> <code>CreateQuerySuggestionsBlockList</code> is currently not supported in the Amazon Web Services GovCloud (US-West) region.</p>
/// <p>For an example of creating a block list for query suggestions using the Python SDK, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/query-suggestions.html#query-suggestions-blocklist">Query suggestions block list</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateQuerySuggestionsBlockListFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_query_suggestions_block_list::builders::CreateQuerySuggestionsBlockListInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListOutput,
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListError,
    > for CreateQuerySuggestionsBlockListFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListOutput,
            crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateQuerySuggestionsBlockListFluentBuilder {
    /// Creates a new `CreateQuerySuggestionsBlockList`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateQuerySuggestionsBlockList as a reference.
    pub fn as_input(&self) -> &crate::operation::create_query_suggestions_block_list::builders::CreateQuerySuggestionsBlockListInputBuilder {
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
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockList::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockList::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListOutput,
        crate::operation::create_query_suggestions_block_list::CreateQuerySuggestionsBlockListError,
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
    /// <p>The identifier of the index you want to create a query suggestions block list for.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_id(input.into());
        self
    }
    /// <p>The identifier of the index you want to create a query suggestions block list for.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_id(input);
        self
    }
    /// <p>The identifier of the index you want to create a query suggestions block list for.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_id()
    }
    /// <p>A name for the block list.</p>
    /// <p>For example, the name 'offensive-words', which includes all offensive words that could appear in user queries and need to be blocked from suggestions.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the block list.</p>
    /// <p>For example, the name 'offensive-words', which includes all offensive words that could appear in user queries and need to be blocked from suggestions.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the block list.</p>
    /// <p>For example, the name 'offensive-words', which includes all offensive words that could appear in user queries and need to be blocked from suggestions.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description for the block list.</p>
    /// <p>For example, the description "List of all offensive words that can appear in user queries and need to be blocked from suggestions."</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the block list.</p>
    /// <p>For example, the description "List of all offensive words that can appear in user queries and need to be blocked from suggestions."</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the block list.</p>
    /// <p>For example, the description "List of all offensive words that can appear in user queries and need to be blocked from suggestions."</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The S3 path to your block list text file in your S3 bucket.</p>
    /// <p>Each block word or phrase should be on a separate line in a text file.</p>
    /// <p>For information on the current quota limits for block lists, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas for Amazon Kendra</a>.</p>
    pub fn source_s3_path(mut self, input: crate::types::S3Path) -> Self {
        self.inner = self.inner.source_s3_path(input);
        self
    }
    /// <p>The S3 path to your block list text file in your S3 bucket.</p>
    /// <p>Each block word or phrase should be on a separate line in a text file.</p>
    /// <p>For information on the current quota limits for block lists, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas for Amazon Kendra</a>.</p>
    pub fn set_source_s3_path(mut self, input: ::std::option::Option<crate::types::S3Path>) -> Self {
        self.inner = self.inner.set_source_s3_path(input);
        self
    }
    /// <p>The S3 path to your block list text file in your S3 bucket.</p>
    /// <p>Each block word or phrase should be on a separate line in a text file.</p>
    /// <p>For information on the current quota limits for block lists, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/quotas.html">Quotas for Amazon Kendra</a>.</p>
    pub fn get_source_s3_path(&self) -> &::std::option::Option<crate::types::S3Path> {
        self.inner.get_source_s3_path()
    }
    /// <p>A token that you provide to identify the request to create a query suggestions block list.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that you provide to identify the request to create a query suggestions block list.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A token that you provide to identify the request to create a query suggestions block list.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to access your S3 bucket that contains the block list text file. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for Amazon Kendra</a>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to access your S3 bucket that contains the block list text file. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for Amazon Kendra</a>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to access your S3 bucket that contains the block list text file. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM access roles for Amazon Kendra</a>.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pairs that identify or categorize the block list. Tag keys and values can consist of Unicode letters, digits, white space, and any of the following symbols: _ . : / = + - @.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of key-value pairs that identify or categorize the block list. Tag keys and values can consist of Unicode letters, digits, white space, and any of the following symbols: _ . : / = + - @.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of key-value pairs that identify or categorize the block list. Tag keys and values can consist of Unicode letters, digits, white space, and any of the following symbols: _ . : / = + - @.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
