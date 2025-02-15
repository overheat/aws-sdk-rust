// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ApproveSkillOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for ApproveSkillOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ApproveSkillOutput {
    /// Creates a new builder-style object to manufacture [`ApproveSkillOutput`](crate::operation::approve_skill::ApproveSkillOutput).
    pub fn builder() -> crate::operation::approve_skill::builders::ApproveSkillOutputBuilder {
        crate::operation::approve_skill::builders::ApproveSkillOutputBuilder::default()
    }
}

/// A builder for [`ApproveSkillOutput`](crate::operation::approve_skill::ApproveSkillOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ApproveSkillOutputBuilder {
    _request_id: Option<String>,
}
impl ApproveSkillOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ApproveSkillOutput`](crate::operation::approve_skill::ApproveSkillOutput).
    pub fn build(self) -> crate::operation::approve_skill::ApproveSkillOutput {
        crate::operation::approve_skill::ApproveSkillOutput {
            _request_id: self._request_id,
        }
    }
}
