// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAcceleratorTypesOutput {
    /// <p> The available accelerator types. </p>
    pub accelerator_types: ::std::option::Option<::std::vec::Vec<crate::types::AcceleratorType>>,
    _request_id: Option<String>,
}
impl DescribeAcceleratorTypesOutput {
    /// <p> The available accelerator types. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.accelerator_types.is_none()`.
    pub fn accelerator_types(&self) -> &[crate::types::AcceleratorType] {
        self.accelerator_types.as_deref().unwrap_or_default()
    }
}
impl ::aws_http::request_id::RequestId for DescribeAcceleratorTypesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAcceleratorTypesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorTypesOutput`](crate::operation::describe_accelerator_types::DescribeAcceleratorTypesOutput).
    pub fn builder() -> crate::operation::describe_accelerator_types::builders::DescribeAcceleratorTypesOutputBuilder {
        crate::operation::describe_accelerator_types::builders::DescribeAcceleratorTypesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAcceleratorTypesOutput`](crate::operation::describe_accelerator_types::DescribeAcceleratorTypesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeAcceleratorTypesOutputBuilder {
    pub(crate) accelerator_types: ::std::option::Option<::std::vec::Vec<crate::types::AcceleratorType>>,
    _request_id: Option<String>,
}
impl DescribeAcceleratorTypesOutputBuilder {
    /// Appends an item to `accelerator_types`.
    ///
    /// To override the contents of this collection use [`set_accelerator_types`](Self::set_accelerator_types).
    ///
    /// <p> The available accelerator types. </p>
    pub fn accelerator_types(mut self, input: crate::types::AcceleratorType) -> Self {
        let mut v = self.accelerator_types.unwrap_or_default();
        v.push(input);
        self.accelerator_types = ::std::option::Option::Some(v);
        self
    }
    /// <p> The available accelerator types. </p>
    pub fn set_accelerator_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AcceleratorType>>) -> Self {
        self.accelerator_types = input;
        self
    }
    /// <p> The available accelerator types. </p>
    pub fn get_accelerator_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AcceleratorType>> {
        &self.accelerator_types
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAcceleratorTypesOutput`](crate::operation::describe_accelerator_types::DescribeAcceleratorTypesOutput).
    pub fn build(self) -> crate::operation::describe_accelerator_types::DescribeAcceleratorTypesOutput {
        crate::operation::describe_accelerator_types::DescribeAcceleratorTypesOutput {
            accelerator_types: self.accelerator_types,
            _request_id: self._request_id,
        }
    }
}
