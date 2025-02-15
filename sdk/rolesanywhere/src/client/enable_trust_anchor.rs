// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableTrustAnchor`](crate::operation::enable_trust_anchor::builders::EnableTrustAnchorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trust_anchor_id(impl Into<String>)`](crate::operation::enable_trust_anchor::builders::EnableTrustAnchorFluentBuilder::trust_anchor_id) / [`set_trust_anchor_id(Option<String>)`](crate::operation::enable_trust_anchor::builders::EnableTrustAnchorFluentBuilder::set_trust_anchor_id):<br>required: **true**<br><p>The unique identifier of the trust anchor.</p><br>
    /// - On success, responds with [`EnableTrustAnchorOutput`](crate::operation::enable_trust_anchor::EnableTrustAnchorOutput) with field(s):
    ///   - [`trust_anchor(Option<TrustAnchorDetail>)`](crate::operation::enable_trust_anchor::EnableTrustAnchorOutput::trust_anchor): <p>The state of the trust anchor after a read or write operation. </p>
    /// - On failure, responds with [`SdkError<EnableTrustAnchorError>`](crate::operation::enable_trust_anchor::EnableTrustAnchorError)
    pub fn enable_trust_anchor(&self) -> crate::operation::enable_trust_anchor::builders::EnableTrustAnchorFluentBuilder {
        crate::operation::enable_trust_anchor::builders::EnableTrustAnchorFluentBuilder::new(self.handle.clone())
    }
}
