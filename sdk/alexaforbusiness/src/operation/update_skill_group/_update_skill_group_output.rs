// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSkillGroupOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for UpdateSkillGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateSkillGroupOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSkillGroupOutput`](crate::operation::update_skill_group::UpdateSkillGroupOutput).
    pub fn builder() -> crate::operation::update_skill_group::builders::UpdateSkillGroupOutputBuilder {
        crate::operation::update_skill_group::builders::UpdateSkillGroupOutputBuilder::default()
    }
}

/// A builder for [`UpdateSkillGroupOutput`](crate::operation::update_skill_group::UpdateSkillGroupOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateSkillGroupOutputBuilder {
    _request_id: Option<String>,
}
impl UpdateSkillGroupOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSkillGroupOutput`](crate::operation::update_skill_group::UpdateSkillGroupOutput).
    pub fn build(self) -> crate::operation::update_skill_group::UpdateSkillGroupOutput {
        crate::operation::update_skill_group::UpdateSkillGroupOutput {
            _request_id: self._request_id,
        }
    }
}
