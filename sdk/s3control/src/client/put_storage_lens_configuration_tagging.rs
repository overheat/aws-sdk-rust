// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutStorageLensConfigurationTagging`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`config_id(impl Into<String>)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::config_id) / [`set_config_id(Option<String>)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::set_config_id):<br>required: **true**<br><p>The ID of the S3 Storage Lens configuration.</p><br>
    ///   - [`account_id(impl Into<String>)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::set_account_id):<br>required: **true**<br><p>The account ID of the requester.</p><br>
    ///   - [`tags(StorageLensTag)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::tags) / [`set_tags(Option<Vec::<StorageLensTag>>)`](crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::set_tags):<br>required: **true**<br><p>The tag set of the S3 Storage Lens configuration.</p> <note>   <p>You can set up to a maximum of 50 tags.</p>  </note><br>
    /// - On success, responds with [`PutStorageLensConfigurationTaggingOutput`](crate::operation::put_storage_lens_configuration_tagging::PutStorageLensConfigurationTaggingOutput)
    /// - On failure, responds with [`SdkError<PutStorageLensConfigurationTaggingError>`](crate::operation::put_storage_lens_configuration_tagging::PutStorageLensConfigurationTaggingError)
    pub fn put_storage_lens_configuration_tagging(
        &self,
    ) -> crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder {
        crate::operation::put_storage_lens_configuration_tagging::builders::PutStorageLensConfigurationTaggingFluentBuilder::new(self.handle.clone())
    }
}
