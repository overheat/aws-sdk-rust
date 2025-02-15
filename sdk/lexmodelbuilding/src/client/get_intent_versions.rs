// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetIntentVersions`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::set_name):<br>required: **true**<br><p>The name of the intent for which versions should be returned.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::set_next_token):<br>required: **false**<br><p>A pagination token for fetching the next page of intent versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p><br>
    ///   - [`max_results(i32)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of intent versions to return in the response. The default is 10.</p><br>
    /// - On success, responds with [`GetIntentVersionsOutput`](crate::operation::get_intent_versions::GetIntentVersionsOutput) with field(s):
    ///   - [`intents(Option<Vec::<IntentMetadata>>)`](crate::operation::get_intent_versions::GetIntentVersionsOutput::intents): <p>An array of <code>IntentMetadata</code> objects, one for each numbered version of the intent plus one for the <code>$LATEST</code> version.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_intent_versions::GetIntentVersionsOutput::next_token): <p>A pagination token for fetching the next page of intent versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    /// - On failure, responds with [`SdkError<GetIntentVersionsError>`](crate::operation::get_intent_versions::GetIntentVersionsError)
    pub fn get_intent_versions(&self) -> crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder {
        crate::operation::get_intent_versions::builders::GetIntentVersionsFluentBuilder::new(self.handle.clone())
    }
}
