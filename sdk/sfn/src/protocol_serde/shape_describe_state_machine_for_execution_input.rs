// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_state_machine_for_execution_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_state_machine_for_execution::DescribeStateMachineForExecutionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.execution_arn {
        object.key("executionArn").string(var_1.as_str());
    }
    Ok(())
}
