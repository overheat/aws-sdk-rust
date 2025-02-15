// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEngineDefaultParameters`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`cache_parameter_group_family(impl Into<String>)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::cache_parameter_group_family) / [`set_cache_parameter_group_family(Option<String>)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::set_cache_parameter_group_family):<br>required: **true**<br><p>The name of the cache parameter group family.</p>  <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | <code>redis6.2</code> | <code>redis7</code> </p><br>
    ///   - [`max_records(i32)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::max_records) / [`set_max_records(Option<i32>)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::set_max_records):<br>required: **false**<br><p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>  <p>Default: 100</p>  <p>Constraints: minimum 20; maximum 100.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::set_marker):<br>required: **false**<br><p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p><br>
    /// - On success, responds with [`DescribeEngineDefaultParametersOutput`](crate::operation::describe_engine_default_parameters::DescribeEngineDefaultParametersOutput) with field(s):
    ///   - [`engine_defaults(Option<EngineDefaults>)`](crate::operation::describe_engine_default_parameters::DescribeEngineDefaultParametersOutput::engine_defaults): <p>Represents the output of a <code>DescribeEngineDefaultParameters</code> operation.</p>
    /// - On failure, responds with [`SdkError<DescribeEngineDefaultParametersError>`](crate::operation::describe_engine_default_parameters::DescribeEngineDefaultParametersError)
    pub fn describe_engine_default_parameters(
        &self,
    ) -> crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder {
        crate::operation::describe_engine_default_parameters::builders::DescribeEngineDefaultParametersFluentBuilder::new(self.handle.clone())
    }
}
