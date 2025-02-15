// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_body_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, crate::operation::invoke_endpoint::InvokeEndpointError> {
    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
}

pub(crate) fn de_content_type_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type").iter();
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_custom_attributes_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("X-Amzn-SageMaker-Custom-Attributes").iter();
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_invoked_production_variant_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-Amzn-Invoked-Production-Variant").iter();
    ::aws_smithy_http::header::one_or_none(headers)
}
