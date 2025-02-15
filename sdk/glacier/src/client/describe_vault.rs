// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVault`](crate::operation::describe_vault::builders::DescribeVaultFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::describe_vault::builders::DescribeVaultFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::describe_vault::builders::DescribeVaultFluentBuilder::set_account_id):<br>required: **true**<br><p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p><br>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::describe_vault::builders::DescribeVaultFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::describe_vault::builders::DescribeVaultFluentBuilder::set_vault_name):<br>required: **true**<br><p>The name of the vault.</p><br>
    /// - On success, responds with [`DescribeVaultOutput`](crate::operation::describe_vault::DescribeVaultOutput) with field(s):
    ///   - [`vault_arn(Option<String>)`](crate::operation::describe_vault::DescribeVaultOutput::vault_arn): <p>The Amazon Resource Name (ARN) of the vault.</p>
    ///   - [`vault_name(Option<String>)`](crate::operation::describe_vault::DescribeVaultOutput::vault_name): <p>The name of the vault.</p>
    ///   - [`creation_date(Option<String>)`](crate::operation::describe_vault::DescribeVaultOutput::creation_date): <p>The Universal Coordinated Time (UTC) date when the vault was created. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
    ///   - [`last_inventory_date(Option<String>)`](crate::operation::describe_vault::DescribeVaultOutput::last_inventory_date): <p>The Universal Coordinated Time (UTC) date when Amazon S3 Glacier completed the last vault inventory. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
    ///   - [`number_of_archives(i64)`](crate::operation::describe_vault::DescribeVaultOutput::number_of_archives): <p>The number of archives in the vault as of the last inventory date. This field will return <code>null</code> if an inventory has not yet run on the vault, for example if you just created the vault.</p>
    ///   - [`size_in_bytes(i64)`](crate::operation::describe_vault::DescribeVaultOutput::size_in_bytes): <p>Total size, in bytes, of the archives in the vault as of the last inventory date. This field will return null if an inventory has not yet run on the vault, for example if you just created the vault.</p>
    /// - On failure, responds with [`SdkError<DescribeVaultError>`](crate::operation::describe_vault::DescribeVaultError)
    pub fn describe_vault(&self) -> crate::operation::describe_vault::builders::DescribeVaultFluentBuilder {
        crate::operation::describe_vault::builders::DescribeVaultFluentBuilder::new(self.handle.clone())
    }
}
