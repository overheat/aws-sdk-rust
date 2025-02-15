// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRecoveryPointRestoreMetadata`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_vault_name(impl Into<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::backup_vault_name) / [`set_backup_vault_name(Option<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::set_backup_vault_name):<br>required: **true**<br><p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p><br>
    ///   - [`recovery_point_arn(impl Into<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::recovery_point_arn) / [`set_recovery_point_arn(Option<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::set_recovery_point_arn):<br>required: **true**<br><p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p><br>
    ///   - [`backup_vault_account_id(impl Into<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::backup_vault_account_id) / [`set_backup_vault_account_id(Option<String>)`](crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::set_backup_vault_account_id):<br>required: **false**<br><p>This is the account ID of the specified backup vault.</p><br>
    /// - On success, responds with [`GetRecoveryPointRestoreMetadataOutput`](crate::operation::get_recovery_point_restore_metadata::GetRecoveryPointRestoreMetadataOutput) with field(s):
    ///   - [`backup_vault_arn(Option<String>)`](crate::operation::get_recovery_point_restore_metadata::GetRecoveryPointRestoreMetadataOutput::backup_vault_arn): <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    ///   - [`recovery_point_arn(Option<String>)`](crate::operation::get_recovery_point_restore_metadata::GetRecoveryPointRestoreMetadataOutput::recovery_point_arn): <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    ///   - [`restore_metadata(Option<HashMap::<String, String>>)`](crate::operation::get_recovery_point_restore_metadata::GetRecoveryPointRestoreMetadataOutput::restore_metadata): <p>The set of metadata key-value pairs that describe the original configuration of the backed-up resource. These values vary depending on the service that is being restored.</p>
    /// - On failure, responds with [`SdkError<GetRecoveryPointRestoreMetadataError>`](crate::operation::get_recovery_point_restore_metadata::GetRecoveryPointRestoreMetadataError)
    pub fn get_recovery_point_restore_metadata(
        &self,
    ) -> crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder {
        crate::operation::get_recovery_point_restore_metadata::builders::GetRecoveryPointRestoreMetadataFluentBuilder::new(self.handle.clone())
    }
}
