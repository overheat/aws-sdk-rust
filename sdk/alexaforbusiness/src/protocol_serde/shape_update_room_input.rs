// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_room_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_room::UpdateRoomInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.room_arn {
        object.key("RoomArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.room_name {
        object.key("RoomName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.provider_calendar_id {
        object.key("ProviderCalendarId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.profile_arn {
        object.key("ProfileArn").string(var_5.as_str());
    }
    Ok(())
}
