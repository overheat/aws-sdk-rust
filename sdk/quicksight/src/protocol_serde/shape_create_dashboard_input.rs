// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dashboard_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_dashboard::CreateDashboardInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.dashboard_publish_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DashboardPublishOptions").start_object();
        crate::protocol_serde::shape_dashboard_publish_options::ser_dashboard_publish_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.definition {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Definition").start_object();
        crate::protocol_serde::shape_dashboard_version_definition::ser_dashboard_version_definition(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.folder_arns {
        let mut array_6 = object.key("FolderArns").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Parameters").start_object();
        crate::protocol_serde::shape_parameters::ser_parameters(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.permissions {
        let mut array_12 = object.key("Permissions").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_resource_permission::ser_resource_permission(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.source_entity {
        #[allow(unused_mut)]
        let mut object_16 = object.key("SourceEntity").start_object();
        crate::protocol_serde::shape_dashboard_source_entity::ser_dashboard_source_entity(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut array_18 = object.key("Tags").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.theme_arn {
        object.key("ThemeArn").string(var_21.as_str());
    }
    if let Some(var_22) = &input.validation_strategy {
        #[allow(unused_mut)]
        let mut object_23 = object.key("ValidationStrategy").start_object();
        crate::protocol_serde::shape_validation_strategy::ser_validation_strategy(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.version_description {
        object.key("VersionDescription").string(var_24.as_str());
    }
    Ok(())
}
