// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_environment_blueprint_configuration_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationOutput,
    crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output).build().map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output).build().map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output).build().map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::throttling_exception_correct_errors(output).build().map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "UnauthorizedException" => {
            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::unauthorized_exception_correct_errors(output).build().map_err(
                        crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::unhandled,
                    )?
                };
                tmp
            })
        }
        _ => crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_environment_blueprint_configuration_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationOutput,
    crate::operation::delete_environment_blueprint_configuration::DeleteEnvironmentBlueprintConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_environment_blueprint_configuration::builders::DeleteEnvironmentBlueprintConfigurationOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
