// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVerifiedAccessInstance`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_description):<br>required: **false**<br><p>A description for the Verified Access instance.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to assign to the Verified Access instance.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`fips_enabled(bool)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::fips_enabled) / [`set_fips_enabled(Option<bool>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_fips_enabled):<br>required: **false**<br><p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p><br>
    /// - On success, responds with [`CreateVerifiedAccessInstanceOutput`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceOutput) with field(s):
    ///   - [`verified_access_instance(Option<VerifiedAccessInstance>)`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceOutput::verified_access_instance): <p>The ID of the Verified Access instance.</p>
    /// - On failure, responds with [`SdkError<CreateVerifiedAccessInstanceError>`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceError)
    pub fn create_verified_access_instance(
        &self,
    ) -> crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder {
        crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::new(self.handle.clone())
    }
}
