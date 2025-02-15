// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_device_usage_data_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_device_usage_data::DeleteDeviceUsageDataInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.device_arn {
        object.key("DeviceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_usage_type {
        object.key("DeviceUsageType").string(var_2.as_str());
    }
    Ok(())
}
