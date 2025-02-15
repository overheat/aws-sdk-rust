// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.error_code.is_none() {
        builder.error_code = "no value was set".parse::<crate::types::AccessDeniedErrorCode>().ok()
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalExceptionBuilder,
) -> crate::types::error::builders::InternalExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn invalid_input_exception_correct_errors(
    mut builder: crate::types::error::builders::InvalidInputExceptionBuilder,
) -> crate::types::error::builders::InvalidInputExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.error_code.is_none() {
        builder.error_code = "no value was set".parse::<crate::types::InvalidInputErrorCode>().ok()
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn no_such_entity_exception_correct_errors(
    mut builder: crate::types::error::builders::NoSuchEntityExceptionBuilder,
) -> crate::types::error::builders::NoSuchEntityExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.error_code.is_none() {
        builder.error_code = "no value was set".parse::<crate::types::NoSuchEntityErrorCode>().ok()
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn service_temporarily_unavailable_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceTemporarilyUnavailableExceptionBuilder,
) -> crate::types::error::builders::ServiceTemporarilyUnavailableExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn add_attributes_to_findings_output_correct_errors(
    mut builder: crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsOutputBuilder,
) -> crate::operation::add_attributes_to_findings::builders::AddAttributesToFindingsOutputBuilder {
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn invalid_cross_account_role_exception_correct_errors(
    mut builder: crate::types::error::builders::InvalidCrossAccountRoleExceptionBuilder,
) -> crate::types::error::builders::InvalidCrossAccountRoleExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.error_code.is_none() {
        builder.error_code = "no value was set".parse::<crate::types::InvalidCrossAccountRoleErrorCode>().ok()
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn limit_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::LimitExceededExceptionBuilder,
) -> crate::types::error::builders::LimitExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.error_code.is_none() {
        builder.error_code = "no value was set".parse::<crate::types::LimitExceededErrorCode>().ok()
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn create_assessment_target_output_correct_errors(
    mut builder: crate::operation::create_assessment_target::builders::CreateAssessmentTargetOutputBuilder,
) -> crate::operation::create_assessment_target::builders::CreateAssessmentTargetOutputBuilder {
    if builder.assessment_target_arn.is_none() {
        builder.assessment_target_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn create_assessment_template_output_correct_errors(
    mut builder: crate::operation::create_assessment_template::builders::CreateAssessmentTemplateOutputBuilder,
) -> crate::operation::create_assessment_template::builders::CreateAssessmentTemplateOutputBuilder {
    if builder.assessment_template_arn.is_none() {
        builder.assessment_template_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn preview_generation_in_progress_exception_correct_errors(
    mut builder: crate::types::error::builders::PreviewGenerationInProgressExceptionBuilder,
) -> crate::types::error::builders::PreviewGenerationInProgressExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn create_exclusions_preview_output_correct_errors(
    mut builder: crate::operation::create_exclusions_preview::builders::CreateExclusionsPreviewOutputBuilder,
) -> crate::operation::create_exclusions_preview::builders::CreateExclusionsPreviewOutputBuilder {
    if builder.preview_token.is_none() {
        builder.preview_token = Some(Default::default())
    }
    builder
}

pub(crate) fn create_resource_group_output_correct_errors(
    mut builder: crate::operation::create_resource_group::builders::CreateResourceGroupOutputBuilder,
) -> crate::operation::create_resource_group::builders::CreateResourceGroupOutputBuilder {
    if builder.resource_group_arn.is_none() {
        builder.resource_group_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_run_in_progress_exception_correct_errors(
    mut builder: crate::types::error::builders::AssessmentRunInProgressExceptionBuilder,
) -> crate::types::error::builders::AssessmentRunInProgressExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.assessment_run_arns.is_none() {
        builder.assessment_run_arns = Some(Default::default())
    }
    if builder.assessment_run_arns_truncated.is_none() {
        builder.assessment_run_arns_truncated = Some(Default::default())
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_assessment_runs_output_correct_errors(
    mut builder: crate::operation::describe_assessment_runs::builders::DescribeAssessmentRunsOutputBuilder,
) -> crate::operation::describe_assessment_runs::builders::DescribeAssessmentRunsOutputBuilder {
    if builder.assessment_runs.is_none() {
        builder.assessment_runs = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_assessment_targets_output_correct_errors(
    mut builder: crate::operation::describe_assessment_targets::builders::DescribeAssessmentTargetsOutputBuilder,
) -> crate::operation::describe_assessment_targets::builders::DescribeAssessmentTargetsOutputBuilder {
    if builder.assessment_targets.is_none() {
        builder.assessment_targets = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_assessment_templates_output_correct_errors(
    mut builder: crate::operation::describe_assessment_templates::builders::DescribeAssessmentTemplatesOutputBuilder,
) -> crate::operation::describe_assessment_templates::builders::DescribeAssessmentTemplatesOutputBuilder {
    if builder.assessment_templates.is_none() {
        builder.assessment_templates = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_cross_account_access_role_output_correct_errors(
    mut builder: crate::operation::describe_cross_account_access_role::builders::DescribeCrossAccountAccessRoleOutputBuilder,
) -> crate::operation::describe_cross_account_access_role::builders::DescribeCrossAccountAccessRoleOutputBuilder {
    if builder.role_arn.is_none() {
        builder.role_arn = Some(Default::default())
    }
    if builder.valid.is_none() {
        builder.valid = Some(Default::default())
    }
    if builder.registered_at.is_none() {
        builder.registered_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn describe_exclusions_output_correct_errors(
    mut builder: crate::operation::describe_exclusions::builders::DescribeExclusionsOutputBuilder,
) -> crate::operation::describe_exclusions::builders::DescribeExclusionsOutputBuilder {
    if builder.exclusions.is_none() {
        builder.exclusions = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_findings_output_correct_errors(
    mut builder: crate::operation::describe_findings::builders::DescribeFindingsOutputBuilder,
) -> crate::operation::describe_findings::builders::DescribeFindingsOutputBuilder {
    if builder.findings.is_none() {
        builder.findings = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_resource_groups_output_correct_errors(
    mut builder: crate::operation::describe_resource_groups::builders::DescribeResourceGroupsOutputBuilder,
) -> crate::operation::describe_resource_groups::builders::DescribeResourceGroupsOutputBuilder {
    if builder.resource_groups.is_none() {
        builder.resource_groups = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_rules_packages_output_correct_errors(
    mut builder: crate::operation::describe_rules_packages::builders::DescribeRulesPackagesOutputBuilder,
) -> crate::operation::describe_rules_packages::builders::DescribeRulesPackagesOutputBuilder {
    if builder.rules_packages.is_none() {
        builder.rules_packages = Some(Default::default())
    }
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn unsupported_feature_exception_correct_errors(
    mut builder: crate::types::error::builders::UnsupportedFeatureExceptionBuilder,
) -> crate::types::error::builders::UnsupportedFeatureExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn get_assessment_report_output_correct_errors(
    mut builder: crate::operation::get_assessment_report::builders::GetAssessmentReportOutputBuilder,
) -> crate::operation::get_assessment_report::builders::GetAssessmentReportOutputBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ReportStatus>().ok()
    }
    builder
}

pub(crate) fn get_exclusions_preview_output_correct_errors(
    mut builder: crate::operation::get_exclusions_preview::builders::GetExclusionsPreviewOutputBuilder,
) -> crate::operation::get_exclusions_preview::builders::GetExclusionsPreviewOutputBuilder {
    if builder.preview_status.is_none() {
        builder.preview_status = "no value was set".parse::<crate::types::PreviewStatus>().ok()
    }
    builder
}

pub(crate) fn get_telemetry_metadata_output_correct_errors(
    mut builder: crate::operation::get_telemetry_metadata::builders::GetTelemetryMetadataOutputBuilder,
) -> crate::operation::get_telemetry_metadata::builders::GetTelemetryMetadataOutputBuilder {
    if builder.telemetry_metadata.is_none() {
        builder.telemetry_metadata = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assessment_run_agents_output_correct_errors(
    mut builder: crate::operation::list_assessment_run_agents::builders::ListAssessmentRunAgentsOutputBuilder,
) -> crate::operation::list_assessment_run_agents::builders::ListAssessmentRunAgentsOutputBuilder {
    if builder.assessment_run_agents.is_none() {
        builder.assessment_run_agents = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assessment_runs_output_correct_errors(
    mut builder: crate::operation::list_assessment_runs::builders::ListAssessmentRunsOutputBuilder,
) -> crate::operation::list_assessment_runs::builders::ListAssessmentRunsOutputBuilder {
    if builder.assessment_run_arns.is_none() {
        builder.assessment_run_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assessment_targets_output_correct_errors(
    mut builder: crate::operation::list_assessment_targets::builders::ListAssessmentTargetsOutputBuilder,
) -> crate::operation::list_assessment_targets::builders::ListAssessmentTargetsOutputBuilder {
    if builder.assessment_target_arns.is_none() {
        builder.assessment_target_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn list_assessment_templates_output_correct_errors(
    mut builder: crate::operation::list_assessment_templates::builders::ListAssessmentTemplatesOutputBuilder,
) -> crate::operation::list_assessment_templates::builders::ListAssessmentTemplatesOutputBuilder {
    if builder.assessment_template_arns.is_none() {
        builder.assessment_template_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn list_event_subscriptions_output_correct_errors(
    mut builder: crate::operation::list_event_subscriptions::builders::ListEventSubscriptionsOutputBuilder,
) -> crate::operation::list_event_subscriptions::builders::ListEventSubscriptionsOutputBuilder {
    if builder.subscriptions.is_none() {
        builder.subscriptions = Some(Default::default())
    }
    builder
}

pub(crate) fn list_exclusions_output_correct_errors(
    mut builder: crate::operation::list_exclusions::builders::ListExclusionsOutputBuilder,
) -> crate::operation::list_exclusions::builders::ListExclusionsOutputBuilder {
    if builder.exclusion_arns.is_none() {
        builder.exclusion_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn list_findings_output_correct_errors(
    mut builder: crate::operation::list_findings::builders::ListFindingsOutputBuilder,
) -> crate::operation::list_findings::builders::ListFindingsOutputBuilder {
    if builder.finding_arns.is_none() {
        builder.finding_arns = Some(Default::default())
    }
    builder
}

pub(crate) fn list_rules_packages_output_correct_errors(
    mut builder: crate::operation::list_rules_packages::builders::ListRulesPackagesOutputBuilder,
) -> crate::operation::list_rules_packages::builders::ListRulesPackagesOutputBuilder {
    if builder.rules_package_arns.is_none() {
        builder.rules_package_arns = Some(Default::default())
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

pub(crate) fn preview_agents_output_correct_errors(
    mut builder: crate::operation::preview_agents::builders::PreviewAgentsOutputBuilder,
) -> crate::operation::preview_agents::builders::PreviewAgentsOutputBuilder {
    if builder.agent_previews.is_none() {
        builder.agent_previews = Some(Default::default())
    }
    builder
}

pub(crate) fn remove_attributes_from_findings_output_correct_errors(
    mut builder: crate::operation::remove_attributes_from_findings::builders::RemoveAttributesFromFindingsOutputBuilder,
) -> crate::operation::remove_attributes_from_findings::builders::RemoveAttributesFromFindingsOutputBuilder {
    if builder.failed_items.is_none() {
        builder.failed_items = Some(Default::default())
    }
    builder
}

pub(crate) fn agents_already_running_assessment_exception_correct_errors(
    mut builder: crate::types::error::builders::AgentsAlreadyRunningAssessmentExceptionBuilder,
) -> crate::types::error::builders::AgentsAlreadyRunningAssessmentExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.agents.is_none() {
        builder.agents = Some(Default::default())
    }
    if builder.agents_truncated.is_none() {
        builder.agents_truncated = Some(Default::default())
    }
    if builder.can_retry.is_none() {
        builder.can_retry = Some(Default::default())
    }
    builder
}

pub(crate) fn start_assessment_run_output_correct_errors(
    mut builder: crate::operation::start_assessment_run::builders::StartAssessmentRunOutputBuilder,
) -> crate::operation::start_assessment_run::builders::StartAssessmentRunOutputBuilder {
    if builder.assessment_run_arn.is_none() {
        builder.assessment_run_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn agent_already_running_assessment_correct_errors(
    mut builder: crate::types::builders::AgentAlreadyRunningAssessmentBuilder,
) -> crate::types::builders::AgentAlreadyRunningAssessmentBuilder {
    if builder.agent_id.is_none() {
        builder.agent_id = Some(Default::default())
    }
    if builder.assessment_run_arn.is_none() {
        builder.assessment_run_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn agent_preview_correct_errors(mut builder: crate::types::builders::AgentPreviewBuilder) -> crate::types::builders::AgentPreviewBuilder {
    if builder.agent_id.is_none() {
        builder.agent_id = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_run_correct_errors(
    mut builder: crate::types::builders::AssessmentRunBuilder,
) -> crate::types::builders::AssessmentRunBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.assessment_template_arn.is_none() {
        builder.assessment_template_arn = Some(Default::default())
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::AssessmentRunState>().ok()
    }
    if builder.duration_in_seconds.is_none() {
        builder.duration_in_seconds = Some(Default::default())
    }
    if builder.rules_package_arns.is_none() {
        builder.rules_package_arns = Some(Default::default())
    }
    if builder.user_attributes_for_findings.is_none() {
        builder.user_attributes_for_findings = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state_changed_at.is_none() {
        builder.state_changed_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.data_collected.is_none() {
        builder.data_collected = Some(Default::default())
    }
    if builder.state_changes.is_none() {
        builder.state_changes = Some(Default::default())
    }
    if builder.notifications.is_none() {
        builder.notifications = Some(Default::default())
    }
    if builder.finding_counts.is_none() {
        builder.finding_counts = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_run_agent_correct_errors(
    mut builder: crate::types::builders::AssessmentRunAgentBuilder,
) -> crate::types::builders::AssessmentRunAgentBuilder {
    if builder.agent_id.is_none() {
        builder.agent_id = Some(Default::default())
    }
    if builder.assessment_run_arn.is_none() {
        builder.assessment_run_arn = Some(Default::default())
    }
    if builder.agent_health.is_none() {
        builder.agent_health = "no value was set".parse::<crate::types::AgentHealth>().ok()
    }
    if builder.agent_health_code.is_none() {
        builder.agent_health_code = "no value was set".parse::<crate::types::AgentHealthCode>().ok()
    }
    if builder.telemetry_metadata.is_none() {
        builder.telemetry_metadata = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_target_correct_errors(
    mut builder: crate::types::builders::AssessmentTargetBuilder,
) -> crate::types::builders::AssessmentTargetBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn assessment_template_correct_errors(
    mut builder: crate::types::builders::AssessmentTemplateBuilder,
) -> crate::types::builders::AssessmentTemplateBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.assessment_target_arn.is_none() {
        builder.assessment_target_arn = Some(Default::default())
    }
    if builder.duration_in_seconds.is_none() {
        builder.duration_in_seconds = Some(Default::default())
    }
    if builder.rules_package_arns.is_none() {
        builder.rules_package_arns = Some(Default::default())
    }
    if builder.user_attributes_for_findings.is_none() {
        builder.user_attributes_for_findings = Some(Default::default())
    }
    if builder.assessment_run_count.is_none() {
        builder.assessment_run_count = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn exclusion_correct_errors(mut builder: crate::types::builders::ExclusionBuilder) -> crate::types::builders::ExclusionBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.title.is_none() {
        builder.title = Some(Default::default())
    }
    if builder.description.is_none() {
        builder.description = Some(Default::default())
    }
    if builder.recommendation.is_none() {
        builder.recommendation = Some(Default::default())
    }
    if builder.scopes.is_none() {
        builder.scopes = Some(Default::default())
    }
    builder
}

pub(crate) fn exclusion_preview_correct_errors(
    mut builder: crate::types::builders::ExclusionPreviewBuilder,
) -> crate::types::builders::ExclusionPreviewBuilder {
    if builder.title.is_none() {
        builder.title = Some(Default::default())
    }
    if builder.description.is_none() {
        builder.description = Some(Default::default())
    }
    if builder.recommendation.is_none() {
        builder.recommendation = Some(Default::default())
    }
    if builder.scopes.is_none() {
        builder.scopes = Some(Default::default())
    }
    builder
}

pub(crate) fn failed_item_details_correct_errors(
    mut builder: crate::types::builders::FailedItemDetailsBuilder,
) -> crate::types::builders::FailedItemDetailsBuilder {
    if builder.failure_code.is_none() {
        builder.failure_code = "no value was set".parse::<crate::types::FailedItemErrorCode>().ok()
    }
    if builder.retryable.is_none() {
        builder.retryable = Some(Default::default())
    }
    builder
}

pub(crate) fn finding_correct_errors(mut builder: crate::types::builders::FindingBuilder) -> crate::types::builders::FindingBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.attributes.is_none() {
        builder.attributes = Some(Default::default())
    }
    if builder.user_attributes.is_none() {
        builder.user_attributes = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn resource_group_correct_errors(
    mut builder: crate::types::builders::ResourceGroupBuilder,
) -> crate::types::builders::ResourceGroupBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.tags.is_none() {
        builder.tags = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn rules_package_correct_errors(mut builder: crate::types::builders::RulesPackageBuilder) -> crate::types::builders::RulesPackageBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.version.is_none() {
        builder.version = Some(Default::default())
    }
    if builder.provider.is_none() {
        builder.provider = Some(Default::default())
    }
    builder
}

pub(crate) fn subscription_correct_errors(mut builder: crate::types::builders::SubscriptionBuilder) -> crate::types::builders::SubscriptionBuilder {
    if builder.resource_arn.is_none() {
        builder.resource_arn = Some(Default::default())
    }
    if builder.topic_arn.is_none() {
        builder.topic_arn = Some(Default::default())
    }
    if builder.event_subscriptions.is_none() {
        builder.event_subscriptions = Some(Default::default())
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn telemetry_metadata_correct_errors(
    mut builder: crate::types::builders::TelemetryMetadataBuilder,
) -> crate::types::builders::TelemetryMetadataBuilder {
    if builder.message_type.is_none() {
        builder.message_type = Some(Default::default())
    }
    if builder.count.is_none() {
        builder.count = Some(Default::default())
    }
    builder
}

pub(crate) fn asset_attributes_correct_errors(
    mut builder: crate::types::builders::AssetAttributesBuilder,
) -> crate::types::builders::AssetAttributesBuilder {
    if builder.schema_version.is_none() {
        builder.schema_version = Some(Default::default())
    }
    builder
}

pub(crate) fn inspector_service_attributes_correct_errors(
    mut builder: crate::types::builders::InspectorServiceAttributesBuilder,
) -> crate::types::builders::InspectorServiceAttributesBuilder {
    if builder.schema_version.is_none() {
        builder.schema_version = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_run_notification_correct_errors(
    mut builder: crate::types::builders::AssessmentRunNotificationBuilder,
) -> crate::types::builders::AssessmentRunNotificationBuilder {
    if builder.date.is_none() {
        builder.date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.event.is_none() {
        builder.event = "no value was set".parse::<crate::types::InspectorEvent>().ok()
    }
    if builder.error.is_none() {
        builder.error = Some(Default::default())
    }
    builder
}

pub(crate) fn assessment_run_state_change_correct_errors(
    mut builder: crate::types::builders::AssessmentRunStateChangeBuilder,
) -> crate::types::builders::AssessmentRunStateChangeBuilder {
    if builder.state_changed_at.is_none() {
        builder.state_changed_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::AssessmentRunState>().ok()
    }
    builder
}

pub(crate) fn attribute_correct_errors(mut builder: crate::types::builders::AttributeBuilder) -> crate::types::builders::AttributeBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn event_subscription_correct_errors(
    mut builder: crate::types::builders::EventSubscriptionBuilder,
) -> crate::types::builders::EventSubscriptionBuilder {
    if builder.event.is_none() {
        builder.event = "no value was set".parse::<crate::types::InspectorEvent>().ok()
    }
    if builder.subscribed_at.is_none() {
        builder.subscribed_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn resource_group_tag_correct_errors(
    mut builder: crate::types::builders::ResourceGroupTagBuilder,
) -> crate::types::builders::ResourceGroupTagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}
