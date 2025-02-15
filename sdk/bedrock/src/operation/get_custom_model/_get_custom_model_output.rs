// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCustomModelOutput {
    /// <p>ARN associated with this model.</p>
    pub model_arn: ::std::string::String,
    /// <p>Model name associated with this model.</p>
    pub model_name: ::std::string::String,
    /// <p>Job name associated with this model.</p>
    pub job_name: ::std::option::Option<::std::string::String>,
    /// <p>Job ARN associated with this model.</p>
    pub job_arn: ::std::string::String,
    /// <p>ARN of the base model.</p>
    pub base_model_arn: ::std::string::String,
    /// <p>The custom model is encrypted at rest using this key.</p>
    pub model_kms_key_arn: ::std::option::Option<::std::string::String>,
    /// <p>Hyperparameter values associated with this model.</p>
    pub hyper_parameters: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>Information about the training dataset.</p>
    pub training_data_config: ::std::option::Option<crate::types::TrainingDataConfig>,
    /// <p>Array of up to 10 validators.</p>
    pub validation_data_config: ::std::option::Option<crate::types::ValidationDataConfig>,
    /// <p>Output data configuration associated with this custom model.</p>
    pub output_data_config: ::std::option::Option<crate::types::OutputDataConfig>,
    /// <p>The training metrics from the job creation.</p>
    pub training_metrics: ::std::option::Option<crate::types::TrainingMetrics>,
    /// <p>The validation metrics from the job creation.</p>
    pub validation_metrics: ::std::option::Option<::std::vec::Vec<crate::types::ValidatorMetric>>,
    /// <p>Creation time of the model.</p>
    pub creation_time: ::aws_smithy_types::DateTime,
    _request_id: Option<String>,
}
impl GetCustomModelOutput {
    /// <p>ARN associated with this model.</p>
    pub fn model_arn(&self) -> &str {
        use std::ops::Deref;
        self.model_arn.deref()
    }
    /// <p>Model name associated with this model.</p>
    pub fn model_name(&self) -> &str {
        use std::ops::Deref;
        self.model_name.deref()
    }
    /// <p>Job name associated with this model.</p>
    pub fn job_name(&self) -> ::std::option::Option<&str> {
        self.job_name.as_deref()
    }
    /// <p>Job ARN associated with this model.</p>
    pub fn job_arn(&self) -> &str {
        use std::ops::Deref;
        self.job_arn.deref()
    }
    /// <p>ARN of the base model.</p>
    pub fn base_model_arn(&self) -> &str {
        use std::ops::Deref;
        self.base_model_arn.deref()
    }
    /// <p>The custom model is encrypted at rest using this key.</p>
    pub fn model_kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.model_kms_key_arn.as_deref()
    }
    /// <p>Hyperparameter values associated with this model.</p>
    pub fn hyper_parameters(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.hyper_parameters.as_ref()
    }
    /// <p>Information about the training dataset.</p>
    pub fn training_data_config(&self) -> ::std::option::Option<&crate::types::TrainingDataConfig> {
        self.training_data_config.as_ref()
    }
    /// <p>Array of up to 10 validators.</p>
    pub fn validation_data_config(&self) -> ::std::option::Option<&crate::types::ValidationDataConfig> {
        self.validation_data_config.as_ref()
    }
    /// <p>Output data configuration associated with this custom model.</p>
    pub fn output_data_config(&self) -> ::std::option::Option<&crate::types::OutputDataConfig> {
        self.output_data_config.as_ref()
    }
    /// <p>The training metrics from the job creation.</p>
    pub fn training_metrics(&self) -> ::std::option::Option<&crate::types::TrainingMetrics> {
        self.training_metrics.as_ref()
    }
    /// <p>The validation metrics from the job creation.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.validation_metrics.is_none()`.
    pub fn validation_metrics(&self) -> &[crate::types::ValidatorMetric] {
        self.validation_metrics.as_deref().unwrap_or_default()
    }
    /// <p>Creation time of the model.</p>
    pub fn creation_time(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_time
    }
}
impl ::aws_http::request_id::RequestId for GetCustomModelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCustomModelOutput {
    /// Creates a new builder-style object to manufacture [`GetCustomModelOutput`](crate::operation::get_custom_model::GetCustomModelOutput).
    pub fn builder() -> crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder {
        crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::default()
    }
}

