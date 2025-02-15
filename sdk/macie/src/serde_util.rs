// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn s3_resource_classification_correct_errors(
    mut builder: crate::types::builders::S3ResourceClassificationBuilder,
) -> crate::types::builders::S3ResourceClassificationBuilder {
    if builder.bucket_name.is_none() {
        builder.bucket_name = Some(Default::default())
    }
    if builder.classification_type.is_none() {
        builder.classification_type = {
            let builder = crate::types::builders::ClassificationTypeBuilder::default();
            crate::serde_util::classification_type_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn classification_type_correct_errors(
    mut builder: crate::types::builders::ClassificationTypeBuilder,
) -> crate::types::builders::ClassificationTypeBuilder {
    if builder.one_time.is_none() {
        builder.one_time = "no value was set".parse::<crate::types::S3OneTimeClassificationType>().ok()
    }
    if builder.continuous.is_none() {
        builder.continuous = "no value was set".parse::<crate::types::S3ContinuousClassificationType>().ok()
    }
    builder
}

pub(crate) fn s3_resource_correct_errors(mut builder: crate::types::builders::S3ResourceBuilder) -> crate::types::builders::S3ResourceBuilder {
    if builder.bucket_name.is_none() {
        builder.bucket_name = Some(Default::default())
    }
    builder
}
