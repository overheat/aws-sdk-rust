// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAnnotationStoreVersions`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::set_name):<br>required: **true**<br><p> The name of the annotation store from which versions are being deleted. </p><br>
    ///   - [`versions(impl Into<String>)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::versions) / [`set_versions(Option<Vec::<String>>)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::set_versions):<br>required: **true**<br><p> The versions of an annotation store to be deleted. </p><br>
    ///   - [`force(bool)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::force) / [`set_force(Option<bool>)`](crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::set_force):<br>required: **false**<br><p> Forces the deletion of an annotation store version when imports are in-progress.. </p><br>
    /// - On success, responds with [`DeleteAnnotationStoreVersionsOutput`](crate::operation::delete_annotation_store_versions::DeleteAnnotationStoreVersionsOutput) with field(s):
    ///   - [`errors(Option<Vec::<VersionDeleteError>>)`](crate::operation::delete_annotation_store_versions::DeleteAnnotationStoreVersionsOutput::errors): <p> Any errors that occur when attempting to delete an annotation store version. </p>
    /// - On failure, responds with [`SdkError<DeleteAnnotationStoreVersionsError>`](crate::operation::delete_annotation_store_versions::DeleteAnnotationStoreVersionsError)
    pub fn delete_annotation_store_versions(
        &self,
    ) -> crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder {
        crate::operation::delete_annotation_store_versions::builders::DeleteAnnotationStoreVersionsFluentBuilder::new(self.handle.clone())
    }
}
