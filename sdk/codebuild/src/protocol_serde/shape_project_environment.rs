// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_project_environment(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ProjectEnvironment,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("type").string(input.r#type.as_str());
    }
    {
        object.key("image").string(input.image.as_str());
    }
    {
        object.key("computeType").string(input.compute_type.as_str());
    }
    if let Some(var_1) = &input.environment_variables {
        let mut array_2 = object.key("environmentVariables").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_environment_variable::ser_environment_variable(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.privileged_mode {
        object.key("privilegedMode").boolean(*var_5);
    }
    if let Some(var_6) = &input.certificate {
        object.key("certificate").string(var_6.as_str());
    }
    if let Some(var_7) = &input.registry_credential {
        #[allow(unused_mut)]
        let mut object_8 = object.key("registryCredential").start_object();
        crate::protocol_serde::shape_registry_credential::ser_registry_credential(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.image_pull_credentials_type {
        object.key("imagePullCredentialsType").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_project_environment<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ProjectEnvironment>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProjectEnvironmentBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnvironmentType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "image" => {
                            builder = builder.set_image(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "computeType" => {
                            builder = builder.set_compute_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ComputeType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "environmentVariables" => {
                            builder = builder
                                .set_environment_variables(crate::protocol_serde::shape_environment_variables::de_environment_variables(tokens)?);
                        }
                        "privilegedMode" => {
                            builder = builder.set_privileged_mode(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "certificate" => {
                            builder = builder.set_certificate(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "registryCredential" => {
                            builder =
                                builder.set_registry_credential(crate::protocol_serde::shape_registry_credential::de_registry_credential(tokens)?);
                        }
                        "imagePullCredentialsType" => {
                            builder = builder.set_image_pull_credentials_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ImagePullCredentialsType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::project_environment_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
