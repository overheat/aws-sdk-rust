// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_connection_output_correct_errors(
    mut builder: crate::operation::create_connection::builders::CreateConnectionOutputBuilder,
) -> crate::operation::create_connection::builders::CreateConnectionOutputBuilder {
    if builder.connection_arn.is_none() {
        builder.connection_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn vpc_configuration_correct_errors(
    mut builder: crate::types::builders::VpcConfigurationBuilder,
) -> crate::types::builders::VpcConfigurationBuilder {
    if builder.vpc_id.is_none() {
        builder.vpc_id = Some(Default::default())
    }
    if builder.subnet_ids.is_none() {
        builder.subnet_ids = Some(Default::default())
    }
    if builder.security_group_ids.is_none() {
        builder.security_group_ids = Some(Default::default())
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
