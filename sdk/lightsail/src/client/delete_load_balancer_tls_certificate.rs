// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLoadBalancerTlsCertificate`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::set_load_balancer_name):<br>required: **true**<br><p>The load balancer name.</p><br>
    ///   - [`certificate_name(impl Into<String>)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::certificate_name) / [`set_certificate_name(Option<String>)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::set_certificate_name):<br>required: **true**<br><p>The SSL/TLS certificate name.</p><br>
    ///   - [`force(bool)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::force) / [`set_force(Option<bool>)`](crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::set_force):<br>required: **false**<br><p>When <code>true</code>, forces the deletion of an SSL/TLS certificate.</p>  <p>There can be two certificates associated with a Lightsail load balancer: the primary and the backup. The <code>force</code> parameter is required when the primary SSL/TLS certificate is in use by an instance attached to the load balancer.</p><br>
    /// - On success, responds with [`DeleteLoadBalancerTlsCertificateOutput`](crate::operation::delete_load_balancer_tls_certificate::DeleteLoadBalancerTlsCertificateOutput) with field(s):
    ///   - [`operations(Option<Vec::<Operation>>)`](crate::operation::delete_load_balancer_tls_certificate::DeleteLoadBalancerTlsCertificateOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<DeleteLoadBalancerTlsCertificateError>`](crate::operation::delete_load_balancer_tls_certificate::DeleteLoadBalancerTlsCertificateError)
    pub fn delete_load_balancer_tls_certificate(
        &self,
    ) -> crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder {
        crate::operation::delete_load_balancer_tls_certificate::builders::DeleteLoadBalancerTlsCertificateFluentBuilder::new(self.handle.clone())
    }
}
