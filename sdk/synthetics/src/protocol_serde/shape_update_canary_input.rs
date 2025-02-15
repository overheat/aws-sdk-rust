// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_canary_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_canary::UpdateCanaryInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.artifact_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ArtifactConfig").start_object();
        crate::protocol_serde::shape_artifact_config_input::ser_artifact_config_input(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.artifact_s3_location {
        object.key("ArtifactS3Location").string(var_3.as_str());
    }
    if let Some(var_4) = &input.code {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Code").start_object();
        crate::protocol_serde::shape_canary_code_input::ser_canary_code_input(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.failure_retention_period_in_days {
        object.key("FailureRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.run_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("RunConfig").start_object();
        crate::protocol_serde::shape_canary_run_config_input::ser_canary_run_config_input(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.runtime_version {
        object.key("RuntimeVersion").string(var_10.as_str());
    }
    if let Some(var_11) = &input.schedule {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Schedule").start_object();
        crate::protocol_serde::shape_canary_schedule_input::ser_canary_schedule_input(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.success_retention_period_in_days {
        object.key("SuccessRetentionPeriodInDays").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.visual_reference {
        #[allow(unused_mut)]
        let mut object_15 = object.key("VisualReference").start_object();
        crate::protocol_serde::shape_visual_reference_input::ser_visual_reference_input(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_17 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config_input::ser_vpc_config_input(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}
