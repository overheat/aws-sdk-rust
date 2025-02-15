// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The connector metadata specific to ServiceNow. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceNowMetadata {}
impl ServiceNowMetadata {
    /// Creates a new builder-style object to manufacture [`ServiceNowMetadata`](crate::types::ServiceNowMetadata).
    pub fn builder() -> crate::types::builders::ServiceNowMetadataBuilder {
        crate::types::builders::ServiceNowMetadataBuilder::default()
    }
}

/// A builder for [`ServiceNowMetadata`](crate::types::ServiceNowMetadata).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ServiceNowMetadataBuilder {}
impl ServiceNowMetadataBuilder {
    /// Consumes the builder and constructs a [`ServiceNowMetadata`](crate::types::ServiceNowMetadata).
    pub fn build(self) -> crate::types::ServiceNowMetadata {
        crate::types::ServiceNowMetadata {}
    }
}
