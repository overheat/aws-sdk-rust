// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeIdentityProviderConfig`](crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The cluster name that the identity provider configuration is associated to.</p><br>
    ///   - [`identity_provider_config(IdentityProviderConfig)`](crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder::identity_provider_config) / [`set_identity_provider_config(Option<IdentityProviderConfig>)`](crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder::set_identity_provider_config):<br>required: **true**<br><p>An object representing an identity provider configuration.</p><br>
    /// - On success, responds with [`DescribeIdentityProviderConfigOutput`](crate::operation::describe_identity_provider_config::DescribeIdentityProviderConfigOutput) with field(s):
    ///   - [`identity_provider_config(Option<IdentityProviderConfigResponse>)`](crate::operation::describe_identity_provider_config::DescribeIdentityProviderConfigOutput::identity_provider_config): <p>The object that represents an OpenID Connect (OIDC) identity provider configuration.</p>
    /// - On failure, responds with [`SdkError<DescribeIdentityProviderConfigError>`](crate::operation::describe_identity_provider_config::DescribeIdentityProviderConfigError)
    pub fn describe_identity_provider_config(
        &self,
    ) -> crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder {
        crate::operation::describe_identity_provider_config::builders::DescribeIdentityProviderConfigFluentBuilder::new(self.handle.clone())
    }
}
