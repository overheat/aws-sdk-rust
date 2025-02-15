// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConformancePackComplianceSummary`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`conformance_pack_names(impl Into<String>)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::conformance_pack_names) / [`set_conformance_pack_names(Option<Vec::<String>>)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::set_conformance_pack_names):<br>required: **true**<br><p>Names of conformance packs.</p><br>
    ///   - [`limit(i32)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of conformance packs returned on each page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::set_next_token):<br>required: **false**<br><p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p><br>
    /// - On success, responds with [`GetConformancePackComplianceSummaryOutput`](crate::operation::get_conformance_pack_compliance_summary::GetConformancePackComplianceSummaryOutput) with field(s):
    ///   - [`conformance_pack_compliance_summary_list(Option<Vec::<ConformancePackComplianceSummary>>)`](crate::operation::get_conformance_pack_compliance_summary::GetConformancePackComplianceSummaryOutput::conformance_pack_compliance_summary_list): <p>A list of <code>ConformancePackComplianceSummary</code> objects. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_conformance_pack_compliance_summary::GetConformancePackComplianceSummaryOutput::next_token): <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<GetConformancePackComplianceSummaryError>`](crate::operation::get_conformance_pack_compliance_summary::GetConformancePackComplianceSummaryError)
    pub fn get_conformance_pack_compliance_summary(
        &self,
    ) -> crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder {
        crate::operation::get_conformance_pack_compliance_summary::builders::GetConformancePackComplianceSummaryFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
