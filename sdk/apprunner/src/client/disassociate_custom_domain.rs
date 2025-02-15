// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateCustomDomain`](crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_arn(impl Into<String>)`](crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder::service_arn) / [`set_service_arn(Option<String>)`](crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder::set_service_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the App Runner service that you want to disassociate a custom domain name from.</p><br>
    ///   - [`domain_name(impl Into<String>)`](crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder::set_domain_name):<br>required: **true**<br><p>The domain name that you want to disassociate from the App Runner service.</p><br>
    /// - On success, responds with [`DisassociateCustomDomainOutput`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainOutput) with field(s):
    ///   - [`dns_target(String)`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainOutput::dns_target): <p>The App Runner subdomain of the App Runner service. The disassociated custom domain name was mapped to this target name.</p>
    ///   - [`service_arn(String)`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainOutput::service_arn): <p>The Amazon Resource Name (ARN) of the App Runner service that a custom domain name is disassociated from.</p>
    ///   - [`custom_domain(Option<CustomDomain>)`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainOutput::custom_domain): <p>A description of the domain name that's being disassociated.</p>
    ///   - [`vpc_dns_targets(Vec::<VpcDnsTarget>)`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainOutput::vpc_dns_targets): <p>DNS Target records for the custom domains of this Amazon VPC. </p>
    /// - On failure, responds with [`SdkError<DisassociateCustomDomainError>`](crate::operation::disassociate_custom_domain::DisassociateCustomDomainError)
    pub fn disassociate_custom_domain(&self) -> crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder {
        crate::operation::disassociate_custom_domain::builders::DisassociateCustomDomainFluentBuilder::new(self.handle.clone())
    }
}
