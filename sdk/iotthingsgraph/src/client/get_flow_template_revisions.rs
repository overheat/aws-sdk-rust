// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetFlowTemplateRevisions`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::set_id):<br>required: **true**<br><p>The ID of the workflow.</p>  <p>The ID should be in the following format.</p>  <p> <code>urn:tdm:REGION/ACCOUNT ID/default:workflow:WORKFLOWNAME</code> </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The string that specifies the next page of results. Use this when you're paginating results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in the response.</p><br>
    /// - On success, responds with [`GetFlowTemplateRevisionsOutput`](crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput) with field(s):
    ///   - [`summaries(Option<Vec::<FlowTemplateSummary>>)`](crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput::summaries): <p>An array of objects that provide summary data about each revision.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsOutput::next_token): <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p>
    /// - On failure, responds with [`SdkError<GetFlowTemplateRevisionsError>`](crate::operation::get_flow_template_revisions::GetFlowTemplateRevisionsError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn get_flow_template_revisions(&self) -> crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder {
        crate::operation::get_flow_template_revisions::builders::GetFlowTemplateRevisionsFluentBuilder::new(self.handle.clone())
    }
}
