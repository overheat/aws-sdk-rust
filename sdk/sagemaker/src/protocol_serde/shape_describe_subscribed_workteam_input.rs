// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_subscribed_workteam_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_subscribed_workteam::DescribeSubscribedWorkteamInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.workteam_arn {
        object.key("WorkteamArn").string(var_1.as_str());
    }
    Ok(())
}
