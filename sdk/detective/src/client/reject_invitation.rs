// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RejectInvitation`](crate::operation::reject_invitation::builders::RejectInvitationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`graph_arn(impl Into<String>)`](crate::operation::reject_invitation::builders::RejectInvitationFluentBuilder::graph_arn) / [`set_graph_arn(Option<String>)`](crate::operation::reject_invitation::builders::RejectInvitationFluentBuilder::set_graph_arn):<br>required: **true**<br><p>The ARN of the behavior graph to reject the invitation to.</p>  <p>The member account's current member status in the behavior graph must be <code>INVITED</code>.</p><br>
    /// - On success, responds with [`RejectInvitationOutput`](crate::operation::reject_invitation::RejectInvitationOutput)
    /// - On failure, responds with [`SdkError<RejectInvitationError>`](crate::operation::reject_invitation::RejectInvitationError)
    pub fn reject_invitation(&self) -> crate::operation::reject_invitation::builders::RejectInvitationFluentBuilder {
        crate::operation::reject_invitation::builders::RejectInvitationFluentBuilder::new(self.handle.clone())
    }
}
