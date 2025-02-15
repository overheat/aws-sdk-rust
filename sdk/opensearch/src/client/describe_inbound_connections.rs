// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInboundConnections`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::set_filters):<br>required: **false**<br><p> A list of filters used to match properties for inbound cross-cluster connections.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::set_next_token):<br>required: **false**<br><p>If your initial <code>DescribeInboundConnections</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>DescribeInboundConnections</code> operations, which returns results in the next page.</p><br>
    /// - On success, responds with [`DescribeInboundConnectionsOutput`](crate::operation::describe_inbound_connections::DescribeInboundConnectionsOutput) with field(s):
    ///   - [`connections(Option<Vec::<InboundConnection>>)`](crate::operation::describe_inbound_connections::DescribeInboundConnectionsOutput::connections): <p>List of inbound connections.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_inbound_connections::DescribeInboundConnectionsOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<DescribeInboundConnectionsError>`](crate::operation::describe_inbound_connections::DescribeInboundConnectionsError)
    pub fn describe_inbound_connections(&self) -> crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder {
        crate::operation::describe_inbound_connections::builders::DescribeInboundConnectionsFluentBuilder::new(self.handle.clone())
    }
}
