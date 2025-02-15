// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopDiscoveryJobOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for StopDiscoveryJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopDiscoveryJobOutput {
    /// Creates a new builder-style object to manufacture [`StopDiscoveryJobOutput`](crate::operation::stop_discovery_job::StopDiscoveryJobOutput).
    pub fn builder() -> crate::operation::stop_discovery_job::builders::StopDiscoveryJobOutputBuilder {
        crate::operation::stop_discovery_job::builders::StopDiscoveryJobOutputBuilder::default()
    }
}

/// A builder for [`StopDiscoveryJobOutput`](crate::operation::stop_discovery_job::StopDiscoveryJobOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StopDiscoveryJobOutputBuilder {
    _request_id: Option<String>,
}
impl StopDiscoveryJobOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StopDiscoveryJobOutput`](crate::operation::stop_discovery_job::StopDiscoveryJobOutput).
    pub fn build(self) -> crate::operation::stop_discovery_job::StopDiscoveryJobOutput {
        crate::operation::stop_discovery_job::StopDiscoveryJobOutput {
            _request_id: self._request_id,
        }
    }
}
