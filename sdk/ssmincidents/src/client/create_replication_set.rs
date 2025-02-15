// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateReplicationSet`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`regions(impl Into<String>, RegionMapInputValue)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::regions) / [`set_regions(Option<HashMap::<String, RegionMapInputValue>>)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::set_regions):<br>required: **true**<br><p>The Regions that Incident Manager replicates your data to. You can have up to three Regions in your replication set.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::set_client_token):<br>required: **false**<br><p>A token that ensures that the operation is called only once with the specified details.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::set_tags):<br>required: **false**<br><p>A list of tags to add to the replication set.</p><br>
    /// - On success, responds with [`CreateReplicationSetOutput`](crate::operation::create_replication_set::CreateReplicationSetOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::create_replication_set::CreateReplicationSetOutput::arn): <p>The Amazon Resource Name (ARN) of the replication set. </p>
    /// - On failure, responds with [`SdkError<CreateReplicationSetError>`](crate::operation::create_replication_set::CreateReplicationSetError)
    pub fn create_replication_set(&self) -> crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder {
        crate::operation::create_replication_set::builders::CreateReplicationSetFluentBuilder::new(self.handle.clone())
    }
}
