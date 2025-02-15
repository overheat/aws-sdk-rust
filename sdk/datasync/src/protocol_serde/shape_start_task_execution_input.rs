// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_task_execution_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_task_execution::StartTaskExecutionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.task_arn {
        object.key("TaskArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.override_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("OverrideOptions").start_object();
        crate::protocol_serde::shape_options::ser_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.includes {
        let mut array_5 = object.key("Includes").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_filter_rule::ser_filter_rule(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.excludes {
        let mut array_9 = object.key("Excludes").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_filter_rule::ser_filter_rule(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.tags {
        let mut array_13 = object.key("Tags").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_tag_list_entry::ser_tag_list_entry(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.task_report_config {
        #[allow(unused_mut)]
        let mut object_17 = object.key("TaskReportConfig").start_object();
        crate::protocol_serde::shape_task_report_config::ser_task_report_config(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}
