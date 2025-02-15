// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_entity_aggregates_for_organization::_describe_entity_aggregates_for_organization_output::DescribeEntityAggregatesForOrganizationOutputBuilder;

pub use crate::operation::describe_entity_aggregates_for_organization::_describe_entity_aggregates_for_organization_input::DescribeEntityAggregatesForOrganizationInputBuilder;

impl DescribeEntityAggregatesForOrganizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_entity_aggregates_for_organization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeEntityAggregatesForOrganization`.
///
/// <p>Returns a list of entity aggregates for your Organizations that are affected by each of the specified events.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeEntityAggregatesForOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_entity_aggregates_for_organization::builders::DescribeEntityAggregatesForOrganizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationOutput,
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationError,
    > for DescribeEntityAggregatesForOrganizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationOutput,
            crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeEntityAggregatesForOrganizationFluentBuilder {
    /// Creates a new `DescribeEntityAggregatesForOrganization`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeEntityAggregatesForOrganization as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_entity_aggregates_for_organization::builders::DescribeEntityAggregatesForOrganizationInputBuilder {
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
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganization::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganization::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationOutput,
        crate::operation::describe_entity_aggregates_for_organization::DescribeEntityAggregatesForOrganizationError,
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
    /// Appends an item to `eventArns`.
    ///
    /// To override the contents of this collection use [`set_event_arns`](Self::set_event_arns).
    ///
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    pub fn event_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_arns(input.into());
        self
    }
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    pub fn set_event_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_event_arns(input);
        self
    }
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    pub fn get_event_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_event_arns()
    }
    /// Appends an item to `awsAccountIds`.
    ///
    /// To override the contents of this collection use [`set_aws_account_ids`](Self::set_aws_account_ids).
    ///
    /// <p>A list of 12-digit Amazon Web Services account numbers that contains the affected entities.</p>
    pub fn aws_account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_ids(input.into());
        self
    }
    /// <p>A list of 12-digit Amazon Web Services account numbers that contains the affected entities.</p>
    pub fn set_aws_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_aws_account_ids(input);
        self
    }
    /// <p>A list of 12-digit Amazon Web Services account numbers that contains the affected entities.</p>
    pub fn get_aws_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_aws_account_ids()
    }
}
