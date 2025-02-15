// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExecuteFastReset`](crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`action(Action)`](crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder::action) / [`set_action(Option<Action>)`](crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder::set_action):<br>required: **true**<br><p>The fast reset action. One of the following values:</p>  <ul>   <li> <p> <b> <code>initiateDatabaseReset</code> </b> &nbsp; – &nbsp; This action generates a unique token needed to actually perform the fast reset.</p> </li>   <li> <p> <b> <code>performDatabaseReset</code> </b> &nbsp; – &nbsp; This action uses the token generated by the <code>initiateDatabaseReset</code> action to actually perform the fast reset.</p> <p></p> </li>  </ul><br>
    ///   - [`token(impl Into<String>)`](crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder::token) / [`set_token(Option<String>)`](crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder::set_token):<br>required: **false**<br><p>The fast-reset token to initiate the reset.</p><br>
    /// - On success, responds with [`ExecuteFastResetOutput`](crate::operation::execute_fast_reset::ExecuteFastResetOutput) with field(s):
    ///   - [`status(String)`](crate::operation::execute_fast_reset::ExecuteFastResetOutput::status): <p>The <code>status</code> is only returned for the <code>performDatabaseReset</code> action, and indicates whether or not the fast reset rquest is accepted.</p>
    ///   - [`payload(Option<FastResetToken>)`](crate::operation::execute_fast_reset::ExecuteFastResetOutput::payload): <p>The <code>payload</code> is only returned by the <code>initiateDatabaseReset</code> action, and contains the unique token to use with the <code>performDatabaseReset</code> action to make the reset occur.</p>
    /// - On failure, responds with [`SdkError<ExecuteFastResetError>`](crate::operation::execute_fast_reset::ExecuteFastResetError)
    pub fn execute_fast_reset(&self) -> crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder {
        crate::operation::execute_fast_reset::builders::ExecuteFastResetFluentBuilder::new(self.handle.clone())
    }
}
