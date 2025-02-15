// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateFeatureGroupInput {
    /// <p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p>
    pub feature_group_name: ::std::option::Option<::std::string::String>,
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub feature_additions: ::std::option::Option<::std::vec::Vec<crate::types::FeatureDefinition>>,
    /// <p>Updates the feature group online store configuration.</p>
    pub online_store_config: ::std::option::Option<crate::types::OnlineStoreConfigUpdate>,
}
impl UpdateFeatureGroupInput {
    /// <p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p>
    pub fn feature_group_name(&self) -> ::std::option::Option<&str> {
        self.feature_group_name.as_deref()
    }
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.feature_additions.is_none()`.
    pub fn feature_additions(&self) -> &[crate::types::FeatureDefinition] {
        self.feature_additions.as_deref().unwrap_or_default()
    }
    /// <p>Updates the feature group online store configuration.</p>
    pub fn online_store_config(&self) -> ::std::option::Option<&crate::types::OnlineStoreConfigUpdate> {
        self.online_store_config.as_ref()
    }
}
impl UpdateFeatureGroupInput {
    /// Creates a new builder-style object to manufacture [`UpdateFeatureGroupInput`](crate::operation::update_feature_group::UpdateFeatureGroupInput).
    pub fn builder() -> crate::operation::update_feature_group::builders::UpdateFeatureGroupInputBuilder {
        crate::operation::update_feature_group::builders::UpdateFeatureGroupInputBuilder::default()
    }
}

/// A builder for [`UpdateFeatureGroupInput`](crate::operation::update_feature_group::UpdateFeatureGroupInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateFeatureGroupInputBuilder {
    pub(crate) feature_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) feature_additions: ::std::option::Option<::std::vec::Vec<crate::types::FeatureDefinition>>,
    pub(crate) online_store_config: ::std::option::Option<crate::types::OnlineStoreConfigUpdate>,
}
impl UpdateFeatureGroupInputBuilder {
    /// <p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p>
    /// This field is required.
    pub fn feature_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.feature_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p>
    pub fn set_feature_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.feature_group_name = input;
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the feature group that you're updating.</p>
    pub fn get_feature_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.feature_group_name
    }
    /// Appends an item to `feature_additions`.
    ///
    /// To override the contents of this collection use [`set_feature_additions`](Self::set_feature_additions).
    ///
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub fn feature_additions(mut self, input: crate::types::FeatureDefinition) -> Self {
        let mut v = self.feature_additions.unwrap_or_default();
        v.push(input);
        self.feature_additions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub fn set_feature_additions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FeatureDefinition>>) -> Self {
        self.feature_additions = input;
        self
    }
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub fn get_feature_additions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FeatureDefinition>> {
        &self.feature_additions
    }
    /// <p>Updates the feature group online store configuration.</p>
    pub fn online_store_config(mut self, input: crate::types::OnlineStoreConfigUpdate) -> Self {
        self.online_store_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Updates the feature group online store configuration.</p>
    pub fn set_online_store_config(mut self, input: ::std::option::Option<crate::types::OnlineStoreConfigUpdate>) -> Self {
        self.online_store_config = input;
        self
    }
    /// <p>Updates the feature group online store configuration.</p>
    pub fn get_online_store_config(&self) -> &::std::option::Option<crate::types::OnlineStoreConfigUpdate> {
        &self.online_store_config
    }
    /// Consumes the builder and constructs a [`UpdateFeatureGroupInput`](crate::operation::update_feature_group::UpdateFeatureGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_feature_group::UpdateFeatureGroupInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::update_feature_group::UpdateFeatureGroupInput {
            feature_group_name: self.feature_group_name,
            feature_additions: self.feature_additions,
            online_store_config: self.online_store_config,
        })
    }
}
