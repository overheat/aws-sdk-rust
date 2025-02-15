// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_container_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContainerDefinition,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.container_hostname {
        object.key("ContainerHostname").string(var_1.as_str());
    }
    if let Some(var_2) = &input.image {
        object.key("Image").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ImageConfig").start_object();
        crate::protocol_serde::shape_image_config::ser_image_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.mode {
        object.key("Mode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.model_data_url {
        object.key("ModelDataUrl").string(var_6.as_str());
    }
    if let Some(var_7) = &input.environment {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Environment").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.model_package_name {
        object.key("ModelPackageName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.inference_specification_name {
        object.key("InferenceSpecificationName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.multi_model_config {
        #[allow(unused_mut)]
        let mut object_14 = object.key("MultiModelConfig").start_object();
        crate::protocol_serde::shape_multi_model_config::ser_multi_model_config(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.model_data_source {
        #[allow(unused_mut)]
        let mut object_16 = object.key("ModelDataSource").start_object();
        crate::protocol_serde::shape_model_data_source::ser_model_data_source(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub(crate) fn de_container_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ContainerDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ContainerDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ContainerHostname" => {
                            builder = builder.set_container_hostname(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Image" => {
                            builder = builder.set_image(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ImageConfig" => {
                            builder = builder.set_image_config(crate::protocol_serde::shape_image_config::de_image_config(tokens)?);
                        }
                        "Mode" => {
                            builder = builder.set_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ContainerMode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ModelDataUrl" => {
                            builder = builder.set_model_data_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Environment" => {
                            builder = builder.set_environment(crate::protocol_serde::shape_environment_map::de_environment_map(tokens)?);
                        }
                        "ModelPackageName" => {
                            builder = builder.set_model_package_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InferenceSpecificationName" => {
                            builder = builder.set_inference_specification_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MultiModelConfig" => {
                            builder = builder.set_multi_model_config(crate::protocol_serde::shape_multi_model_config::de_multi_model_config(tokens)?);
                        }
                        "ModelDataSource" => {
                            builder = builder.set_model_data_source(crate::protocol_serde::shape_model_data_source::de_model_data_source(tokens)?);
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
