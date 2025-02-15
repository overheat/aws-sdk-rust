// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>Access is denied. Your account is not authorized to perform this operation.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>The data store is in a transition state and the user requested action can not be performed.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>Unknown error occurs in the service.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p> The requested data store was not found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The user has exceeded their maximum number of allowed calls to the given API. </p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The user input parameter was invalid.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_fhir_datastore::CreateFHIRDatastoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_fhir_datastore::CreateFHIRDatastoreError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::create_fhir_datastore::CreateFHIRDatastoreError> for Error {
    fn from(err: crate::operation::create_fhir_datastore::CreateFHIRDatastoreError) -> Self {
        match err {
            crate::operation::create_fhir_datastore::CreateFHIRDatastoreError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::create_fhir_datastore::CreateFHIRDatastoreError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::create_fhir_datastore::CreateFHIRDatastoreError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::create_fhir_datastore::CreateFHIRDatastoreError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_fhir_datastore::CreateFHIRDatastoreError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError> for Error {
    fn from(err: crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError) -> Self {
        match err {
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_fhir_datastore::DeleteFHIRDatastoreError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError> for Error {
    fn from(err: crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError) -> Self {
        match err {
            crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::describe_fhir_datastore::DescribeFHIRDatastoreError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError> for Error {
    fn from(err: crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError) -> Self {
        match err {
            crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::describe_fhir_export_job::DescribeFHIRExportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError> for Error {
    fn from(err: crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError) -> Self {
        match err {
            crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::describe_fhir_import_job::DescribeFHIRImportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_datastores::ListFHIRDatastoresError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_datastores::ListFHIRDatastoresError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_fhir_datastores::ListFHIRDatastoresError> for Error {
    fn from(err: crate::operation::list_fhir_datastores::ListFHIRDatastoresError) -> Self {
        match err {
            crate::operation::list_fhir_datastores::ListFHIRDatastoresError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_fhir_datastores::ListFHIRDatastoresError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_fhir_datastores::ListFHIRDatastoresError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_fhir_datastores::ListFHIRDatastoresError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError> for Error {
    fn from(err: crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError) -> Self {
        match err {
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_fhir_export_jobs::ListFHIRExportJobsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError> for Error {
    fn from(err: crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError) -> Self {
        match err {
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_fhir_import_jobs::ListFHIRImportJobsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_fhir_export_job::StartFHIRExportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_fhir_export_job::StartFHIRExportJobError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_fhir_export_job::StartFHIRExportJobError> for Error {
    fn from(err: crate::operation::start_fhir_export_job::StartFHIRExportJobError) -> Self {
        match err {
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::start_fhir_export_job::StartFHIRExportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_fhir_import_job::StartFHIRImportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_fhir_import_job::StartFHIRImportJobError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_fhir_import_job::StartFHIRImportJobError> for Error {
    fn from(err: crate::operation::start_fhir_import_job::StartFHIRImportJobError) -> Self {
        match err {
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::start_fhir_import_job::StartFHIRImportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone())
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
