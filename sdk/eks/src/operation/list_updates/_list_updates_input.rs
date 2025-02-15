// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListUpdatesInput {
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    pub nodegroup_name: ::std::option::Option<::std::string::String>,
    /// <p>The names of the installed add-ons that have available updates.</p>
    pub addon_name: ::std::option::Option<::std::string::String>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl ListUpdatesInput {
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    pub fn nodegroup_name(&self) -> ::std::option::Option<&str> {
        self.nodegroup_name.as_deref()
    }
    /// <p>The names of the installed add-ons that have available updates.</p>
    pub fn addon_name(&self) -> ::std::option::Option<&str> {
        self.addon_name.as_deref()
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListUpdatesInput {
    /// Creates a new builder-style object to manufacture [`ListUpdatesInput`](crate::operation::list_updates::ListUpdatesInput).
    pub fn builder() -> crate::operation::list_updates::builders::ListUpdatesInputBuilder {
        crate::operation::list_updates::builders::ListUpdatesInputBuilder::default()
    }
}

/// A builder for [`ListUpdatesInput`](crate::operation::list_updates::ListUpdatesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListUpdatesInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) nodegroup_name: ::std::option::Option<::std::string::String>,
    pub(crate) addon_name: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListUpdatesInputBuilder {
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    pub fn nodegroup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.nodegroup_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    pub fn set_nodegroup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.nodegroup_name = input;
        self
    }
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    pub fn get_nodegroup_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.nodegroup_name
    }
    /// <p>The names of the installed add-ons that have available updates.</p>
    pub fn addon_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.addon_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The names of the installed add-ons that have available updates.</p>
    pub fn set_addon_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.addon_name = input;
        self
    }
    /// <p>The names of the installed add-ons that have available updates.</p>
    pub fn get_addon_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.addon_name
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`ListUpdatesInput`](crate::operation::list_updates::ListUpdatesInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_updates::ListUpdatesInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_updates::ListUpdatesInput {
            name: self.name,
            nodegroup_name: self.nodegroup_name,
            addon_name: self.addon_name,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
