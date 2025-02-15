// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTeamMemberOutput {
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    pub user_arn: ::std::option::Option<::std::string::String>,
    /// <p>The project role granted to the user.</p>
    pub project_role: ::std::option::Option<::std::string::String>,
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    pub remote_access_allowed: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl UpdateTeamMemberOutput {
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    pub fn user_arn(&self) -> ::std::option::Option<&str> {
        self.user_arn.as_deref()
    }
    /// <p>The project role granted to the user.</p>
    pub fn project_role(&self) -> ::std::option::Option<&str> {
        self.project_role.as_deref()
    }
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    pub fn remote_access_allowed(&self) -> ::std::option::Option<bool> {
        self.remote_access_allowed
    }
}
impl ::aws_http::request_id::RequestId for UpdateTeamMemberOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateTeamMemberOutput {
    /// Creates a new builder-style object to manufacture [`UpdateTeamMemberOutput`](crate::operation::update_team_member::UpdateTeamMemberOutput).
    pub fn builder() -> crate::operation::update_team_member::builders::UpdateTeamMemberOutputBuilder {
        crate::operation::update_team_member::builders::UpdateTeamMemberOutputBuilder::default()
    }
}

/// A builder for [`UpdateTeamMemberOutput`](crate::operation::update_team_member::UpdateTeamMemberOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateTeamMemberOutputBuilder {
    pub(crate) user_arn: ::std::option::Option<::std::string::String>,
    pub(crate) project_role: ::std::option::Option<::std::string::String>,
    pub(crate) remote_access_allowed: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl UpdateTeamMemberOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    pub fn user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    pub fn set_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the user whose team membership attributes were updated.</p>
    pub fn get_user_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_arn
    }
    /// <p>The project role granted to the user.</p>
    pub fn project_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The project role granted to the user.</p>
    pub fn set_project_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_role = input;
        self
    }
    /// <p>The project role granted to the user.</p>
    pub fn get_project_role(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_role
    }
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    pub fn remote_access_allowed(mut self, input: bool) -> Self {
        self.remote_access_allowed = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    pub fn set_remote_access_allowed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.remote_access_allowed = input;
        self
    }
    /// <p>Whether a team member is allowed to remotely access project resources using the SSH public key associated with the user's profile.</p>
    pub fn get_remote_access_allowed(&self) -> &::std::option::Option<bool> {
        &self.remote_access_allowed
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateTeamMemberOutput`](crate::operation::update_team_member::UpdateTeamMemberOutput).
    pub fn build(self) -> crate::operation::update_team_member::UpdateTeamMemberOutput {
        crate::operation::update_team_member::UpdateTeamMemberOutput {
            user_arn: self.user_arn,
            project_role: self.project_role,
            remote_access_allowed: self.remote_access_allowed,
            _request_id: self._request_id,
        }
    }
}
