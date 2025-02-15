// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAgent`](crate::operation::describe_agent::builders::DescribeAgentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_arn(impl Into<String>)`](crate::operation::describe_agent::builders::DescribeAgentFluentBuilder::agent_arn) / [`set_agent_arn(Option<String>)`](crate::operation::describe_agent::builders::DescribeAgentFluentBuilder::set_agent_arn):<br>required: **true**<br><p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that you want information about.</p><br>
    /// - On success, responds with [`DescribeAgentOutput`](crate::operation::describe_agent::DescribeAgentOutput) with field(s):
    ///   - [`agent_arn(Option<String>)`](crate::operation::describe_agent::DescribeAgentOutput::agent_arn): <p>The ARN of the agent.</p>
    ///   - [`name(Option<String>)`](crate::operation::describe_agent::DescribeAgentOutput::name): <p>The name of the agent.</p>
    ///   - [`status(Option<AgentStatus>)`](crate::operation::describe_agent::DescribeAgentOutput::status): <p>The status of the agent.</p>  <ul>   <li> <p>If the status is <code>ONLINE</code>, the agent is configured properly and ready to use.</p> </li>   <li> <p>If the status is <code>OFFLINE</code>, the agent has been out of contact with DataSync for five minutes or longer. This can happen for a few reasons. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/troubleshooting-datasync-agents.html#troubleshoot-agent-offline">What do I do if my agent is offline?</a> </p> </li>  </ul>
    ///   - [`last_connection_time(Option<DateTime>)`](crate::operation::describe_agent::DescribeAgentOutput::last_connection_time): <p>The last time that the agent was communicating with the DataSync service.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_agent::DescribeAgentOutput::creation_time): <p>The time that the agent was <a href="https://docs.aws.amazon.com/datasync/latest/userguide/activate-agent.html">activated</a>.</p>
    ///   - [`endpoint_type(Option<EndpointType>)`](crate::operation::describe_agent::DescribeAgentOutput::endpoint_type): <p>The type of <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choose-service-endpoint.html">service endpoint</a> that your agent is connected to.</p>
    ///   - [`private_link_config(Option<PrivateLinkConfig>)`](crate::operation::describe_agent::DescribeAgentOutput::private_link_config): <p>The network configuration that the agent uses when connecting to a <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choose-service-endpoint.html#choose-service-endpoint-vpc">VPC service endpoint</a>.</p>
    ///   - [`platform(Option<Platform>)`](crate::operation::describe_agent::DescribeAgentOutput::platform): <p>The platform-related details about the agent, such as the version number.</p>
    /// - On failure, responds with [`SdkError<DescribeAgentError>`](crate::operation::describe_agent::DescribeAgentError)
    pub fn describe_agent(&self) -> crate::operation::describe_agent::builders::DescribeAgentFluentBuilder {
        crate::operation::describe_agent::builders::DescribeAgentFluentBuilder::new(self.handle.clone())
    }
}
