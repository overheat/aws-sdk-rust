// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvokeScreenAutomationOutput {
    /// <p>The updated workbook cursor after performing the automation action.</p>
    pub workbook_cursor: i64,
    _request_id: Option<String>,
}
impl InvokeScreenAutomationOutput {
    /// <p>The updated workbook cursor after performing the automation action.</p>
    pub fn workbook_cursor(&self) -> i64 {
        self.workbook_cursor
    }
}
impl ::aws_http::request_id::RequestId for InvokeScreenAutomationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl InvokeScreenAutomationOutput {
    /// Creates a new builder-style object to manufacture [`InvokeScreenAutomationOutput`](crate::operation::invoke_screen_automation::InvokeScreenAutomationOutput).
    pub fn builder() -> crate::operation::invoke_screen_automation::builders::InvokeScreenAutomationOutputBuilder {
        crate::operation::invoke_screen_automation::builders::InvokeScreenAutomationOutputBuilder::default()
    }
}

/// A builder for [`InvokeScreenAutomationOutput`](crate::operation::invoke_screen_automation::InvokeScreenAutomationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InvokeScreenAutomationOutputBuilder {
    pub(crate) workbook_cursor: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl InvokeScreenAutomationOutputBuilder {
    /// <p>The updated workbook cursor after performing the automation action.</p>
    /// This field is required.
    pub fn workbook_cursor(mut self, input: i64) -> Self {
        self.workbook_cursor = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated workbook cursor after performing the automation action.</p>
    pub fn set_workbook_cursor(mut self, input: ::std::option::Option<i64>) -> Self {
        self.workbook_cursor = input;
        self
    }
    /// <p>The updated workbook cursor after performing the automation action.</p>
    pub fn get_workbook_cursor(&self) -> &::std::option::Option<i64> {
        &self.workbook_cursor
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`InvokeScreenAutomationOutput`](crate::operation::invoke_screen_automation::InvokeScreenAutomationOutput).
    pub fn build(self) -> crate::operation::invoke_screen_automation::InvokeScreenAutomationOutput {
        crate::operation::invoke_screen_automation::InvokeScreenAutomationOutput {
            workbook_cursor: self.workbook_cursor.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}
