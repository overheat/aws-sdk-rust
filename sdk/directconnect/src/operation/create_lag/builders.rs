// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_lag::_create_lag_output::CreateLagOutputBuilder;

pub use crate::operation::create_lag::_create_lag_input::CreateLagInputBuilder;

impl CreateLagInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_lag::CreateLagOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_lag::CreateLagError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_lag();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLag`.
///
/// <p>Creates a link aggregation group (LAG) with the specified number of bundled physical dedicated connections between the customer network and a specific Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple interfaces, enabling you to treat them as a single interface.</p>
/// <p>All connections in a LAG must use the same bandwidth (either 1Gbps or 10Gbps) and must terminate at the same Direct Connect endpoint.</p>
/// <p>You can have up to 10 dedicated connections per LAG. Regardless of this limit, if you request more connections for the LAG than Direct Connect can allocate on a single endpoint, no LAG is created.</p>
/// <p>You can specify an existing physical dedicated connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical dedicated connection, and re-establishes them as a member of the LAG. The LAG will be created on the same Direct Connect endpoint to which the dedicated connection terminates. Any virtual interfaces associated with the dedicated connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p>
/// <p>If the Amazon Web Services account used to create a LAG is a registered Direct Connect Partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLagFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_lag::builders::CreateLagInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_lag::CreateLagOutput, crate::operation::create_lag::CreateLagError>
    for CreateLagFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_lag::CreateLagOutput, crate::operation::create_lag::CreateLagError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLagFluentBuilder {
    /// Creates a new `CreateLag`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLag as a reference.
    pub fn as_input(&self) -> &crate::operation::create_lag::builders::CreateLagInputBuilder {
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
        crate::operation::create_lag::CreateLagOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_lag::CreateLagError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_lag::CreateLag::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_lag::CreateLag::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_lag::CreateLagOutput,
        crate::operation::create_lag::CreateLagError,
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
    /// <p>The number of physical dedicated connections initially provisioned and bundled by the LAG. You can have a maximum of four connections when the port speed is 1G or 10G, or two when the port speed is 100G. </p>
    pub fn number_of_connections(mut self, input: i32) -> Self {
        self.inner = self.inner.number_of_connections(input);
        self
    }
    /// <p>The number of physical dedicated connections initially provisioned and bundled by the LAG. You can have a maximum of four connections when the port speed is 1G or 10G, or two when the port speed is 100G. </p>
    pub fn set_number_of_connections(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_number_of_connections(input);
        self
    }
    /// <p>The number of physical dedicated connections initially provisioned and bundled by the LAG. You can have a maximum of four connections when the port speed is 1G or 10G, or two when the port speed is 100G. </p>
    pub fn get_number_of_connections(&self) -> &::std::option::Option<i32> {
        self.inner.get_number_of_connections()
    }
    /// <p>The location for the LAG.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The location for the LAG.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The location for the LAG.</p>
    pub fn get_location(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_location()
    }
    /// <p>The bandwidth of the individual physical dedicated connections bundled by the LAG. The possible values are 1Gbps and 10Gbps. </p>
    pub fn connections_bandwidth(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connections_bandwidth(input.into());
        self
    }
    /// <p>The bandwidth of the individual physical dedicated connections bundled by the LAG. The possible values are 1Gbps and 10Gbps. </p>
    pub fn set_connections_bandwidth(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connections_bandwidth(input);
        self
    }
    /// <p>The bandwidth of the individual physical dedicated connections bundled by the LAG. The possible values are 1Gbps and 10Gbps. </p>
    pub fn get_connections_bandwidth(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connections_bandwidth()
    }
    /// <p>The name of the LAG.</p>
    pub fn lag_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lag_name(input.into());
        self
    }
    /// <p>The name of the LAG.</p>
    pub fn set_lag_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lag_name(input);
        self
    }
    /// <p>The name of the LAG.</p>
    pub fn get_lag_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lag_name()
    }
    /// <p>The ID of an existing dedicated connection to migrate to the LAG.</p>
    pub fn connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_id(input.into());
        self
    }
    /// <p>The ID of an existing dedicated connection to migrate to the LAG.</p>
    pub fn set_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_id(input);
        self
    }
    /// <p>The ID of an existing dedicated connection to migrate to the LAG.</p>
    pub fn get_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_id()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to associate with the LAG.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to associate with the LAG.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to associate with the LAG.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// Appends an item to `childConnectionTags`.
    ///
    /// To override the contents of this collection use [`set_child_connection_tags`](Self::set_child_connection_tags).
    ///
    /// <p>The tags to associate with the automtically created LAGs.</p>
    pub fn child_connection_tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.child_connection_tags(input);
        self
    }
    /// <p>The tags to associate with the automtically created LAGs.</p>
    pub fn set_child_connection_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_child_connection_tags(input);
        self
    }
    /// <p>The tags to associate with the automtically created LAGs.</p>
    pub fn get_child_connection_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_child_connection_tags()
    }
    /// <p>The name of the service provider associated with the LAG.</p>
    pub fn provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provider_name(input.into());
        self
    }
    /// <p>The name of the service provider associated with the LAG.</p>
    pub fn set_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provider_name(input);
        self
    }
    /// <p>The name of the service provider associated with the LAG.</p>
    pub fn get_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provider_name()
    }
    /// <p>Indicates whether the connection will support MAC Security (MACsec).</p> <note>
    /// <p>All connections in the LAG must be capable of supporting MAC Security (MACsec). For information about MAC Security (MACsec) prerequisties, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites">MACsec prerequisties</a> in the <i>Direct Connect User Guide</i>.</p>
    /// </note>
    pub fn request_mac_sec(mut self, input: bool) -> Self {
        self.inner = self.inner.request_mac_sec(input);
        self
    }
    /// <p>Indicates whether the connection will support MAC Security (MACsec).</p> <note>
    /// <p>All connections in the LAG must be capable of supporting MAC Security (MACsec). For information about MAC Security (MACsec) prerequisties, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites">MACsec prerequisties</a> in the <i>Direct Connect User Guide</i>.</p>
    /// </note>
    pub fn set_request_mac_sec(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_request_mac_sec(input);
        self
    }
    /// <p>Indicates whether the connection will support MAC Security (MACsec).</p> <note>
    /// <p>All connections in the LAG must be capable of supporting MAC Security (MACsec). For information about MAC Security (MACsec) prerequisties, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites">MACsec prerequisties</a> in the <i>Direct Connect User Guide</i>.</p>
    /// </note>
    pub fn get_request_mac_sec(&self) -> &::std::option::Option<bool> {
        self.inner.get_request_mac_sec()
    }
}
