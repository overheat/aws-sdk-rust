// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateMemberAccountInput {
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub member_account_id: ::std::option::Option<::std::string::String>,
}
impl AssociateMemberAccountInput {
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub fn member_account_id(&self) -> ::std::option::Option<&str> {
        self.member_account_id.as_deref()
    }
}
impl AssociateMemberAccountInput {
    /// Creates a new builder-style object to manufacture [`AssociateMemberAccountInput`](crate::operation::associate_member_account::AssociateMemberAccountInput).
    pub fn builder() -> crate::operation::associate_member_account::builders::AssociateMemberAccountInputBuilder {
        crate::operation::associate_member_account::builders::AssociateMemberAccountInputBuilder::default()
    }
}

/// A builder for [`AssociateMemberAccountInput`](crate::operation::associate_member_account::AssociateMemberAccountInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssociateMemberAccountInputBuilder {
    pub(crate) member_account_id: ::std::option::Option<::std::string::String>,
}
impl AssociateMemberAccountInputBuilder {
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    /// This field is required.
    pub fn member_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub fn set_member_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_account_id = input;
        self
    }
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub fn get_member_account_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.member_account_id
    }
    /// Consumes the builder and constructs a [`AssociateMemberAccountInput`](crate::operation::associate_member_account::AssociateMemberAccountInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_member_account::AssociateMemberAccountInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::associate_member_account::AssociateMemberAccountInput {
            member_account_id: self.member_account_id,
        })
    }
}
