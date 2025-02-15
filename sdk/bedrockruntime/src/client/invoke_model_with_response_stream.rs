// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`InvokeModelWithResponseStream`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`body(Blob)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::body) / [`set_body(Option<Blob>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::set_body):<br>required: **true**<br><p>Inference input in the format specified by the content-type. To see the format and content of this field for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>.</p><br>
    ///   - [`content_type(impl Into<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::content_type) / [`set_content_type(Option<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::set_content_type):<br>required: **false**<br><p>The MIME type of the input data in the request. The default value is <code>application/json</code>.</p><br>
    ///   - [`accept(impl Into<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::accept) / [`set_accept(Option<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::set_accept):<br>required: **false**<br><p>The desired MIME type of the inference body in the response. The default value is <code>application/json</code>.</p><br>
    ///   - [`model_id(impl Into<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::model_id) / [`set_model_id(Option<String>)`](crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::set_model_id):<br>required: **true**<br><p>Id of the model to invoke using the streaming request.</p><br>
    /// - On success, responds with [`InvokeModelWithResponseStreamOutput`](crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamOutput) with field(s):
    ///   - [`body(Receiver<ResponseStream, ResponseStreamError>)`](crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamOutput::body): <p>Inference response from the model in the format specified by Content-Type. To see the format and content of this field for different models, refer to <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html">Inference parameters</a>.</p>
    ///   - [`content_type(String)`](crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamOutput::content_type): <p>The MIME type of the inference result.</p>
    /// - On failure, responds with [`SdkError<InvokeModelWithResponseStreamError>`](crate::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError)
    pub fn invoke_model_with_response_stream(
        &self,
    ) -> crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder {
        crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamFluentBuilder::new(self.handle.clone())
    }
}
