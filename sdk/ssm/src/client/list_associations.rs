// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAssociations`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`association_filter_list(AssociationFilter)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::association_filter_list) / [`set_association_filter_list(Option<Vec::<AssociationFilter>>)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::set_association_filter_list):<br>required: **false**<br><p>One or more filters. Use a filter to return a more specific list of results.</p> <note>   <p>Filtering associations using the <code>InstanceID</code> attribute only returns legacy associations created using the <code>InstanceID</code> attribute. Associations targeting the managed node that are part of the Target Attributes <code>ResourceGroup</code> or <code>Tags</code> aren't returned.</p>  </note><br>
    ///   - [`max_results(i32)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_associations::builders::ListAssociationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of items to return. (You received this token from a previous call.)</p><br>
    /// - On success, responds with [`ListAssociationsOutput`](crate::operation::list_associations::ListAssociationsOutput) with field(s):
    ///   - [`associations(Option<Vec::<Association>>)`](crate::operation::list_associations::ListAssociationsOutput::associations): <p>The associations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_associations::ListAssociationsOutput::next_token): <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    /// - On failure, responds with [`SdkError<ListAssociationsError>`](crate::operation::list_associations::ListAssociationsError)
    pub fn list_associations(&self) -> crate::operation::list_associations::builders::ListAssociationsFluentBuilder {
        crate::operation::list_associations::builders::ListAssociationsFluentBuilder::new(self.handle.clone())
    }
}
