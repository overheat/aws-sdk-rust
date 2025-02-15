// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_alias_output_correct_errors(
    mut builder: crate::operation::create_alias::builders::CreateAliasOutputBuilder,
) -> crate::operation::create_alias::builders::CreateAliasOutputBuilder {
    if builder.alias.is_none() {
        builder.alias = {
            let builder = crate::types::builders::AliasBuilder::default();
            crate::serde_util::alias_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn create_key_output_correct_errors(
    mut builder: crate::operation::create_key::builders::CreateKeyOutputBuilder,
) -> crate::operation::create_key::builders::CreateKeyOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn delete_key_output_correct_errors(
    mut builder: crate::operation::delete_key::builders::DeleteKeyOutputBuilder,
) -> crate::operation::delete_key::builders::DeleteKeyOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_alias_output_correct_errors(
    mut builder: crate::operation::get_alias::builders::GetAliasOutputBuilder,
) -> crate::operation::get_alias::builders::GetAliasOutputBuilder {
    if builder.alias.is_none() {
        builder.alias = {
            let builder = crate::types::builders::AliasBuilder::default();
            crate::serde_util::alias_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_key_output_correct_errors(
    mut builder: crate::operation::get_key::builders::GetKeyOutputBuilder,
) -> crate::operation::get_key::builders::GetKeyOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_parameters_for_export_output_correct_errors(
    mut builder: crate::operation::get_parameters_for_export::builders::GetParametersForExportOutputBuilder,
) -> crate::operation::get_parameters_for_export::builders::GetParametersForExportOutputBuilder {
    if builder.signing_key_certificate.is_none() {
        builder.signing_key_certificate = Some(Default::default())
    }
    if builder.signing_key_certificate_chain.is_none() {
        builder.signing_key_certificate_chain = Some(Default::default())
    }
    if builder.signing_key_algorithm.is_none() {
        builder.signing_key_algorithm = "no value was set".parse::<crate::types::KeyAlgorithm>().ok()
    }
    if builder.export_token.is_none() {
        builder.export_token = Some(Default::default())
    }
    if builder.parameters_valid_until_timestamp.is_none() {
        builder.parameters_valid_until_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_parameters_for_import_output_correct_errors(
    mut builder: crate::operation::get_parameters_for_import::builders::GetParametersForImportOutputBuilder,
) -> crate::operation::get_parameters_for_import::builders::GetParametersForImportOutputBuilder {
    if builder.wrapping_key_certificate.is_none() {
        builder.wrapping_key_certificate = Some(Default::default())
    }
    if builder.wrapping_key_certificate_chain.is_none() {
        builder.wrapping_key_certificate_chain = Some(Default::default())
    }
    if builder.wrapping_key_algorithm.is_none() {
        builder.wrapping_key_algorithm = "no value was set".parse::<crate::types::KeyAlgorithm>().ok()
    }
    if builder.import_token.is_none() {
        builder.import_token = Some(Default::default())
    }
    if builder.parameters_valid_until_timestamp.is_none() {
        builder.parameters_valid_until_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_public_key_certificate_output_correct_errors(
    mut builder: crate::operation::get_public_key_certificate::builders::GetPublicKeyCertificateOutputBuilder,
) -> crate::operation::get_public_key_certificate::builders::GetPublicKeyCertificateOutputBuilder {
    if builder.key_certificate.is_none() {
        builder.key_certificate = Some(Default::default())
    }
    if builder.key_certificate_chain.is_none() {
        builder.key_certificate_chain = Some(Default::default())
    }
    builder
}

pub(crate) fn import_key_output_correct_errors(
    mut builder: crate::operation::import_key::builders::ImportKeyOutputBuilder,
) -> crate::operation::import_key::builders::ImportKeyOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn list_aliases_output_correct_errors(
    mut builder: crate::operation::list_aliases::builders::ListAliasesOutputBuilder,
) -> crate::operation::list_aliases::builders::ListAliasesOutputBuilder {
    if builder.aliases.is_none() {
        builder.aliases = Some(Default::default())
    }
    builder
}

pub(crate) fn list_keys_output_correct_errors(
    mut builder: crate::operation::list_keys::builders::ListKeysOutputBuilder,
) -> crate::operation::list_keys::builders::ListKeysOutputBuilder {
    if builder.keys.is_none() {
        builder.keys = Some(Default::default())
    }
    builder
}

pub(crate) fn list_tags_for_resource_output_correct_errors(
    mut builder: crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder {
    if builder.tags.is_none() {
        builder.tags = Some(Default::default())
    }
    builder
}

pub(crate) fn restore_key_output_correct_errors(
    mut builder: crate::operation::restore_key::builders::RestoreKeyOutputBuilder,
) -> crate::operation::restore_key::builders::RestoreKeyOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn start_key_usage_output_correct_errors(
    mut builder: crate::operation::start_key_usage::builders::StartKeyUsageOutputBuilder,
) -> crate::operation::start_key_usage::builders::StartKeyUsageOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn stop_key_usage_output_correct_errors(
    mut builder: crate::operation::stop_key_usage::builders::StopKeyUsageOutputBuilder,
) -> crate::operation::stop_key_usage::builders::StopKeyUsageOutputBuilder {
    if builder.key.is_none() {
        builder.key = {
            let builder = crate::types::builders::KeyBuilder::default();
            crate::serde_util::key_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn update_alias_output_correct_errors(
    mut builder: crate::operation::update_alias::builders::UpdateAliasOutputBuilder,
) -> crate::operation::update_alias::builders::UpdateAliasOutputBuilder {
    if builder.alias.is_none() {
        builder.alias = {
            let builder = crate::types::builders::AliasBuilder::default();
            crate::serde_util::alias_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn alias_correct_errors(mut builder: crate::types::builders::AliasBuilder) -> crate::types::builders::AliasBuilder {
    if builder.alias_name.is_none() {
        builder.alias_name = Some(Default::default())
    }
    builder
}

pub(crate) fn key_correct_errors(mut builder: crate::types::builders::KeyBuilder) -> crate::types::builders::KeyBuilder {
    if builder.key_arn.is_none() {
        builder.key_arn = Some(Default::default())
    }
    if builder.key_attributes.is_none() {
        builder.key_attributes = {
            let builder = crate::types::builders::KeyAttributesBuilder::default();
            crate::serde_util::key_attributes_correct_errors(builder).build().ok()
        }
    }
    if builder.key_check_value.is_none() {
        builder.key_check_value = Some(Default::default())
    }
    if builder.key_check_value_algorithm.is_none() {
        builder.key_check_value_algorithm = "no value was set".parse::<crate::types::KeyCheckValueAlgorithm>().ok()
    }
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    if builder.exportable.is_none() {
        builder.exportable = Some(Default::default())
    }
    if builder.key_state.is_none() {
        builder.key_state = "no value was set".parse::<crate::types::KeyState>().ok()
    }
    if builder.key_origin.is_none() {
        builder.key_origin = "no value was set".parse::<crate::types::KeyOrigin>().ok()
    }
    if builder.create_timestamp.is_none() {
        builder.create_timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn wrapped_key_correct_errors(mut builder: crate::types::builders::WrappedKeyBuilder) -> crate::types::builders::WrappedKeyBuilder {
    if builder.wrapping_key_arn.is_none() {
        builder.wrapping_key_arn = Some(Default::default())
    }
    if builder.wrapped_key_material_format.is_none() {
        builder.wrapped_key_material_format = "no value was set".parse::<crate::types::WrappedKeyMaterialFormat>().ok()
    }
    if builder.key_material.is_none() {
        builder.key_material = Some(Default::default())
    }
    builder
}

pub(crate) fn key_attributes_correct_errors(
    mut builder: crate::types::builders::KeyAttributesBuilder,
) -> crate::types::builders::KeyAttributesBuilder {
    if builder.key_usage.is_none() {
        builder.key_usage = "no value was set".parse::<crate::types::KeyUsage>().ok()
    }
    if builder.key_class.is_none() {
        builder.key_class = "no value was set".parse::<crate::types::KeyClass>().ok()
    }
    if builder.key_algorithm.is_none() {
        builder.key_algorithm = "no value was set".parse::<crate::types::KeyAlgorithm>().ok()
    }
    if builder.key_modes_of_use.is_none() {
        builder.key_modes_of_use = {
            let builder = crate::types::builders::KeyModesOfUseBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn key_summary_correct_errors(mut builder: crate::types::builders::KeySummaryBuilder) -> crate::types::builders::KeySummaryBuilder {
    if builder.key_arn.is_none() {
        builder.key_arn = Some(Default::default())
    }
    if builder.key_state.is_none() {
        builder.key_state = "no value was set".parse::<crate::types::KeyState>().ok()
    }
    if builder.key_attributes.is_none() {
        builder.key_attributes = {
            let builder = crate::types::builders::KeyAttributesBuilder::default();
            crate::serde_util::key_attributes_correct_errors(builder).build().ok()
        }
    }
    if builder.key_check_value.is_none() {
        builder.key_check_value = Some(Default::default())
    }
    if builder.exportable.is_none() {
        builder.exportable = Some(Default::default())
    }
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}
