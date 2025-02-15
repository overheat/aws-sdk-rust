// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the status of a definition.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AlertManagerDefinitionStatus {
    /// Status code of this definition.
    pub status_code: crate::types::AlertManagerDefinitionStatusCode,
    /// The reason for failure if any.
    pub status_reason: ::std::option::Option<::std::string::String>,
}
impl AlertManagerDefinitionStatus {
    /// Status code of this definition.
    pub fn status_code(&self) -> &crate::types::AlertManagerDefinitionStatusCode {
        &self.status_code
    }
    /// The reason for failure if any.
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
}
impl AlertManagerDefinitionStatus {
    /// Creates a new builder-style object to manufacture [`AlertManagerDefinitionStatus`](crate::types::AlertManagerDefinitionStatus).
    pub fn builder() -> crate::types::builders::AlertManagerDefinitionStatusBuilder {
        crate::types::builders::AlertManagerDefinitionStatusBuilder::default()
    }
}

/// A builder for [`AlertManagerDefinitionStatus`](crate::types::AlertManagerDefinitionStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AlertManagerDefinitionStatusBuilder {
    pub(crate) status_code: ::std::option::Option<crate::types::AlertManagerDefinitionStatusCode>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
}
impl AlertManagerDefinitionStatusBuilder {
    /// Status code of this definition.
    /// This field is required.
    pub fn status_code(mut self, input: crate::types::AlertManagerDefinitionStatusCode) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// Status code of this definition.
    pub fn set_status_code(mut self, input: ::std::option::Option<crate::types::AlertManagerDefinitionStatusCode>) -> Self {
        self.status_code = input;
        self
    }
    /// Status code of this definition.
    pub fn get_status_code(&self) -> &::std::option::Option<crate::types::AlertManagerDefinitionStatusCode> {
        &self.status_code
    }
    /// The reason for failure if any.
    pub fn status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// The reason for failure if any.
    pub fn set_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// The reason for failure if any.
    pub fn get_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_reason
    }
    /// Consumes the builder and constructs a [`AlertManagerDefinitionStatus`](crate::types::AlertManagerDefinitionStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`status_code`](crate::types::builders::AlertManagerDefinitionStatusBuilder::status_code)
    pub fn build(self) -> ::std::result::Result<crate::types::AlertManagerDefinitionStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AlertManagerDefinitionStatus {
            status_code: self.status_code.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status_code",
                    "status_code was not specified but it is required when building AlertManagerDefinitionStatus",
                )
            })?,
            status_reason: self.status_reason,
        })
    }
}
