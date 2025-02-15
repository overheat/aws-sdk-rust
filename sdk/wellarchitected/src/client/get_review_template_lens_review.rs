// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetReviewTemplateLensReview`](crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_arn(impl Into<String>)`](crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder::template_arn) / [`set_template_arn(Option<String>)`](crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder::set_template_arn):<br>required: **true**<br><p>The review template ARN.</p><br>
    ///   - [`lens_alias(impl Into<String>)`](crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder::lens_alias) / [`set_lens_alias(Option<String>)`](crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder::set_lens_alias):<br>required: **true**<br><p>The alias of the lens.</p>  <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>  <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>  <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p><br>
    /// - On success, responds with [`GetReviewTemplateLensReviewOutput`](crate::operation::get_review_template_lens_review::GetReviewTemplateLensReviewOutput) with field(s):
    ///   - [`template_arn(Option<String>)`](crate::operation::get_review_template_lens_review::GetReviewTemplateLensReviewOutput::template_arn): <p>The review template ARN.</p>
    ///   - [`lens_review(Option<ReviewTemplateLensReview>)`](crate::operation::get_review_template_lens_review::GetReviewTemplateLensReviewOutput::lens_review): <p>A lens review of a question.</p>
    /// - On failure, responds with [`SdkError<GetReviewTemplateLensReviewError>`](crate::operation::get_review_template_lens_review::GetReviewTemplateLensReviewError)
    pub fn get_review_template_lens_review(
        &self,
    ) -> crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder {
        crate::operation::get_review_template_lens_review::builders::GetReviewTemplateLensReviewFluentBuilder::new(self.handle.clone())
    }
}
