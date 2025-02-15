// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetChangeToken`](crate::operation::get_change_token::builders::GetChangeTokenFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_change_token::builders::GetChangeTokenFluentBuilder::send) it.
    /// - On success, responds with [`GetChangeTokenOutput`](crate::operation::get_change_token::GetChangeTokenOutput) with field(s):
    ///   - [`change_token(Option<String>)`](crate::operation::get_change_token::GetChangeTokenOutput::change_token): <p>The <code>ChangeToken</code> that you used in the request. Use this value in a <code>GetChangeTokenStatus</code> request to get the current status of the request. </p>
    /// - On failure, responds with [`SdkError<GetChangeTokenError>`](crate::operation::get_change_token::GetChangeTokenError)
    pub fn get_change_token(&self) -> crate::operation::get_change_token::builders::GetChangeTokenFluentBuilder {
        crate::operation::get_change_token::builders::GetChangeTokenFluentBuilder::new(self.handle.clone())
    }
}
