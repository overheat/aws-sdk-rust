// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_from_master_account_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountOutput,
    crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalException" => crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidAccessException" => crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::InvalidAccessException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidAccessExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_access_exception::de_invalid_access_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInputException" => crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededException" => crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_from_master_account_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountOutput,
    crate::operation::disassociate_from_master_account::DisassociateFromMasterAccountError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disassociate_from_master_account::builders::DisassociateFromMasterAccountOutputBuilder::default();
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
