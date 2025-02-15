// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFeatureGroup`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`feature_group_name(impl Into<String>)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::feature_group_name) / [`set_feature_group_name(Option<String>)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::set_feature_group_name):<br>required: **true**<br><p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p><br>
    ///   - [`feature_additions(FeatureDefinition)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::feature_additions) / [`set_feature_additions(Option<Vec::<FeatureDefinition>>)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::set_feature_additions):<br>required: **false**<br><p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p><br>
    ///   - [`online_store_config(OnlineStoreConfigUpdate)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::online_store_config) / [`set_online_store_config(Option<OnlineStoreConfigUpdate>)`](crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::set_online_store_config):<br>required: **false**<br><p>Updates the feature group online store configuration.</p><br>
    /// - On success, responds with [`UpdateFeatureGroupOutput`](crate::operation::update_feature_group::UpdateFeatureGroupOutput) with field(s):
    ///   - [`feature_group_arn(Option<String>)`](crate::operation::update_feature_group::UpdateFeatureGroupOutput::feature_group_arn): <p>The Amazon Resource Number (ARN) of the feature group that you're updating.</p>
    /// - On failure, responds with [`SdkError<UpdateFeatureGroupError>`](crate::operation::update_feature_group::UpdateFeatureGroupError)
    pub fn update_feature_group(&self) -> crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder {
        crate::operation::update_feature_group::builders::UpdateFeatureGroupFluentBuilder::new(self.handle.clone())
    }
}
