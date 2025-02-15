// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteStreamingDistributionOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DeleteStreamingDistributionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteStreamingDistributionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteStreamingDistributionOutput`](crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput).
    pub fn builder() -> crate::operation::delete_streaming_distribution::builders::DeleteStreamingDistributionOutputBuilder {
        crate::operation::delete_streaming_distribution::builders::DeleteStreamingDistributionOutputBuilder::default()
    }
}

/// A builder for [`DeleteStreamingDistributionOutput`](crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteStreamingDistributionOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteStreamingDistributionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteStreamingDistributionOutput`](crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput).
    pub fn build(self) -> crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput {
        crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput {
            _request_id: self._request_id,
        }
    }
}
