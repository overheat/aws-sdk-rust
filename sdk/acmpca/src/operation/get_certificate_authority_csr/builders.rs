// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_certificate_authority_csr::_get_certificate_authority_csr_output::GetCertificateAuthorityCsrOutputBuilder;

pub use crate::operation::get_certificate_authority_csr::_get_certificate_authority_csr_input::GetCertificateAuthorityCsrInputBuilder;

impl GetCertificateAuthorityCsrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_certificate_authority_csr();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetCertificateAuthorityCsr`.
///
/// <p>Retrieves the certificate signing request (CSR) for your private certificate authority (CA). The CSR is created when you call the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html">CreateCertificateAuthority</a> action. Sign the CSR with your Amazon Web Services Private CA-hosted or on-premises root or subordinate CA. Then import the signed certificate back into Amazon Web Services Private CA by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ImportCertificateAuthorityCertificate.html">ImportCertificateAuthorityCertificate</a> action. The CSR is returned as a base64 PEM-encoded string. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCertificateAuthorityCsrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_certificate_authority_csr::builders::GetCertificateAuthorityCsrInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrOutput,
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrError,
    > for GetCertificateAuthorityCsrFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrOutput,
            crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetCertificateAuthorityCsrFluentBuilder {
    /// Creates a new `GetCertificateAuthorityCsr`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetCertificateAuthorityCsr as a reference.
    pub fn as_input(&self) -> &crate::operation::get_certificate_authority_csr::builders::GetCertificateAuthorityCsrInputBuilder {
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
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsr::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsr::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrOutput,
        crate::operation::get_certificate_authority_csr::GetCertificateAuthorityCsrError,
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
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html">CreateCertificateAuthority</a> action. This must be of the form: </p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    pub fn certificate_authority_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_authority_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html">CreateCertificateAuthority</a> action. This must be of the form: </p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    pub fn set_certificate_authority_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_authority_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that was returned when you called the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html">CreateCertificateAuthority</a> action. This must be of the form: </p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code> </p>
    pub fn get_certificate_authority_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_authority_arn()
    }
}
