// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_asset_model_composite_model_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssetModelCompositeModelDefinition,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    if let Some(var_2) = &input.properties {
        let mut array_3 = object.key("properties").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_asset_model_property_definition::ser_asset_model_property_definition(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}
