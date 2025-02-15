// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_profile_object_type_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_profile_object_type::PutProfileObjectTypeInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.allow_profile_creation {
        object.key("AllowProfileCreation").boolean(*var_1);
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.encryption_key {
        object.key("EncryptionKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.expiration_days {
        object.key("ExpirationDays").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.fields {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Fields").start_object();
        for (key_7, value_8) in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_9 = object_6.key(key_7.as_str()).start_object();
                crate::protocol_serde::shape_object_type_field::ser_object_type_field(&mut object_9, value_8)?;
                object_9.finish();
            }
        }
        object_6.finish();
    }
    if let Some(var_10) = &input.keys {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Keys").start_object();
        for (key_12, value_13) in var_10 {
            {
                let mut array_14 = object_11.key(key_12.as_str()).start_array();
                for item_15 in value_13 {
                    {
                        #[allow(unused_mut)]
                        let mut object_16 = array_14.value().start_object();
                        crate::protocol_serde::shape_object_type_key::ser_object_type_key(&mut object_16, item_15)?;
                        object_16.finish();
                    }
                }
                array_14.finish();
            }
        }
        object_11.finish();
    }
    if let Some(var_17) = &input.source_last_updated_timestamp_format {
        object.key("SourceLastUpdatedTimestampFormat").string(var_17.as_str());
    }
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("Tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    if let Some(var_22) = &input.template_id {
        object.key("TemplateId").string(var_22.as_str());
    }
    Ok(())
}
