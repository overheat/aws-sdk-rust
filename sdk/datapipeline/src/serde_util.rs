// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_pipeline_output_correct_errors(
    mut builder: crate::operation::create_pipeline::builders::CreatePipelineOutputBuilder,
) -> crate::operation::create_pipeline::builders::CreatePipelineOutputBuilder {
    if builder.pipeline_id.is_none() {
        builder.pipeline_id = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_objects_output_correct_errors(
    mut builder: crate::operation::describe_objects::builders::DescribeObjectsOutputBuilder,
) -> crate::operation::describe_objects::builders::DescribeObjectsOutputBuilder {
    if builder.pipeline_objects.is_none() {
        builder.pipeline_objects = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_pipelines_output_correct_errors(
    mut builder: crate::operation::describe_pipelines::builders::DescribePipelinesOutputBuilder,
) -> crate::operation::describe_pipelines::builders::DescribePipelinesOutputBuilder {
    if builder.pipeline_description_list.is_none() {
        builder.pipeline_description_list = Some(Default::default())
    }
    builder
}

pub(crate) fn evaluate_expression_output_correct_errors(
    mut builder: crate::operation::evaluate_expression::builders::EvaluateExpressionOutputBuilder,
) -> crate::operation::evaluate_expression::builders::EvaluateExpressionOutputBuilder {
    if builder.evaluated_expression.is_none() {
        builder.evaluated_expression = Some(Default::default())
    }
    builder
}

pub(crate) fn list_pipelines_output_correct_errors(
    mut builder: crate::operation::list_pipelines::builders::ListPipelinesOutputBuilder,
) -> crate::operation::list_pipelines::builders::ListPipelinesOutputBuilder {
    if builder.pipeline_id_list.is_none() {
        builder.pipeline_id_list = Some(Default::default())
    }
    builder
}

pub(crate) fn put_pipeline_definition_output_correct_errors(
    mut builder: crate::operation::put_pipeline_definition::builders::PutPipelineDefinitionOutputBuilder,
) -> crate::operation::put_pipeline_definition::builders::PutPipelineDefinitionOutputBuilder {
    if builder.errored.is_none() {
        builder.errored = Some(Default::default())
    }
    builder
}

pub(crate) fn report_task_progress_output_correct_errors(
    mut builder: crate::operation::report_task_progress::builders::ReportTaskProgressOutputBuilder,
) -> crate::operation::report_task_progress::builders::ReportTaskProgressOutputBuilder {
    if builder.canceled.is_none() {
        builder.canceled = Some(Default::default())
    }
    builder
}

pub(crate) fn report_task_runner_heartbeat_output_correct_errors(
    mut builder: crate::operation::report_task_runner_heartbeat::builders::ReportTaskRunnerHeartbeatOutputBuilder,
) -> crate::operation::report_task_runner_heartbeat::builders::ReportTaskRunnerHeartbeatOutputBuilder {
    if builder.terminate.is_none() {
        builder.terminate = Some(Default::default())
    }
    builder
}

pub(crate) fn validate_pipeline_definition_output_correct_errors(
    mut builder: crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionOutputBuilder,
) -> crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionOutputBuilder {
    if builder.errored.is_none() {
        builder.errored = Some(Default::default())
    }
    builder
}

pub(crate) fn parameter_object_correct_errors(
    mut builder: crate::types::builders::ParameterObjectBuilder,
) -> crate::types::builders::ParameterObjectBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.attributes.is_none() {
        builder.attributes = Some(Default::default())
    }
    builder
}

pub(crate) fn parameter_value_correct_errors(
    mut builder: crate::types::builders::ParameterValueBuilder,
) -> crate::types::builders::ParameterValueBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.string_value.is_none() {
        builder.string_value = Some(Default::default())
    }
    builder
}

pub(crate) fn pipeline_description_correct_errors(
    mut builder: crate::types::builders::PipelineDescriptionBuilder,
) -> crate::types::builders::PipelineDescriptionBuilder {
    if builder.pipeline_id.is_none() {
        builder.pipeline_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.fields.is_none() {
        builder.fields = Some(Default::default())
    }
    builder
}

pub(crate) fn pipeline_object_correct_errors(
    mut builder: crate::types::builders::PipelineObjectBuilder,
) -> crate::types::builders::PipelineObjectBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.fields.is_none() {
        builder.fields = Some(Default::default())
    }
    builder
}

pub(crate) fn field_correct_errors(mut builder: crate::types::builders::FieldBuilder) -> crate::types::builders::FieldBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn parameter_attribute_correct_errors(
    mut builder: crate::types::builders::ParameterAttributeBuilder,
) -> crate::types::builders::ParameterAttributeBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.string_value.is_none() {
        builder.string_value = Some(Default::default())
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}
