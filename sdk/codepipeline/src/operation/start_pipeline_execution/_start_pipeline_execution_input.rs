// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the input of a <code>StartPipelineExecution</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartPipelineExecutionInput {
    /// <p>The name of the pipeline to start.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A list that overrides pipeline variables for a pipeline execution that's being started. Variable names must match <code>[A-Za-z0-9@\-_]+</code>, and the values can be anything except an empty string.</p>
    pub variables: ::std::option::Option<::std::vec::Vec<crate::types::PipelineVariable>>,
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    pub client_request_token: ::std::option::Option<::std::string::String>,
}
impl StartPipelineExecutionInput {
    /// <p>The name of the pipeline to start.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A list that overrides pipeline variables for a pipeline execution that's being started. Variable names must match <code>[A-Za-z0-9@\-_]+</code>, and the values can be anything except an empty string.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.variables.is_none()`.
    pub fn variables(&self) -> &[crate::types::PipelineVariable] {
        self.variables.as_deref().unwrap_or_default()
    }
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
}
impl StartPipelineExecutionInput {
    /// Creates a new builder-style object to manufacture [`StartPipelineExecutionInput`](crate::operation::start_pipeline_execution::StartPipelineExecutionInput).
    pub fn builder() -> crate::operation::start_pipeline_execution::builders::StartPipelineExecutionInputBuilder {
        crate::operation::start_pipeline_execution::builders::StartPipelineExecutionInputBuilder::default()
    }
}

/// A builder for [`StartPipelineExecutionInput`](crate::operation::start_pipeline_execution::StartPipelineExecutionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StartPipelineExecutionInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) variables: ::std::option::Option<::std::vec::Vec<crate::types::PipelineVariable>>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
}
impl StartPipelineExecutionInputBuilder {
    /// <p>The name of the pipeline to start.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline to start.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the pipeline to start.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Appends an item to `variables`.
    ///
    /// To override the contents of this collection use [`set_variables`](Self::set_variables).
    ///
    /// <p>A list that overrides pipeline variables for a pipeline execution that's being started. Variable names must match <code>[A-Za-z0-9@\-_]+</code>, and the values can be anything except an empty string.</p>
    pub fn variables(mut self, input: crate::types::PipelineVariable) -> Self {
        let mut v = self.variables.unwrap_or_default();
        v.push(input);
        self.variables = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list that overrides pipeline variables for a pipeline execution that's being started. Variable names must match <code>[A-Za-z0-9@\-_]+</code>, and the values can be anything except an empty string.</p>
    pub fn set_variables(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PipelineVariable>>) -> Self {
        self.variables = input;
        self
    }
    /// <p>A list that overrides pipeline variables for a pipeline execution that's being started. Variable names must match <code>[A-Za-z0-9@\-_]+</code>, and the values can be anything except an empty string.</p>
    pub fn get_variables(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PipelineVariable>> {
        &self.variables
    }
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>The system-generated unique ID used to identify a unique execution request.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_request_token
    }
    /// Consumes the builder and constructs a [`StartPipelineExecutionInput`](crate::operation::start_pipeline_execution::StartPipelineExecutionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_pipeline_execution::StartPipelineExecutionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_pipeline_execution::StartPipelineExecutionInput {
            name: self.name,
            variables: self.variables,
            client_request_token: self.client_request_token,
        })
    }
}
