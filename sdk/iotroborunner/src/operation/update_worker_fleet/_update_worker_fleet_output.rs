// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateWorkerFleetOutput {
    /// Full ARN of the worker fleet.
    pub arn: ::std::string::String,
    /// Filters access by the worker fleet's identifier
    pub id: ::std::string::String,
    /// Human friendly name of the resource.
    pub name: ::std::string::String,
    /// Timestamp at which the resource was last updated.
    pub updated_at: ::aws_smithy_types::DateTime,
    /// JSON blob containing additional fixed properties regarding the worker fleet
    pub additional_fixed_properties: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateWorkerFleetOutput {
    /// Full ARN of the worker fleet.
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// Filters access by the worker fleet's identifier
    pub fn id(&self) -> &str {
        use std::ops::Deref;
        self.id.deref()
    }
    /// Human friendly name of the resource.
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// Timestamp at which the resource was last updated.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
    /// JSON blob containing additional fixed properties regarding the worker fleet
    pub fn additional_fixed_properties(&self) -> ::std::option::Option<&str> {
        self.additional_fixed_properties.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateWorkerFleetOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateWorkerFleetOutput {
    /// Creates a new builder-style object to manufacture [`UpdateWorkerFleetOutput`](crate::operation::update_worker_fleet::UpdateWorkerFleetOutput).
    pub fn builder() -> crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder {
        crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder::default()
    }
}

/// A builder for [`UpdateWorkerFleetOutput`](crate::operation::update_worker_fleet::UpdateWorkerFleetOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateWorkerFleetOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) additional_fixed_properties: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateWorkerFleetOutputBuilder {
    /// Full ARN of the worker fleet.
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// Full ARN of the worker fleet.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Full ARN of the worker fleet.
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Filters access by the worker fleet's identifier
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Filters access by the worker fleet's identifier
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Filters access by the worker fleet's identifier
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Human friendly name of the resource.
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// Human friendly name of the resource.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Human friendly name of the resource.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Timestamp at which the resource was last updated.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// JSON blob containing additional fixed properties regarding the worker fleet
    pub fn additional_fixed_properties(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.additional_fixed_properties = ::std::option::Option::Some(input.into());
        self
    }
    /// JSON blob containing additional fixed properties regarding the worker fleet
    pub fn set_additional_fixed_properties(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.additional_fixed_properties = input;
        self
    }
    /// JSON blob containing additional fixed properties regarding the worker fleet
    pub fn get_additional_fixed_properties(&self) -> &::std::option::Option<::std::string::String> {
        &self.additional_fixed_properties
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateWorkerFleetOutput`](crate::operation::update_worker_fleet::UpdateWorkerFleetOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder::arn)
    /// - [`id`](crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder::id)
    /// - [`name`](crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder::name)
    /// - [`updated_at`](crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder::updated_at)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_worker_fleet::UpdateWorkerFleetOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_worker_fleet::UpdateWorkerFleetOutput {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building UpdateWorkerFleetOutput",
                )
            })?,
            id: self.id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "id",
                    "id was not specified but it is required when building UpdateWorkerFleetOutput",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building UpdateWorkerFleetOutput",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building UpdateWorkerFleetOutput",
                )
            })?,
            additional_fixed_properties: self.additional_fixed_properties,
            _request_id: self._request_id,
        })
    }
}
