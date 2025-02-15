// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSkillsStoreCategoriesOutput {
    /// <p>The list of categories.</p>
    pub category_list: ::std::option::Option<::std::vec::Vec<crate::types::Category>>,
    /// <p>The tokens used for pagination.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSkillsStoreCategoriesOutput {
    /// <p>The list of categories.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.category_list.is_none()`.
    pub fn category_list(&self) -> &[crate::types::Category] {
        self.category_list.as_deref().unwrap_or_default()
    }
    /// <p>The tokens used for pagination.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListSkillsStoreCategoriesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSkillsStoreCategoriesOutput {
    /// Creates a new builder-style object to manufacture [`ListSkillsStoreCategoriesOutput`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput).
    pub fn builder() -> crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesOutputBuilder {
        crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesOutputBuilder::default()
    }
}

/// A builder for [`ListSkillsStoreCategoriesOutput`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListSkillsStoreCategoriesOutputBuilder {
    pub(crate) category_list: ::std::option::Option<::std::vec::Vec<crate::types::Category>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSkillsStoreCategoriesOutputBuilder {
    /// Appends an item to `category_list`.
    ///
    /// To override the contents of this collection use [`set_category_list`](Self::set_category_list).
    ///
    /// <p>The list of categories.</p>
    pub fn category_list(mut self, input: crate::types::Category) -> Self {
        let mut v = self.category_list.unwrap_or_default();
        v.push(input);
        self.category_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of categories.</p>
    pub fn set_category_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Category>>) -> Self {
        self.category_list = input;
        self
    }
    /// <p>The list of categories.</p>
    pub fn get_category_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Category>> {
        &self.category_list
    }
    /// <p>The tokens used for pagination.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The tokens used for pagination.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The tokens used for pagination.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListSkillsStoreCategoriesOutput`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput).
    pub fn build(self) -> crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput {
        crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput {
            category_list: self.category_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
