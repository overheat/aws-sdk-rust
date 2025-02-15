// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAddressBookOutput {
    /// <p>The details of the requested address book.</p>
    pub address_book: ::std::option::Option<crate::types::AddressBook>,
    _request_id: Option<String>,
}
impl GetAddressBookOutput {
    /// <p>The details of the requested address book.</p>
    pub fn address_book(&self) -> ::std::option::Option<&crate::types::AddressBook> {
        self.address_book.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetAddressBookOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAddressBookOutput {
    /// Creates a new builder-style object to manufacture [`GetAddressBookOutput`](crate::operation::get_address_book::GetAddressBookOutput).
    pub fn builder() -> crate::operation::get_address_book::builders::GetAddressBookOutputBuilder {
        crate::operation::get_address_book::builders::GetAddressBookOutputBuilder::default()
    }
}

/// A builder for [`GetAddressBookOutput`](crate::operation::get_address_book::GetAddressBookOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetAddressBookOutputBuilder {
    pub(crate) address_book: ::std::option::Option<crate::types::AddressBook>,
    _request_id: Option<String>,
}
impl GetAddressBookOutputBuilder {
    /// <p>The details of the requested address book.</p>
    pub fn address_book(mut self, input: crate::types::AddressBook) -> Self {
        self.address_book = ::std::option::Option::Some(input);
        self
    }
    /// <p>The details of the requested address book.</p>
    pub fn set_address_book(mut self, input: ::std::option::Option<crate::types::AddressBook>) -> Self {
        self.address_book = input;
        self
    }
    /// <p>The details of the requested address book.</p>
    pub fn get_address_book(&self) -> &::std::option::Option<crate::types::AddressBook> {
        &self.address_book
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetAddressBookOutput`](crate::operation::get_address_book::GetAddressBookOutput).
    pub fn build(self) -> crate::operation::get_address_book::GetAddressBookOutput {
        crate::operation::get_address_book::GetAddressBookOutput {
            address_book: self.address_book,
            _request_id: self._request_id,
        }
    }
}
