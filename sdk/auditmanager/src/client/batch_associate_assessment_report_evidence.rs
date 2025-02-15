// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchAssociateAssessmentReportEvidence`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`assessment_id(impl Into<String>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::assessment_id) / [`set_assessment_id(Option<String>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::set_assessment_id):<br>required: **true**<br><p> The identifier for the assessment. </p><br>
    ///   - [`evidence_folder_id(impl Into<String>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::evidence_folder_id) / [`set_evidence_folder_id(Option<String>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::set_evidence_folder_id):<br>required: **true**<br><p> The identifier for the folder that the evidence is stored in. </p><br>
    ///   - [`evidence_ids(impl Into<String>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::evidence_ids) / [`set_evidence_ids(Option<Vec::<String>>)`](crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::set_evidence_ids):<br>required: **true**<br><p> The list of evidence identifiers. </p><br>
    /// - On success, responds with [`BatchAssociateAssessmentReportEvidenceOutput`](crate::operation::batch_associate_assessment_report_evidence::BatchAssociateAssessmentReportEvidenceOutput) with field(s):
    ///   - [`evidence_ids(Option<Vec::<String>>)`](crate::operation::batch_associate_assessment_report_evidence::BatchAssociateAssessmentReportEvidenceOutput::evidence_ids): <p> The list of evidence identifiers. </p>
    ///   - [`errors(Option<Vec::<AssessmentReportEvidenceError>>)`](crate::operation::batch_associate_assessment_report_evidence::BatchAssociateAssessmentReportEvidenceOutput::errors): <p> A list of errors that the <code>BatchAssociateAssessmentReportEvidence</code> API returned. </p>
    /// - On failure, responds with [`SdkError<BatchAssociateAssessmentReportEvidenceError>`](crate::operation::batch_associate_assessment_report_evidence::BatchAssociateAssessmentReportEvidenceError)
    pub fn batch_associate_assessment_report_evidence(
        &self,
    ) -> crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder {
        crate::operation::batch_associate_assessment_report_evidence::builders::BatchAssociateAssessmentReportEvidenceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
