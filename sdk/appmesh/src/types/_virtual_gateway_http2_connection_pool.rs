// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a type of connection pool.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualGatewayHttp2ConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    pub max_requests: i32,
}
impl VirtualGatewayHttp2ConnectionPool {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    pub fn max_requests(&self) -> i32 {
        self.max_requests
    }
}
impl VirtualGatewayHttp2ConnectionPool {
    /// Creates a new builder-style object to manufacture [`VirtualGatewayHttp2ConnectionPool`](crate::types::VirtualGatewayHttp2ConnectionPool).
    pub fn builder() -> crate::types::builders::VirtualGatewayHttp2ConnectionPoolBuilder {
        crate::types::builders::VirtualGatewayHttp2ConnectionPoolBuilder::default()
    }
}

/// A builder for [`VirtualGatewayHttp2ConnectionPool`](crate::types::VirtualGatewayHttp2ConnectionPool).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VirtualGatewayHttp2ConnectionPoolBuilder {
    pub(crate) max_requests: ::std::option::Option<i32>,
}
impl VirtualGatewayHttp2ConnectionPoolBuilder {
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    /// This field is required.
    pub fn max_requests(mut self, input: i32) -> Self {
        self.max_requests = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    pub fn set_max_requests(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_requests = input;
        self
    }
    /// <p>Maximum number of inflight requests Envoy can concurrently support across hosts in upstream cluster.</p>
    pub fn get_max_requests(&self) -> &::std::option::Option<i32> {
        &self.max_requests
    }
    /// Consumes the builder and constructs a [`VirtualGatewayHttp2ConnectionPool`](crate::types::VirtualGatewayHttp2ConnectionPool).
    pub fn build(self) -> crate::types::VirtualGatewayHttp2ConnectionPool {
        crate::types::VirtualGatewayHttp2ConnectionPool {
            max_requests: self.max_requests.unwrap_or_default(),
        }
    }
}
