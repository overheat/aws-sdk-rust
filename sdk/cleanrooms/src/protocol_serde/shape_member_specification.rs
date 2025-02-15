// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_member_specification(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MemberSpecification,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("accountId").string(input.account_id.as_str());
    }
    {
        let mut array_1 = object.key("memberAbilities").start_array();
        for item_2 in &input.member_abilities {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    {
        object.key("displayName").string(input.display_name.as_str());
    }
    Ok(())
}
