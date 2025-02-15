// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutManagedInsightRules`](crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`managed_rules(ManagedRule)`](crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesFluentBuilder::managed_rules) / [`set_managed_rules(Option<Vec::<ManagedRule>>)`](crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesFluentBuilder::set_managed_rules):<br>required: **true**<br><p> A list of <code>ManagedRules</code> to enable. </p><br>
    /// - On success, responds with [`PutManagedInsightRulesOutput`](crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput) with field(s):
    ///   - [`failures(Option<Vec::<PartialFailure>>)`](crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput::failures): <p> An array that lists the rules that could not be enabled. </p>
    /// - On failure, responds with [`SdkError<PutManagedInsightRulesError>`](crate::operation::put_managed_insight_rules::PutManagedInsightRulesError)
    pub fn put_managed_insight_rules(&self) -> crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesFluentBuilder {
        crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesFluentBuilder::new(self.handle.clone())
    }
}
