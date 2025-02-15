// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateFileSystemAliases`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::set_client_request_token):<br>required: **false**<br><p>(Optional) An idempotency token for resource creation, in a string of up to 63 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p><br>
    ///   - [`file_system_id(impl Into<String>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::file_system_id) / [`set_file_system_id(Option<String>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::set_file_system_id):<br>required: **true**<br><p>Specifies the file system with which you want to associate one or more DNS aliases.</p><br>
    ///   - [`aliases(impl Into<String>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::aliases) / [`set_aliases(Option<Vec::<String>>)`](crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::set_aliases):<br>required: **true**<br><p>An array of one or more DNS alias names to associate with the file system. The alias name has to comply with the following formatting requirements:</p>  <ul>   <li> <p>Formatted as a fully-qualified domain name (FQDN), <i> <code>hostname.domain</code> </i>, for example, <code>accounting.corp.example.com</code>.</p> </li>   <li> <p>Can contain alphanumeric characters and the hyphen (-).</p> </li>   <li> <p>Cannot start or end with a hyphen.</p> </li>   <li> <p>Can start with a numeric.</p> </li>  </ul>  <p>For DNS alias names, Amazon FSx stores alphabetic characters as lowercase letters (a-z), regardless of how you specify them: as uppercase letters, lowercase letters, or the corresponding letters in escape codes.</p><br>
    /// - On success, responds with [`AssociateFileSystemAliasesOutput`](crate::operation::associate_file_system_aliases::AssociateFileSystemAliasesOutput) with field(s):
    ///   - [`aliases(Option<Vec::<Alias>>)`](crate::operation::associate_file_system_aliases::AssociateFileSystemAliasesOutput::aliases): <p>An array of the DNS aliases that Amazon FSx is associating with the file system.</p>
    /// - On failure, responds with [`SdkError<AssociateFileSystemAliasesError>`](crate::operation::associate_file_system_aliases::AssociateFileSystemAliasesError)
    pub fn associate_file_system_aliases(
        &self,
    ) -> crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder {
        crate::operation::associate_file_system_aliases::builders::AssociateFileSystemAliasesFluentBuilder::new(self.handle.clone())
    }
}
