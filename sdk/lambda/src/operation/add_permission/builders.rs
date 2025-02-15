// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_permission::_add_permission_output::AddPermissionOutputBuilder;

pub use crate::operation::add_permission::_add_permission_input::AddPermissionInputBuilder;

impl AddPermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_permission::AddPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_permission::AddPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddPermission`.
///
/// <p>Grants an Amazon Web Service, Amazon Web Services account, or Amazon Web Services organization permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version or alias to invoke the function. Note: Lambda does not support adding policies to version $LATEST.</p>
/// <p>To grant permission to another account, specify the account ID as the <code>Principal</code>. To grant permission to an organization defined in Organizations, specify the organization ID as the <code>PrincipalOrgID</code>. For Amazon Web Services, the principal is a domain-style identifier that the service defines, such as <code>s3.amazonaws.com</code> or <code>sns.amazonaws.com</code>. For Amazon Web Services, you can also specify the ARN of the associated resource as the <code>SourceArn</code>. If you grant permission to a service principal without specifying the source, other accounts could potentially configure resources in their account to invoke your Lambda function.</p>
/// <p>This operation adds a statement to a resource-based permissions policy for the function. For more information about function policies, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Using resource-based policies for Lambda</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddPermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_permission::builders::AddPermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_permission::AddPermissionOutput,
        crate::operation::add_permission::AddPermissionError,
    > for AddPermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_permission::AddPermissionOutput,
            crate::operation::add_permission::AddPermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddPermissionFluentBuilder {
    /// Creates a new `AddPermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddPermission as a reference.
    pub fn as_input(&self) -> &crate::operation::add_permission::builders::AddPermissionInputBuilder {
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
        crate::operation::add_permission::AddPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_permission::AddPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_permission::AddPermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_permission::AddPermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_permission::AddPermissionOutput,
        crate::operation::add_permission::AddPermissionError,
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
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn statement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn set_statement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn get_statement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_id()
    }
    /// <p>The action that the principal can use on the function. For example, <code>lambda:InvokeFunction</code> or <code>lambda:GetFunction</code>.</p>
    pub fn action(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action(input.into());
        self
    }
    /// <p>The action that the principal can use on the function. For example, <code>lambda:InvokeFunction</code> or <code>lambda:GetFunction</code>.</p>
    pub fn set_action(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>The action that the principal can use on the function. For example, <code>lambda:InvokeFunction</code> or <code>lambda:GetFunction</code>.</p>
    pub fn get_action(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_action()
    }
    /// <p>The Amazon Web Service or Amazon Web Services account that invokes the function. If you specify a service, use <code>SourceArn</code> or <code>SourceAccount</code> to limit who can invoke the function through that service.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal(input.into());
        self
    }
    /// <p>The Amazon Web Service or Amazon Web Services account that invokes the function. If you specify a service, use <code>SourceArn</code> or <code>SourceAccount</code> to limit who can invoke the function through that service.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>The Amazon Web Service or Amazon Web Services account that invokes the function. If you specify a service, use <code>SourceArn</code> or <code>SourceAccount</code> to limit who can invoke the function through that service.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal()
    }
    /// <p>For Amazon Web Services, the ARN of the Amazon Web Services resource that invokes the function. For example, an Amazon S3 bucket or Amazon SNS topic.</p>
    /// <p>Note that Lambda configures the comparison using the <code>StringLike</code> operator.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_arn(input.into());
        self
    }
    /// <p>For Amazon Web Services, the ARN of the Amazon Web Services resource that invokes the function. For example, an Amazon S3 bucket or Amazon SNS topic.</p>
    /// <p>Note that Lambda configures the comparison using the <code>StringLike</code> operator.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_arn(input);
        self
    }
    /// <p>For Amazon Web Services, the ARN of the Amazon Web Services resource that invokes the function. For example, an Amazon S3 bucket or Amazon SNS topic.</p>
    /// <p>Note that Lambda configures the comparison using the <code>StringLike</code> operator.</p>
    pub fn get_source_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_arn()
    }
    /// <p>For Amazon Web Service, the ID of the Amazon Web Services account that owns the resource. Use this together with <code>SourceArn</code> to ensure that the specified account owns the resource. It is possible for an Amazon S3 bucket to be deleted by its owner and recreated by another account.</p>
    pub fn source_account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_account(input.into());
        self
    }
    /// <p>For Amazon Web Service, the ID of the Amazon Web Services account that owns the resource. Use this together with <code>SourceArn</code> to ensure that the specified account owns the resource. It is possible for an Amazon S3 bucket to be deleted by its owner and recreated by another account.</p>
    pub fn set_source_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_account(input);
        self
    }
    /// <p>For Amazon Web Service, the ID of the Amazon Web Services account that owns the resource. Use this together with <code>SourceArn</code> to ensure that the specified account owns the resource. It is possible for an Amazon S3 bucket to be deleted by its owner and recreated by another account.</p>
    pub fn get_source_account(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_account()
    }
    /// <p>For Alexa Smart Home functions, a token that the invoker must supply.</p>
    pub fn event_source_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_source_token(input.into());
        self
    }
    /// <p>For Alexa Smart Home functions, a token that the invoker must supply.</p>
    pub fn set_event_source_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_source_token(input);
        self
    }
    /// <p>For Alexa Smart Home functions, a token that the invoker must supply.</p>
    pub fn get_event_source_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_source_token()
    }
    /// <p>Specify a version or alias to add permissions to a published version of the function.</p>
    pub fn qualifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.qualifier(input.into());
        self
    }
    /// <p>Specify a version or alias to add permissions to a published version of the function.</p>
    pub fn set_qualifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_qualifier(input);
        self
    }
    /// <p>Specify a version or alias to add permissions to a published version of the function.</p>
    pub fn get_qualifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_qualifier()
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn get_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_revision_id()
    }
    /// <p>The identifier for your organization in Organizations. Use this to grant permissions to all the Amazon Web Services accounts under this organization.</p>
    pub fn principal_org_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_org_id(input.into());
        self
    }
    /// <p>The identifier for your organization in Organizations. Use this to grant permissions to all the Amazon Web Services accounts under this organization.</p>
    pub fn set_principal_org_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_org_id(input);
        self
    }
    /// <p>The identifier for your organization in Organizations. Use this to grant permissions to all the Amazon Web Services accounts under this organization.</p>
    pub fn get_principal_org_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_org_id()
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn function_url_auth_type(mut self, input: crate::types::FunctionUrlAuthType) -> Self {
        self.inner = self.inner.function_url_auth_type(input);
        self
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn set_function_url_auth_type(mut self, input: ::std::option::Option<crate::types::FunctionUrlAuthType>) -> Self {
        self.inner = self.inner.set_function_url_auth_type(input);
        self
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn get_function_url_auth_type(&self) -> &::std::option::Option<crate::types::FunctionUrlAuthType> {
        self.inner.get_function_url_auth_type()
    }
}
