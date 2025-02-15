// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_message_move_task_input_input(
    input: &crate::operation::cancel_message_move_task::CancelMessageMoveTaskInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CancelMessageMoveTask", "2012-11-05");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TaskHandle");
    if let Some(var_2) = &input.task_handle {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
