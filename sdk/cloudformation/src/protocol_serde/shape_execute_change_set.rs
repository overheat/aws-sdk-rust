// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_execute_change_set_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::execute_change_set::ExecuteChangeSetOutput, crate::operation::execute_change_set::ExecuteChangeSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ChangeSetNotFound" => crate::operation::execute_change_set::ExecuteChangeSetError::ChangeSetNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ChangeSetNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_change_set_not_found_exception::de_change_set_not_found_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InsufficientCapabilitiesException" => crate::operation::execute_change_set::ExecuteChangeSetError::InsufficientCapabilitiesException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InsufficientCapabilitiesExceptionBuilder::default();
                output = crate::protocol_serde::shape_insufficient_capabilities_exception::de_insufficient_capabilities_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidChangeSetStatus" => crate::operation::execute_change_set::ExecuteChangeSetError::InvalidChangeSetStatusException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidChangeSetStatusExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_change_set_status_exception::de_invalid_change_set_status_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TokenAlreadyExistsException" => crate::operation::execute_change_set::ExecuteChangeSetError::TokenAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TokenAlreadyExistsExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_token_already_exists_exception::de_token_already_exists_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::execute_change_set::ExecuteChangeSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::execute_change_set::ExecuteChangeSetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_execute_change_set_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::execute_change_set::ExecuteChangeSetOutput, crate::operation::execute_change_set::ExecuteChangeSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::execute_change_set::builders::ExecuteChangeSetOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
