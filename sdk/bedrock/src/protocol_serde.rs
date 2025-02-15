// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::http::HeaderMap,
    response_body: &[u8],
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_model_customization_job;

pub(crate) mod shape_create_provisioned_model_throughput;

pub(crate) mod shape_delete_custom_model;

pub(crate) mod shape_delete_model_invocation_logging_configuration;

pub(crate) mod shape_delete_provisioned_model_throughput;

pub(crate) mod shape_get_custom_model;

pub(crate) mod shape_get_foundation_model;

pub(crate) mod shape_get_model_customization_job;

pub(crate) mod shape_get_model_invocation_logging_configuration;

pub(crate) mod shape_get_provisioned_model_throughput;

pub(crate) mod shape_list_custom_models;

pub(crate) mod shape_list_foundation_models;

pub(crate) mod shape_list_model_customization_jobs;

pub(crate) mod shape_list_provisioned_model_throughputs;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_model_invocation_logging_configuration;

pub(crate) mod shape_stop_model_customization_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_provisioned_model_throughput;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_model_customization_job_input;

pub(crate) mod shape_create_provisioned_model_throughput_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_put_model_invocation_logging_configuration_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_provisioned_model_throughput_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_custom_model_summary_list;

pub(crate) mod shape_foundation_model_details;

pub(crate) mod shape_foundation_model_summary_list;

pub(crate) mod shape_logging_config;

pub(crate) mod shape_model_customization_hyper_parameters;

pub(crate) mod shape_model_customization_job_summaries;

pub(crate) mod shape_output_data_config;

pub(crate) mod shape_provisioned_model_summaries;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_training_data_config;

pub(crate) mod shape_training_metrics;

pub(crate) mod shape_validation_data_config;

pub(crate) mod shape_validation_metrics;

pub(crate) mod shape_vpc_config;

pub(crate) mod shape_cloud_watch_config;

pub(crate) mod shape_custom_model_summary;

pub(crate) mod shape_foundation_model_summary;

pub(crate) mod shape_inference_type_list;

pub(crate) mod shape_model_customization_job_summary;

pub(crate) mod shape_model_customization_list;

pub(crate) mod shape_model_modality_list;

pub(crate) mod shape_provisioned_model_summary;

pub(crate) mod shape_s3_config;

pub(crate) mod shape_security_group_ids;

pub(crate) mod shape_subnet_ids;

pub(crate) mod shape_validator;

pub(crate) mod shape_validator_metric;

pub(crate) mod shape_validators;
