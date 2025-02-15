// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartMulticastGroupSessionOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for StartMulticastGroupSessionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartMulticastGroupSessionOutput {
    /// Creates a new builder-style object to manufacture [`StartMulticastGroupSessionOutput`](crate::operation::start_multicast_group_session::StartMulticastGroupSessionOutput).
    pub fn builder() -> crate::operation::start_multicast_group_session::builders::StartMulticastGroupSessionOutputBuilder {
        crate::operation::start_multicast_group_session::builders::StartMulticastGroupSessionOutputBuilder::default()
    }
}

/// A builder for [`StartMulticastGroupSessionOutput`](crate::operation::start_multicast_group_session::StartMulticastGroupSessionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StartMulticastGroupSessionOutputBuilder {
    _request_id: Option<String>,
}
impl StartMulticastGroupSessionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartMulticastGroupSessionOutput`](crate::operation::start_multicast_group_session::StartMulticastGroupSessionOutput).
    pub fn build(self) -> crate::operation::start_multicast_group_session::StartMulticastGroupSessionOutput {
        crate::operation::start_multicast_group_session::StartMulticastGroupSessionOutput {
            _request_id: self._request_id,
        }
    }
}
