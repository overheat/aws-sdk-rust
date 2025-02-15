// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_channel_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_channel::CreateChannelInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authorized {
        object.key("authorized").boolean(*var_1);
    }
    if let Some(var_2) = &input.insecure_ingest {
        object.key("insecureIngest").boolean(*var_2);
    }
    if let Some(var_3) = &input.latency_mode {
        object.key("latencyMode").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.preset {
        object.key("preset").string(var_5.as_str());
    }
    if let Some(var_6) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.r#type {
        object.key("type").string(var_11.as_str());
    }
    Ok(())
}
