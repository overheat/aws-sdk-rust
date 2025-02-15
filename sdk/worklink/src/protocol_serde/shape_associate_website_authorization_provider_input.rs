// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_website_authorization_provider_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::associate_website_authorization_provider::AssociateWebsiteAuthorizationProviderInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authorization_provider_type {
        object.key("AuthorizationProviderType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_name {
        object.key("DomainName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.fleet_arn {
        object.key("FleetArn").string(var_3.as_str());
    }
    Ok(())
}
