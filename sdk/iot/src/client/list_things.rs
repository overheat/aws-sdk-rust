// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListThings`](crate::operation::list_things::builders::ListThingsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_things::builders::ListThingsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_next_token):<br>required: **false**<br><p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_things::builders::ListThingsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in this operation.</p><br>
    ///   - [`attribute_name(impl Into<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::attribute_name) / [`set_attribute_name(Option<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_attribute_name):<br>required: **false**<br><p>The attribute name used to search for things.</p><br>
    ///   - [`attribute_value(impl Into<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::attribute_value) / [`set_attribute_value(Option<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_attribute_value):<br>required: **false**<br><p>The attribute value used to search for things.</p><br>
    ///   - [`thing_type_name(impl Into<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::thing_type_name) / [`set_thing_type_name(Option<String>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_thing_type_name):<br>required: **false**<br><p>The name of the thing type used to search for things.</p><br>
    ///   - [`use_prefix_attribute_value(bool)`](crate::operation::list_things::builders::ListThingsFluentBuilder::use_prefix_attribute_value) / [`set_use_prefix_attribute_value(Option<bool>)`](crate::operation::list_things::builders::ListThingsFluentBuilder::set_use_prefix_attribute_value):<br>required: **false**<br><p>When <code>true</code>, the action returns the thing resources with attribute values that start with the <code>attributeValue</code> provided.</p>  <p>When <code>false</code>, or not present, the action returns only the thing resources with attribute values that match the entire <code>attributeValue</code> provided. </p><br>
    /// - On success, responds with [`ListThingsOutput`](crate::operation::list_things::ListThingsOutput) with field(s):
    ///   - [`things(Option<Vec::<ThingAttribute>>)`](crate::operation::list_things::ListThingsOutput::things): <p>The things.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_things::ListThingsOutput::next_token): <p>The token to use to get the next set of results. Will not be returned if operation has returned all results.</p>
    /// - On failure, responds with [`SdkError<ListThingsError>`](crate::operation::list_things::ListThingsError)
    pub fn list_things(&self) -> crate::operation::list_things::builders::ListThingsFluentBuilder {
        crate::operation::list_things::builders::ListThingsFluentBuilder::new(self.handle.clone())
    }
}
