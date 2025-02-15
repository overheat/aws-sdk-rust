// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_batch_get_traces_output_next_token(
    input: &crate::operation::batch_get_traces::BatchGetTracesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_groups_output_next_token(
    input: &crate::operation::get_groups::GetGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_insight_events_output_next_token(
    input: &crate::operation::get_insight_events::GetInsightEventsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_insight_summaries_output_next_token(
    input: &crate::operation::get_insight_summaries::GetInsightSummariesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_sampling_rules_output_next_token(
    input: &crate::operation::get_sampling_rules::GetSamplingRulesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_sampling_statistic_summaries_output_next_token(
    input: &crate::operation::get_sampling_statistic_summaries::GetSamplingStatisticSummariesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_service_graph_output_next_token(
    input: &crate::operation::get_service_graph::GetServiceGraphOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_time_series_service_statistics_output_next_token(
    input: &crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_trace_graph_output_next_token(
    input: &crate::operation::get_trace_graph::GetTraceGraphOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_get_trace_summaries_output_next_token(
    input: &crate::operation::get_trace_summaries::GetTraceSummariesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resource_policies_output_next_token(
    input: &crate::operation::list_resource_policies::ListResourcePoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_tags_for_resource_output_next_token(
    input: &crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_batch_get_traces_output_traces(
    input: crate::operation::batch_get_traces::BatchGetTracesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Trace>> {
    let input = match input.traces {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_groups_output_groups(
    input: crate::operation::get_groups::GetGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GroupSummary>> {
    let input = match input.groups {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_sampling_rules_output_sampling_rule_records(
    input: crate::operation::get_sampling_rules::GetSamplingRulesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SamplingRuleRecord>> {
    let input = match input.sampling_rule_records {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_sampling_statistic_summaries_output_sampling_statistic_summaries(
    input: crate::operation::get_sampling_statistic_summaries::GetSamplingStatisticSummariesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SamplingStatisticSummary>> {
    let input = match input.sampling_statistic_summaries {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_service_graph_output_services(
    input: crate::operation::get_service_graph::GetServiceGraphOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Service>> {
    let input = match input.services {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_time_series_service_statistics_output_time_series_service_statistics(
    input: crate::operation::get_time_series_service_statistics::GetTimeSeriesServiceStatisticsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::TimeSeriesServiceStatistics>> {
    let input = match input.time_series_service_statistics {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_trace_graph_output_services(
    input: crate::operation::get_trace_graph::GetTraceGraphOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Service>> {
    let input = match input.services {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_trace_summaries_output_trace_summaries(
    input: crate::operation::get_trace_summaries::GetTraceSummariesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::TraceSummary>> {
    let input = match input.trace_summaries {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_resource_policies_output_resource_policies(
    input: crate::operation::list_resource_policies::ListResourcePoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ResourcePolicy>> {
    let input = match input.resource_policies {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_tags_for_resource_output_tags(
    input: crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
    let input = match input.tags {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
