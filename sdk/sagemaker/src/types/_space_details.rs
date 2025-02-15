// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The space's details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SpaceDetails {
    /// <p>The ID of the associated Domain.</p>
    pub domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the space.</p>
    pub space_name: ::std::option::Option<::std::string::String>,
    /// <p>The status.</p>
    pub status: ::std::option::Option<crate::types::SpaceStatus>,
    /// <p>The creation time.</p>
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last modified time.</p>
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SpaceDetails {
    /// <p>The ID of the associated Domain.</p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
    /// <p>The name of the space.</p>
    pub fn space_name(&self) -> ::std::option::Option<&str> {
        self.space_name.as_deref()
    }
    /// <p>The status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::SpaceStatus> {
        self.status.as_ref()
    }
    /// <p>The creation time.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The last modified time.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
}
impl SpaceDetails {
    /// Creates a new builder-style object to manufacture [`SpaceDetails`](crate::types::SpaceDetails).
    pub fn builder() -> crate::types::builders::SpaceDetailsBuilder {
        crate::types::builders::SpaceDetailsBuilder::default()
    }
}

/// A builder for [`SpaceDetails`](crate::types::SpaceDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SpaceDetailsBuilder {
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) space_name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::SpaceStatus>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl SpaceDetailsBuilder {
    /// <p>The ID of the associated Domain.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p>The ID of the associated Domain.</p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_id
    }
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.space_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.space_name = input;
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.space_name
    }
    /// <p>The status.</p>
    pub fn status(mut self, input: crate::types::SpaceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::SpaceStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::SpaceStatus> {
        &self.status
    }
    /// <p>The creation time.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation time.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The creation time.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>The last modified time.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last modified time.</p>
    pub fn set_last_modified_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified_time = input;
        self
    }
    /// <p>The last modified time.</p>
    pub fn get_last_modified_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified_time
    }
    /// Consumes the builder and constructs a [`SpaceDetails`](crate::types::SpaceDetails).
    pub fn build(self) -> crate::types::SpaceDetails {
        crate::types::SpaceDetails {
            domain_id: self.domain_id,
            space_name: self.space_name,
            status: self.status,
            creation_time: self.creation_time,
            last_modified_time: self.last_modified_time,
        }
    }
}
