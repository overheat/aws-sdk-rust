// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_certificate_authority_audit_report::_describe_certificate_authority_audit_report_output::DescribeCertificateAuthorityAuditReportOutputBuilder;

pub use crate::operation::describe_certificate_authority_audit_report::_describe_certificate_authority_audit_report_input::DescribeCertificateAuthorityAuditReportInputBuilder;

impl DescribeCertificateAuthorityAuditReportInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_certificate_authority_audit_report();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeCertificateAuthorityAuditReport`.
///
/// <p>Lists information about a specific audit report created by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthorityAuditReport.html">CreateCertificateAuthorityAuditReport</a> action. Audit information is created every time the certificate authority (CA) private key is used. The private key is used when you call the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_IssueCertificate.html">IssueCertificate</a> action or the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_RevokeCertificate.html">RevokeCertificate</a> action. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeCertificateAuthorityAuditReportFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_certificate_authority_audit_report::builders::DescribeCertificateAuthorityAuditReportInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportOutput,
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportError,
    > for DescribeCertificateAuthorityAuditReportFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportOutput,
            crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeCertificateAuthorityAuditReportFluentBuilder {
    /// Creates a new `DescribeCertificateAuthorityAuditReport`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeCertificateAuthorityAuditReport as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_certificate_authority_audit_report::builders::DescribeCertificateAuthorityAuditReportInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReport::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReport::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportOutput,
        crate::operation::describe_certificate_authority_audit_report::DescribeCertificateAuthorityAuditReportError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the private CA. This must be of the form:</p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    pub fn certificate_authority_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_authority_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the private CA. This must be of the form:</p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    pub fn set_certificate_authority_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_authority_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the private CA. This must be of the form:</p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    pub fn get_certificate_authority_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_authority_arn()
    }
    /// <p>The report ID returned by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthorityAuditReport.html">CreateCertificateAuthorityAuditReport</a> action.</p>
    pub fn audit_report_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.audit_report_id(input.into());
        self
    }
    /// <p>The report ID returned by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthorityAuditReport.html">CreateCertificateAuthorityAuditReport</a> action.</p>
    pub fn set_audit_report_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_audit_report_id(input);
        self
    }
    /// <p>The report ID returned by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthorityAuditReport.html">CreateCertificateAuthorityAuditReport</a> action.</p>
    pub fn get_audit_report_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_audit_report_id()
    }
}
