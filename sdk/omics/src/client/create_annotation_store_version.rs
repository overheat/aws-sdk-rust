// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAnnotationStoreVersion`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::set_name):<br>required: **true**<br><p> The name of an annotation store version from which versions are being created. </p><br>
    ///   - [`version_name(impl Into<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::version_name) / [`set_version_name(Option<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::set_version_name):<br>required: **true**<br><p> The name given to an annotation store version to distinguish it from other versions. </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::set_description):<br>required: **false**<br><p> The description of an annotation store version. </p><br>
    ///   - [`version_options(VersionOptions)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::version_options) / [`set_version_options(Option<VersionOptions>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::set_version_options):<br>required: **false**<br><p> The options for an annotation store version. </p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::set_tags):<br>required: **false**<br><p> Any tags added to annotation store version. </p><br>
    /// - On success, responds with [`CreateAnnotationStoreVersionOutput`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput) with field(s):
    ///   - [`id(String)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::id): <p> A generated ID for the annotation store </p>
    ///   - [`version_name(String)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::version_name): <p> The name given to an annotation store version to distinguish it from other versions. </p>
    ///   - [`store_id(String)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::store_id): <p> The ID for the annotation store from which new versions are being created. </p>
    ///   - [`version_options(Option<VersionOptions>)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::version_options): <p> The options for an annotation store version. </p>
    ///   - [`name(String)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::name): <p> The name given to an annotation store version to distinguish it from other versions. </p>
    ///   - [`status(VersionStatus)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::status): <p> The status of a annotation store version. </p>
    ///   - [`creation_time(DateTime)`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionOutput::creation_time): <p> The time stamp for the creation of an annotation store version. </p>
    /// - On failure, responds with [`SdkError<CreateAnnotationStoreVersionError>`](crate::operation::create_annotation_store_version::CreateAnnotationStoreVersionError)
    pub fn create_annotation_store_version(
        &self,
    ) -> crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder {
        crate::operation::create_annotation_store_version::builders::CreateAnnotationStoreVersionFluentBuilder::new(self.handle.clone())
    }
}
