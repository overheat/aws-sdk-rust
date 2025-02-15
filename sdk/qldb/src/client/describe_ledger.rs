// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLedger`](crate::operation::describe_ledger::builders::DescribeLedgerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::describe_ledger::builders::DescribeLedgerFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::describe_ledger::builders::DescribeLedgerFluentBuilder::set_name):<br>required: **true**<br><p>The name of the ledger that you want to describe.</p><br>
    /// - On success, responds with [`DescribeLedgerOutput`](crate::operation::describe_ledger::DescribeLedgerOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::describe_ledger::DescribeLedgerOutput::name): <p>The name of the ledger.</p>
    ///   - [`arn(Option<String>)`](crate::operation::describe_ledger::DescribeLedgerOutput::arn): <p>The Amazon Resource Name (ARN) for the ledger.</p>
    ///   - [`state(Option<LedgerState>)`](crate::operation::describe_ledger::DescribeLedgerOutput::state): <p>The current status of the ledger.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::operation::describe_ledger::DescribeLedgerOutput::creation_date_time): <p>The date and time, in epoch time format, when the ledger was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    ///   - [`permissions_mode(Option<PermissionsMode>)`](crate::operation::describe_ledger::DescribeLedgerOutput::permissions_mode): <p>The permissions mode of the ledger.</p>
    ///   - [`deletion_protection(Option<bool>)`](crate::operation::describe_ledger::DescribeLedgerOutput::deletion_protection): <p>Specifies whether the ledger is protected from being deleted by any user. If not defined during ledger creation, this feature is enabled (<code>true</code>) by default.</p>  <p>If deletion protection is enabled, you must first disable it before you can delete the ledger. You can disable it by calling the <code>UpdateLedger</code> operation to set this parameter to <code>false</code>.</p>
    ///   - [`encryption_description(Option<LedgerEncryptionDescription>)`](crate::operation::describe_ledger::DescribeLedgerOutput::encryption_description): <p>Information about the encryption of data at rest in the ledger. This includes the current status, the KMS key, and when the key became inaccessible (in the case of an error).</p>
    /// - On failure, responds with [`SdkError<DescribeLedgerError>`](crate::operation::describe_ledger::DescribeLedgerError)
    pub fn describe_ledger(&self) -> crate::operation::describe_ledger::builders::DescribeLedgerFluentBuilder {
        crate::operation::describe_ledger::builders::DescribeLedgerFluentBuilder::new(self.handle.clone())
    }
}
