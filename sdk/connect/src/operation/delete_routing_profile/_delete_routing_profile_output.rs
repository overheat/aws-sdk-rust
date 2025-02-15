// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteRoutingProfileOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DeleteRoutingProfileOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteRoutingProfileOutput {
    /// Creates a new builder-style object to manufacture [`DeleteRoutingProfileOutput`](crate::operation::delete_routing_profile::DeleteRoutingProfileOutput).
    pub fn builder() -> crate::operation::delete_routing_profile::builders::DeleteRoutingProfileOutputBuilder {
        crate::operation::delete_routing_profile::builders::DeleteRoutingProfileOutputBuilder::default()
    }
}

/// A builder for [`DeleteRoutingProfileOutput`](crate::operation::delete_routing_profile::DeleteRoutingProfileOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteRoutingProfileOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteRoutingProfileOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteRoutingProfileOutput`](crate::operation::delete_routing_profile::DeleteRoutingProfileOutput).
    pub fn build(self) -> crate::operation::delete_routing_profile::DeleteRoutingProfileOutput {
        crate::operation::delete_routing_profile::DeleteRoutingProfileOutput {
            _request_id: self._request_id,
        }
    }
}
