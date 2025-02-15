// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The details of a share. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ShareDetails {
    /// <p> The ID for a share offer for an analytics store . </p>
    pub share_id: ::std::option::Option<::std::string::String>,
    /// <p> The resource Arn of the analytics store being shared. </p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p> The principal subscriber is the account the analytics store data is being shared with. </p>
    pub principal_subscriber: ::std::option::Option<::std::string::String>,
    /// <p> The account ID for the data owner. The owner creates the share offer. </p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p> The status of a share. </p>
    pub status: ::std::option::Option<crate::types::ShareStatus>,
    /// <p> The status message for a share. It provides more details on the status of the share. </p>
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p> The name of the share. </p>
    pub share_name: ::std::option::Option<::std::string::String>,
    /// <p> The timestamp for when the share was created. </p>
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The timestamp of the share update. </p>
    pub update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ShareDetails {
    /// <p> The ID for a share offer for an analytics store . </p>
    pub fn share_id(&self) -> ::std::option::Option<&str> {
        self.share_id.as_deref()
    }
    /// <p> The resource Arn of the analytics store being shared. </p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p> The principal subscriber is the account the analytics store data is being shared with. </p>
    pub fn principal_subscriber(&self) -> ::std::option::Option<&str> {
        self.principal_subscriber.as_deref()
    }
    /// <p> The account ID for the data owner. The owner creates the share offer. </p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p> The status of a share. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ShareStatus> {
        self.status.as_ref()
    }
    /// <p> The status message for a share. It provides more details on the status of the share. </p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p> The name of the share. </p>
    pub fn share_name(&self) -> ::std::option::Option<&str> {
        self.share_name.as_deref()
    }
    /// <p> The timestamp for when the share was created. </p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p> The timestamp of the share update. </p>
    pub fn update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
}
impl ShareDetails {
    /// Creates a new builder-style object to manufacture [`ShareDetails`](crate::types::ShareDetails).
    pub fn builder() -> crate::types::builders::ShareDetailsBuilder {
        crate::types::builders::ShareDetailsBuilder::default()
    }
}

/// A builder for [`ShareDetails`](crate::types::ShareDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ShareDetailsBuilder {
    pub(crate) share_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) principal_subscriber: ::std::option::Option<::std::string::String>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ShareStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) share_name: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ShareDetailsBuilder {
    /// <p> The ID for a share offer for an analytics store . </p>
    pub fn share_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.share_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID for a share offer for an analytics store . </p>
    pub fn set_share_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.share_id = input;
        self
    }
    /// <p> The ID for a share offer for an analytics store . </p>
    pub fn get_share_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.share_id
    }
    /// <p> The resource Arn of the analytics store being shared. </p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The resource Arn of the analytics store being shared. </p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p> The resource Arn of the analytics store being shared. </p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p> The principal subscriber is the account the analytics store data is being shared with. </p>
    pub fn principal_subscriber(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal_subscriber = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The principal subscriber is the account the analytics store data is being shared with. </p>
    pub fn set_principal_subscriber(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal_subscriber = input;
        self
    }
    /// <p> The principal subscriber is the account the analytics store data is being shared with. </p>
    pub fn get_principal_subscriber(&self) -> &::std::option::Option<::std::string::String> {
        &self.principal_subscriber
    }
    /// <p> The account ID for the data owner. The owner creates the share offer. </p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The account ID for the data owner. The owner creates the share offer. </p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p> The account ID for the data owner. The owner creates the share offer. </p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p> The status of a share. </p>
    pub fn status(mut self, input: crate::types::ShareStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The status of a share. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ShareStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p> The status of a share. </p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ShareStatus> {
        &self.status
    }
    /// <p> The status message for a share. It provides more details on the status of the share. </p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The status message for a share. It provides more details on the status of the share. </p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p> The status message for a share. It provides more details on the status of the share. </p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// <p> The name of the share. </p>
    pub fn share_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.share_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the share. </p>
    pub fn set_share_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.share_name = input;
        self
    }
    /// <p> The name of the share. </p>
    pub fn get_share_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.share_name
    }
    /// <p> The timestamp for when the share was created. </p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The timestamp for when the share was created. </p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p> The timestamp for when the share was created. </p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p> The timestamp of the share update. </p>
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The timestamp of the share update. </p>
    pub fn set_update_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.update_time = input;
        self
    }
    /// <p> The timestamp of the share update. </p>
    pub fn get_update_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.update_time
    }
    /// Consumes the builder and constructs a [`ShareDetails`](crate::types::ShareDetails).
    pub fn build(self) -> crate::types::ShareDetails {
        crate::types::ShareDetails {
            share_id: self.share_id,
            resource_arn: self.resource_arn,
            principal_subscriber: self.principal_subscriber,
            owner_id: self.owner_id,
            status: self.status,
            status_message: self.status_message,
            share_name: self.share_name,
            creation_time: self.creation_time,
            update_time: self.update_time,
        }
    }
}