/// A builder for [`GetCustomModelOutput`](crate::operation::get_custom_model::GetCustomModelOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetCustomModelOutputBuilder {
    pub(crate) model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) model_name: ::std::option::Option<::std::string::String>,
    pub(crate) job_name: ::std::option::Option<::std::string::String>,
    pub(crate) job_arn: ::std::option::Option<::std::string::String>,
    pub(crate) base_model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) model_kms_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) hyper_parameters: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) training_data_config: ::std::option::Option<crate::types::TrainingDataConfig>,
    pub(crate) validation_data_config: ::std::option::Option<crate::types::ValidationDataConfig>,
    pub(crate) output_data_config: ::std::option::Option<crate::types::OutputDataConfig>,
    pub(crate) training_metrics: ::std::option::Option<crate::types::TrainingMetrics>,
    pub(crate) validation_metrics: ::std::option::Option<::std::vec::Vec<crate::types::ValidatorMetric>>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetCustomModelOutputBuilder {
    /// <p>ARN associated with this model.</p>
    /// This field is required.
    pub fn model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN associated with this model.</p>
    pub fn set_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_arn = input;
        self
    }
    /// <p>ARN associated with this model.</p>
    pub fn get_model_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_arn
    }
    /// <p>Model name associated with this model.</p>
    /// This field is required.
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Model name associated with this model.</p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_name = input;
        self
    }
    /// <p>Model name associated with this model.</p>
    pub fn get_model_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_name
    }
    /// <p>Job name associated with this model.</p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Job name associated with this model.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// <p>Job name associated with this model.</p>
    pub fn get_job_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_name
    }
    /// <p>Job ARN associated with this model.</p>
    /// This field is required.
    pub fn job_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Job ARN associated with this model.</p>
    pub fn set_job_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_arn = input;
        self
    }
    /// <p>Job ARN associated with this model.</p>
    pub fn get_job_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_arn
    }
    /// <p>ARN of the base model.</p>
    /// This field is required.
    pub fn base_model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.base_model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the base model.</p>
    pub fn set_base_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.base_model_arn = input;
        self
    }
    /// <p>ARN of the base model.</p>
    pub fn get_base_model_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.base_model_arn
    }
    /// <p>The custom model is encrypted at rest using this key.</p>
    pub fn model_kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom model is encrypted at rest using this key.</p>
    pub fn set_model_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_kms_key_arn = input;
        self
    }
    /// <p>The custom model is encrypted at rest using this key.</p>
    pub fn get_model_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_kms_key_arn
    }
    /// Adds a key-value pair to `hyper_parameters`.
    ///
    /// To override the contents of this collection use [`set_hyper_parameters`](Self::set_hyper_parameters).
    ///
    /// <p>Hyperparameter values associated with this model.</p>
    pub fn hyper_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.hyper_parameters.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.hyper_parameters = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Hyperparameter values associated with this model.</p>
    pub fn set_hyper_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.hyper_parameters = input;
        self
    }
    /// <p>Hyperparameter values associated with this model.</p>
    pub fn get_hyper_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.hyper_parameters
    }
    /// <p>Information about the training dataset.</p>
    /// This field is required.
    pub fn training_data_config(mut self, input: crate::types::TrainingDataConfig) -> Self {
        self.training_data_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the training dataset.</p>
    pub fn set_training_data_config(mut self, input: ::std::option::Option<crate::types::TrainingDataConfig>) -> Self {
        self.training_data_config = input;
        self
    }
    /// <p>Information about the training dataset.</p>
    pub fn get_training_data_config(&self) -> &::std::option::Option<crate::types::TrainingDataConfig> {
        &self.training_data_config
    }
    /// <p>Array of up to 10 validators.</p>
    pub fn validation_data_config(mut self, input: crate::types::ValidationDataConfig) -> Self {
        self.validation_data_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Array of up to 10 validators.</p>
    pub fn set_validation_data_config(mut self, input: ::std::option::Option<crate::types::ValidationDataConfig>) -> Self {
        self.validation_data_config = input;
        self
    }
    /// <p>Array of up to 10 validators.</p>
    pub fn get_validation_data_config(&self) -> &::std::option::Option<crate::types::ValidationDataConfig> {
        &self.validation_data_config
    }
    /// <p>Output data configuration associated with this custom model.</p>
    /// This field is required.
    pub fn output_data_config(mut self, input: crate::types::OutputDataConfig) -> Self {
        self.output_data_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Output data configuration associated with this custom model.</p>
    pub fn set_output_data_config(mut self, input: ::std::option::Option<crate::types::OutputDataConfig>) -> Self {
        self.output_data_config = input;
        self
    }
    /// <p>Output data configuration associated with this custom model.</p>
    pub fn get_output_data_config(&self) -> &::std::option::Option<crate::types::OutputDataConfig> {
        &self.output_data_config
    }
    /// <p>The training metrics from the job creation.</p>
    pub fn training_metrics(mut self, input: crate::types::TrainingMetrics) -> Self {
        self.training_metrics = ::std::option::Option::Some(input);
        self
    }
    /// <p>The training metrics from the job creation.</p>
    pub fn set_training_metrics(mut self, input: ::std::option::Option<crate::types::TrainingMetrics>) -> Self {
        self.training_metrics = input;
        self
    }
    /// <p>The training metrics from the job creation.</p>
    pub fn get_training_metrics(&self) -> &::std::option::Option<crate::types::TrainingMetrics> {
        &self.training_metrics
    }
    /// Appends an item to `validation_metrics`.
    ///
    /// To override the contents of this collection use [`set_validation_metrics`](Self::set_validation_metrics).
    ///
    /// <p>The validation metrics from the job creation.</p>
    pub fn validation_metrics(mut self, input: crate::types::ValidatorMetric) -> Self {
        let mut v = self.validation_metrics.unwrap_or_default();
        v.push(input);
        self.validation_metrics = ::std::option::Option::Some(v);
        self
    }
    /// <p>The validation metrics from the job creation.</p>
    pub fn set_validation_metrics(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ValidatorMetric>>) -> Self {
        self.validation_metrics = input;
        self
    }
    /// <p>The validation metrics from the job creation.</p>
    pub fn get_validation_metrics(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ValidatorMetric>> {
        &self.validation_metrics
    }
    /// <p>Creation time of the model.</p>
    /// This field is required.
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Creation time of the model.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>Creation time of the model.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetCustomModelOutput`](crate::operation::get_custom_model::GetCustomModelOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`model_arn`](crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::model_arn)
    /// - [`model_name`](crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::model_name)
    /// - [`job_arn`](crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::job_arn)
    /// - [`base_model_arn`](crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::base_model_arn)
    /// - [`creation_time`](crate::operation::get_custom_model::builders::GetCustomModelOutputBuilder::creation_time)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_custom_model::GetCustomModelOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_custom_model::GetCustomModelOutput {
            model_arn: self.model_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "model_arn",
                    "model_arn was not specified but it is required when building GetCustomModelOutput",
                )
            })?,
            model_name: self.model_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "model_name",
                    "model_name was not specified but it is required when building GetCustomModelOutput",
                )
            })?,
            job_name: self.job_name,
            job_arn: self.job_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "job_arn",
                    "job_arn was not specified but it is required when building GetCustomModelOutput",
                )
            })?,
            base_model_arn: self.base_model_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "base_model_arn",
                    "base_model_arn was not specified but it is required when building GetCustomModelOutput",
                )
            })?,
            model_kms_key_arn: self.model_kms_key_arn,
            hyper_parameters: self.hyper_parameters,
            training_data_config: self.training_data_config,
            validation_data_config: self.validation_data_config,
            output_data_config: self.output_data_config,
            training_metrics: self.training_metrics,
            validation_metrics: self.validation_metrics,
            creation_time: self.creation_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_time",
                    "creation_time was not specified but it is required when building GetCustomModelOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
