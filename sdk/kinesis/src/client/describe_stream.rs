// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeStream`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::set_stream_name):<br>required: **false**<br><p>The name of the stream to describe.</p><br>
    ///   - [`limit(i32)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 results are returned.</p><br>
    ///   - [`exclusive_start_shard_id(impl Into<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::exclusive_start_shard_id) / [`set_exclusive_start_shard_id(Option<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::set_exclusive_start_shard_id):<br>required: **false**<br><p>The shard ID of the shard to start with.</p>  <p>Specify this parameter to indicate that you want to describe the stream starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>  <p>If you don't specify this parameter, the default behavior for <code>DescribeStream</code> is to describe the stream starting with the first shard in the stream.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::set_stream_arn):<br>required: **false**<br><p>The ARN of the stream.</p><br>
    /// - On success, responds with [`DescribeStreamOutput`](crate::operation::describe_stream::DescribeStreamOutput) with field(s):
    ///   - [`stream_description(Option<StreamDescription>)`](crate::operation::describe_stream::DescribeStreamOutput::stream_description): <p>The current status of the stream, the stream Amazon Resource Name (ARN), an array of shard objects that comprise the stream, and whether there are more shards available.</p>
    /// - On failure, responds with [`SdkError<DescribeStreamError>`](crate::operation::describe_stream::DescribeStreamError)
    pub fn describe_stream(&self) -> crate::operation::describe_stream::builders::DescribeStreamFluentBuilder {
        crate::operation::describe_stream::builders::DescribeStreamFluentBuilder::new(self.handle.clone())
    }
}
