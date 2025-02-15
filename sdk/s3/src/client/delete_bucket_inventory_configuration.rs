// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketInventoryConfiguration`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket containing the inventory configuration to delete.</p><br>
    ///   - [`id(impl Into<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::set_id):<br>required: **true**<br><p>The ID used to identify the inventory configuration.</p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`DeleteBucketInventoryConfigurationOutput`](crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketInventoryConfigurationError>`](crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationError)
    pub fn delete_bucket_inventory_configuration(
        &self,
    ) -> crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder {
        crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationFluentBuilder::new(self.handle.clone())
    }
}
