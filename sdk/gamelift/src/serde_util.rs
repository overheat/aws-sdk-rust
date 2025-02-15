// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_matchmaking_rule_set_output_correct_errors(
    mut builder: crate::operation::create_matchmaking_rule_set::builders::CreateMatchmakingRuleSetOutputBuilder,
) -> crate::operation::create_matchmaking_rule_set::builders::CreateMatchmakingRuleSetOutputBuilder {
    if builder.rule_set.is_none() {
        builder.rule_set = {
            let builder = crate::types::builders::MatchmakingRuleSetBuilder::default();
            Some(crate::serde_util::matchmaking_rule_set_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn describe_matchmaking_rule_sets_output_correct_errors(
    mut builder: crate::operation::describe_matchmaking_rule_sets::builders::DescribeMatchmakingRuleSetsOutputBuilder,
) -> crate::operation::describe_matchmaking_rule_sets::builders::DescribeMatchmakingRuleSetsOutputBuilder {
    if builder.rule_sets.is_none() {
        builder.rule_sets = Some(Default::default())
    }
    builder
}

pub(crate) fn matchmaking_rule_set_correct_errors(
    mut builder: crate::types::builders::MatchmakingRuleSetBuilder,
) -> crate::types::builders::MatchmakingRuleSetBuilder {
    if builder.rule_set_body.is_none() {
        builder.rule_set_body = Some(Default::default())
    }
    builder
}

pub(crate) fn anywhere_configuration_correct_errors(
    mut builder: crate::types::builders::AnywhereConfigurationBuilder,
) -> crate::types::builders::AnywhereConfigurationBuilder {
    if builder.cost.is_none() {
        builder.cost = Some(Default::default())
    }
    builder
}

pub(crate) fn certificate_configuration_correct_errors(
    mut builder: crate::types::builders::CertificateConfigurationBuilder,
) -> crate::types::builders::CertificateConfigurationBuilder {
    if builder.certificate_type.is_none() {
        builder.certificate_type = "no value was set".parse::<crate::types::CertificateType>().ok()
    }
    builder
}

pub(crate) fn ip_permission_correct_errors(mut builder: crate::types::builders::IpPermissionBuilder) -> crate::types::builders::IpPermissionBuilder {
    if builder.from_port.is_none() {
        builder.from_port = Some(Default::default())
    }
    if builder.to_port.is_none() {
        builder.to_port = Some(Default::default())
    }
    if builder.ip_range.is_none() {
        builder.ip_range = Some(Default::default())
    }
    if builder.protocol.is_none() {
        builder.protocol = "no value was set".parse::<crate::types::IpProtocol>().ok()
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn game_property_correct_errors(mut builder: crate::types::builders::GamePropertyBuilder) -> crate::types::builders::GamePropertyBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn instance_definition_correct_errors(
    mut builder: crate::types::builders::InstanceDefinitionBuilder,
) -> crate::types::builders::InstanceDefinitionBuilder {
    if builder.instance_type.is_none() {
        builder.instance_type = "no value was set".parse::<crate::types::GameServerGroupInstanceType>().ok()
    }
    builder
}

pub(crate) fn server_process_correct_errors(
    mut builder: crate::types::builders::ServerProcessBuilder,
) -> crate::types::builders::ServerProcessBuilder {
    if builder.launch_path.is_none() {
        builder.launch_path = Some(Default::default())
    }
    if builder.concurrent_executions.is_none() {
        builder.concurrent_executions = Some(Default::default())
    }
    builder
}

pub(crate) fn target_configuration_correct_errors(
    mut builder: crate::types::builders::TargetConfigurationBuilder,
) -> crate::types::builders::TargetConfigurationBuilder {
    if builder.target_value.is_none() {
        builder.target_value = Some(Default::default())
    }
    builder
}
