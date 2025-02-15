// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetResourceAccessForBucket`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_name(impl Into<String>)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::resource_name) / [`set_resource_name(Option<String>)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::set_resource_name):<br>required: **true**<br><p>The name of the Lightsail instance for which to set bucket access. The instance must be in a running or stopped state.</p><br>
    ///   - [`bucket_name(impl Into<String>)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::bucket_name) / [`set_bucket_name(Option<String>)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::set_bucket_name):<br>required: **true**<br><p>The name of the bucket for which to set access to another Lightsail resource.</p><br>
    ///   - [`access(ResourceBucketAccess)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::access) / [`set_access(Option<ResourceBucketAccess>)`](crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::set_access):<br>required: **true**<br><p>The access setting.</p>  <p>The following access settings are available:</p>  <ul>   <li> <p> <code>allow</code> - Allows access to the bucket and its objects.</p> </li>   <li> <p> <code>deny</code> - Denies access to the bucket and its objects. Use this setting to remove access for a resource previously set to <code>allow</code>.</p> </li>  </ul><br>
    /// - On success, responds with [`SetResourceAccessForBucketOutput`](crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketOutput) with field(s):
    ///   - [`operations(Option<Vec::<Operation>>)`](crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<SetResourceAccessForBucketError>`](crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError)
    pub fn set_resource_access_for_bucket(
        &self,
    ) -> crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder {
        crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketFluentBuilder::new(self.handle.clone())
    }
}
