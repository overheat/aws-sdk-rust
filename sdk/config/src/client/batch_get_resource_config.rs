// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetResourceConfig`](crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_keys(ResourceKey)`](crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigFluentBuilder::resource_keys) / [`set_resource_keys(Option<Vec::<ResourceKey>>)`](crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigFluentBuilder::set_resource_keys):<br>required: **true**<br><p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID.</p><br>
    /// - On success, responds with [`BatchGetResourceConfigOutput`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput) with field(s):
    ///   - [`base_configuration_items(Option<Vec::<BaseConfigurationItem>>)`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput::base_configuration_items): <p>A list that contains the current configuration of one or more resources.</p>
    ///   - [`unprocessed_resource_keys(Option<Vec::<ResourceKey>>)`](crate::operation::batch_get_resource_config::BatchGetResourceConfigOutput::unprocessed_resource_keys): <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    /// - On failure, responds with [`SdkError<BatchGetResourceConfigError>`](crate::operation::batch_get_resource_config::BatchGetResourceConfigError)
    pub fn batch_get_resource_config(&self) -> crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigFluentBuilder {
        crate::operation::batch_get_resource_config::builders::BatchGetResourceConfigFluentBuilder::new(self.handle.clone())
    }
}
