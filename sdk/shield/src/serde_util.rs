// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn describe_attack_statistics_output_correct_errors(
    mut builder: crate::operation::describe_attack_statistics::builders::DescribeAttackStatisticsOutputBuilder,
) -> crate::operation::describe_attack_statistics::builders::DescribeAttackStatisticsOutputBuilder {
    if builder.time_range.is_none() {
        builder.time_range = {
            let builder = crate::types::builders::TimeRangeBuilder::default();
            Some(builder.build())
        }
    }
    if builder.data_items.is_none() {
        builder.data_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_protection_group_output_correct_errors(
    mut builder: crate::operation::describe_protection_group::builders::DescribeProtectionGroupOutputBuilder,
) -> crate::operation::describe_protection_group::builders::DescribeProtectionGroupOutputBuilder {
    if builder.protection_group.is_none() {
        builder.protection_group = {
            let builder = crate::types::builders::ProtectionGroupBuilder::default();
            crate::serde_util::protection_group_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_subscription_state_output_correct_errors(
    mut builder: crate::operation::get_subscription_state::builders::GetSubscriptionStateOutputBuilder,
) -> crate::operation::get_subscription_state::builders::GetSubscriptionStateOutputBuilder {
    if builder.subscription_state.is_none() {
        builder.subscription_state = "no value was set".parse::<crate::types::SubscriptionState>().ok()
    }
    builder
}

pub(crate) fn list_protection_groups_output_correct_errors(
    mut builder: crate::operation::list_protection_groups::builders::ListProtectionGroupsOutputBuilder,
) -> crate::operation::list_protection_groups::builders::ListProtectionGroupsOutputBuilder {
    if builder.protection_groups.is_none() {
        builder.protection_groups = Some(Default::default())
    }
    builder
}

pub(crate) fn list_resources_in_protection_group_output_correct_errors(
    mut builder: crate::operation::list_resources_in_protection_group::builders::ListResourcesInProtectionGroupOutputBuilder,
) -> crate::operation::list_resources_in_protection_group::builders::ListResourcesInProtectionGroupOutputBuilder {
    if builder.resource_arns.is_none() {
        builder.resource_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn protection_group_correct_errors(
    mut builder: crate::types::builders::ProtectionGroupBuilder,
) -> crate::types::builders::ProtectionGroupBuilder {
    if builder.protection_group_id.is_none() {
        builder.protection_group_id = Some(Default::default())
    }
    if builder.aggregation.is_none() {
        builder.aggregation = "no value was set".parse::<crate::types::ProtectionGroupAggregation>().ok()
    }
    if builder.pattern.is_none() {
        builder.pattern = "no value was set".parse::<crate::types::ProtectionGroupPattern>().ok()
    }
    if builder.members.is_none() {
        builder.members = Some(Default::default())
    }
    builder
}

pub(crate) fn subscription_correct_errors(mut builder: crate::types::builders::SubscriptionBuilder) -> crate::types::builders::SubscriptionBuilder {
    if builder.subscription_limits.is_none() {
        builder.subscription_limits = {
            let builder = crate::types::builders::SubscriptionLimitsBuilder::default();
            Some(crate::serde_util::subscription_limits_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn application_layer_automatic_response_configuration_correct_errors(
    mut builder: crate::types::builders::ApplicationLayerAutomaticResponseConfigurationBuilder,
) -> crate::types::builders::ApplicationLayerAutomaticResponseConfigurationBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ApplicationLayerAutomaticResponseStatus>().ok()
    }
    if builder.action.is_none() {
        builder.action = {
            let builder = crate::types::builders::ResponseActionBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn attack_statistics_data_item_correct_errors(
    mut builder: crate::types::builders::AttackStatisticsDataItemBuilder,
) -> crate::types::builders::AttackStatisticsDataItemBuilder {
    if builder.attack_count.is_none() {
        builder.attack_count = Some(Default::default())
    }
    builder
}

pub(crate) fn emergency_contact_correct_errors(
    mut builder: crate::types::builders::EmergencyContactBuilder,
) -> crate::types::builders::EmergencyContactBuilder {
    if builder.email_address.is_none() {
        builder.email_address = Some(Default::default())
    }
    builder
}

pub(crate) fn subscription_limits_correct_errors(
    mut builder: crate::types::builders::SubscriptionLimitsBuilder,
) -> crate::types::builders::SubscriptionLimitsBuilder {
    if builder.protection_limits.is_none() {
        builder.protection_limits = {
            let builder = crate::types::builders::ProtectionLimitsBuilder::default();
            crate::serde_util::protection_limits_correct_errors(builder).build().ok()
        }
    }
    if builder.protection_group_limits.is_none() {
        builder.protection_group_limits = {
            let builder = crate::types::builders::ProtectionGroupLimitsBuilder::default();
            Some(crate::serde_util::protection_group_limits_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn validation_exception_field_correct_errors(
    mut builder: crate::types::builders::ValidationExceptionFieldBuilder,
) -> crate::types::builders::ValidationExceptionFieldBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn protection_group_limits_correct_errors(
    mut builder: crate::types::builders::ProtectionGroupLimitsBuilder,
) -> crate::types::builders::ProtectionGroupLimitsBuilder {
    if builder.max_protection_groups.is_none() {
        builder.max_protection_groups = Some(Default::default())
    }
    if builder.pattern_type_limits.is_none() {
        builder.pattern_type_limits = {
            let builder = crate::types::builders::ProtectionGroupPatternTypeLimitsBuilder::default();
            Some(crate::serde_util::protection_group_pattern_type_limits_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn protection_limits_correct_errors(
    mut builder: crate::types::builders::ProtectionLimitsBuilder,
) -> crate::types::builders::ProtectionLimitsBuilder {
    if builder.protected_resource_type_limits.is_none() {
        builder.protected_resource_type_limits = Some(Default::default())
    }
    builder
}

pub(crate) fn attack_vector_description_correct_errors(
    mut builder: crate::types::builders::AttackVectorDescriptionBuilder,
) -> crate::types::builders::AttackVectorDescriptionBuilder {
    if builder.vector_type.is_none() {
        builder.vector_type = Some(Default::default())
    }
    builder
}

pub(crate) fn attack_volume_statistics_correct_errors(
    mut builder: crate::types::builders::AttackVolumeStatisticsBuilder,
) -> crate::types::builders::AttackVolumeStatisticsBuilder {
    if builder.max.is_none() {
        builder.max = Some(Default::default())
    }
    builder
}

pub(crate) fn protection_group_pattern_type_limits_correct_errors(
    mut builder: crate::types::builders::ProtectionGroupPatternTypeLimitsBuilder,
) -> crate::types::builders::ProtectionGroupPatternTypeLimitsBuilder {
    if builder.arbitrary_pattern_limits.is_none() {
        builder.arbitrary_pattern_limits = {
            let builder = crate::types::builders::ProtectionGroupArbitraryPatternLimitsBuilder::default();
            Some(crate::serde_util::protection_group_arbitrary_pattern_limits_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn protection_group_arbitrary_pattern_limits_correct_errors(
    mut builder: crate::types::builders::ProtectionGroupArbitraryPatternLimitsBuilder,
) -> crate::types::builders::ProtectionGroupArbitraryPatternLimitsBuilder {
    if builder.max_members.is_none() {
        builder.max_members = Some(Default::default())
    }
    builder
}

pub(crate) fn summarized_attack_vector_correct_errors(
    mut builder: crate::types::builders::SummarizedAttackVectorBuilder,
) -> crate::types::builders::SummarizedAttackVectorBuilder {
    if builder.vector_type.is_none() {
        builder.vector_type = Some(Default::default())
    }
    builder
}
