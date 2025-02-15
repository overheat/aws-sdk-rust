// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Thumbnail Configuration
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThumbnailConfiguration {
    /// Whether Thumbnail is enabled.
    pub state: ::std::option::Option<crate::types::ThumbnailState>,
}
impl ThumbnailConfiguration {
    /// Whether Thumbnail is enabled.
    pub fn state(&self) -> ::std::option::Option<&crate::types::ThumbnailState> {
        self.state.as_ref()
    }
}
impl ThumbnailConfiguration {
    /// Creates a new builder-style object to manufacture [`ThumbnailConfiguration`](crate::types::ThumbnailConfiguration).
    pub fn builder() -> crate::types::builders::ThumbnailConfigurationBuilder {
        crate::types::builders::ThumbnailConfigurationBuilder::default()
    }
}

/// A builder for [`ThumbnailConfiguration`](crate::types::ThumbnailConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ThumbnailConfigurationBuilder {
    pub(crate) state: ::std::option::Option<crate::types::ThumbnailState>,
}
impl ThumbnailConfigurationBuilder {
    /// Whether Thumbnail is enabled.
    /// This field is required.
    pub fn state(mut self, input: crate::types::ThumbnailState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// Whether Thumbnail is enabled.
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ThumbnailState>) -> Self {
        self.state = input;
        self
    }
    /// Whether Thumbnail is enabled.
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ThumbnailState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`ThumbnailConfiguration`](crate::types::ThumbnailConfiguration).
    pub fn build(self) -> crate::types::ThumbnailConfiguration {
        crate::types::ThumbnailConfiguration { state: self.state }
    }
}
