// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PublishType`](crate::operation::publish_type::builders::PublishTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`r#type(ThirdPartyType)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::type) / [`set_type(Option<ThirdPartyType>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::set_type):<br>required: **false**<br><p>The type of the extension.</p>  <p>Conditional: You must specify <code>Arn</code>, or <code>TypeName</code> and <code>Type</code>.</p><br>
    ///   - [`arn(impl Into<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::set_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the extension.</p>  <p>Conditional: You must specify <code>Arn</code>, or <code>TypeName</code> and <code>Type</code>.</p><br>
    ///   - [`type_name(impl Into<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::set_type_name):<br>required: **false**<br><p>The name of the extension.</p>  <p>Conditional: You must specify <code>Arn</code>, or <code>TypeName</code> and <code>Type</code>.</p><br>
    ///   - [`public_version_number(impl Into<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::public_version_number) / [`set_public_version_number(Option<String>)`](crate::operation::publish_type::builders::PublishTypeFluentBuilder::set_public_version_number):<br>required: **false**<br><p>The version number to assign to this version of the extension.</p>  <p>Use the following format, and adhere to semantic versioning when assigning a version number to your extension:</p>  <p> <code>MAJOR.MINOR.PATCH</code> </p>  <p>For more information, see <a href="https://semver.org/">Semantic Versioning 2.0.0</a>.</p>  <p>If you don't specify a version number, CloudFormation increments the version number by one minor version release.</p>  <p>You cannot specify a version number the first time you publish a type. CloudFormation automatically sets the first version number to be <code>1.0.0</code>.</p><br>
    /// - On success, responds with [`PublishTypeOutput`](crate::operation::publish_type::PublishTypeOutput) with field(s):
    ///   - [`public_type_arn(Option<String>)`](crate::operation::publish_type::PublishTypeOutput::public_type_arn): <p>The Amazon Resource Name (ARN) assigned to the public extension upon publication.</p>
    /// - On failure, responds with [`SdkError<PublishTypeError>`](crate::operation::publish_type::PublishTypeError)
    pub fn publish_type(&self) -> crate::operation::publish_type::builders::PublishTypeFluentBuilder {
        crate::operation::publish_type::builders::PublishTypeFluentBuilder::new(self.handle.clone())
    }
}
