// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendEvent`](crate::operation::send_event::builders::SendEventFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`content_type(impl Into<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::content_type) / [`set_content_type(Option<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::set_content_type):<br>required: **true**<br><p>The content type of the request. Supported types are:</p>  <ul>   <li> <p>application/vnd.amazonaws.connect.event.typing</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.connection.acknowledged</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.message.delivered</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.message.read</p> </li>  </ul><br>
    ///   - [`content(impl Into<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::content) / [`set_content(Option<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::set_content):<br>required: **false**<br><p>The content of the event to be sent (for example, message text). For content related to message receipts, this is supported in the form of a JSON string.</p>  <p>Sample Content: "{\"messageId\":\"11111111-aaaa-bbbb-cccc-EXAMPLE01234\"}"</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p><br>
    ///   - [`connection_token(impl Into<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::connection_token) / [`set_connection_token(Option<String>)`](crate::operation::send_event::builders::SendEventFluentBuilder::set_connection_token):<br>required: **true**<br><p>The authentication token associated with the participant's connection.</p><br>
    /// - On success, responds with [`SendEventOutput`](crate::operation::send_event::SendEventOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::send_event::SendEventOutput::id): <p>The ID of the response.</p>
    ///   - [`absolute_time(Option<String>)`](crate::operation::send_event::SendEventOutput::absolute_time): <p>The time when the event was sent.</p>  <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
    /// - On failure, responds with [`SdkError<SendEventError>`](crate::operation::send_event::SendEventError)
    pub fn send_event(&self) -> crate::operation::send_event::builders::SendEventFluentBuilder {
        crate::operation::send_event::builders::SendEventFluentBuilder::new(self.handle.clone())
    }
}
