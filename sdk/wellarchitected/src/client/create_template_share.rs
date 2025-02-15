// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTemplateShare`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_arn(impl Into<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::template_arn) / [`set_template_arn(Option<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::set_template_arn):<br>required: **true**<br><p>The review template ARN.</p><br>
    ///   - [`shared_with(impl Into<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::shared_with) / [`set_shared_with(Option<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::set_shared_with):<br>required: **true**<br><p>The Amazon Web Services account ID, organization ID, or organizational unit (OU) ID with which the workload, lens, profile, or review template is shared.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::set_client_request_token):<br>required: **true**<br><p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p>  <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after the original request has completed successfully, the result of the original request is returned.</p> <important>   <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p>  </important><br>
    /// - On success, responds with [`CreateTemplateShareOutput`](crate::operation::create_template_share::CreateTemplateShareOutput) with field(s):
    ///   - [`template_arn(Option<String>)`](crate::operation::create_template_share::CreateTemplateShareOutput::template_arn): <p>The review template ARN.</p>
    ///   - [`share_id(Option<String>)`](crate::operation::create_template_share::CreateTemplateShareOutput::share_id): <p>The ID associated with the share.</p>
    /// - On failure, responds with [`SdkError<CreateTemplateShareError>`](crate::operation::create_template_share::CreateTemplateShareError)
    pub fn create_template_share(&self) -> crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder {
        crate::operation::create_template_share::builders::CreateTemplateShareFluentBuilder::new(self.handle.clone())
    }
}
