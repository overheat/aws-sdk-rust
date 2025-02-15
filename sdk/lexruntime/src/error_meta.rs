// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGatewayException(crate::types::error::BadGatewayException),
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    ConflictException(crate::types::error::ConflictException),
    /// <p> One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, </p>
    /// <ul>
    /// <li> <p>If Amazon Lex does not have sufficient permissions to call a Lambda function.</p> </li>
    /// <li> <p>If a Lambda function takes longer than 30 seconds to execute.</p> </li>
    /// <li> <p>If a fulfillment Lambda function returns a <code>Delegate</code> dialog action without removing any slot values.</p> </li>
    /// </ul>
    DependencyFailedException(crate::types::error::DependencyFailedException),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailureException(crate::types::error::InternalFailureException),
    /// <p>Exceeded a limit.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>This exception is not used.</p>
    LoopDetectedException(crate::types::error::LoopDetectedException),
    /// <p>The accept header in the request does not have a valid value.</p>
    NotAcceptableException(crate::types::error::NotAcceptableException),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFoundException(crate::types::error::NotFoundException),
    /// <p>The input speech is too long.</p>
    RequestTimeoutException(crate::types::error::RequestTimeoutException),
    /// <p>The Content-Type header (<code>PostContent</code> API) has an invalid value. </p>
    UnsupportedMediaTypeException(crate::types::error::UnsupportedMediaTypeException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadGatewayException(inner) => inner.fmt(f),
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DependencyFailedException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::LoopDetectedException(inner) => inner.fmt(f),
            Error::NotAcceptableException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::RequestTimeoutException(inner) => inner.fmt(f),
            Error::UnsupportedMediaTypeException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_session::DeleteSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_session::DeleteSessionError, R>) -> Self {
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
impl From<crate::operation::delete_session::DeleteSessionError> for Error {
    fn from(err: crate::operation::delete_session::DeleteSessionError) -> Self {
        match err {
            crate::operation::delete_session::DeleteSessionError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::delete_session::DeleteSessionError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_session::DeleteSessionError::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::operation::delete_session::DeleteSessionError::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::operation::delete_session::DeleteSessionError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::delete_session::DeleteSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session::GetSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session::GetSessionError, R>) -> Self {
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
impl From<crate::operation::get_session::GetSessionError> for Error {
    fn from(err: crate::operation::get_session::GetSessionError) -> Self {
        match err {
            crate::operation::get_session::GetSessionError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::get_session::GetSessionError::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::operation::get_session::GetSessionError::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::operation::get_session::GetSessionError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::get_session::GetSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::post_content::PostContentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::post_content::PostContentError, R>) -> Self {
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
impl From<crate::operation::post_content::PostContentError> for Error {
    fn from(err: crate::operation::post_content::PostContentError) -> Self {
        match err {
            crate::operation::post_content::PostContentError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::post_content::PostContentError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::post_content::PostContentError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::post_content::PostContentError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::post_content::PostContentError::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::operation::post_content::PostContentError::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::operation::post_content::PostContentError::LoopDetectedException(inner) => Error::LoopDetectedException(inner),
            crate::operation::post_content::PostContentError::NotAcceptableException(inner) => Error::NotAcceptableException(inner),
            crate::operation::post_content::PostContentError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::post_content::PostContentError::RequestTimeoutException(inner) => Error::RequestTimeoutException(inner),
            crate::operation::post_content::PostContentError::UnsupportedMediaTypeException(inner) => Error::UnsupportedMediaTypeException(inner),
            crate::operation::post_content::PostContentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::post_text::PostTextError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::post_text::PostTextError, R>) -> Self {
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
impl From<crate::operation::post_text::PostTextError> for Error {
    fn from(err: crate::operation::post_text::PostTextError) -> Self {
        match err {
            crate::operation::post_text::PostTextError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::post_text::PostTextError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::post_text::PostTextError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::post_text::PostTextError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::post_text::PostTextError::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::operation::post_text::PostTextError::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::operation::post_text::PostTextError::LoopDetectedException(inner) => Error::LoopDetectedException(inner),
            crate::operation::post_text::PostTextError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::post_text::PostTextError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_session::PutSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_session::PutSessionError, R>) -> Self {
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
impl From<crate::operation::put_session::PutSessionError> for Error {
    fn from(err: crate::operation::put_session::PutSessionError) -> Self {
        match err {
            crate::operation::put_session::PutSessionError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::put_session::PutSessionError::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::operation::put_session::PutSessionError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::put_session::PutSessionError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::put_session::PutSessionError::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::operation::put_session::PutSessionError::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::operation::put_session::PutSessionError::NotAcceptableException(inner) => Error::NotAcceptableException(inner),
            crate::operation::put_session::PutSessionError::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::operation::put_session::PutSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::BadGatewayException(inner) => inner.source(),
            Error::BadRequestException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::DependencyFailedException(inner) => inner.source(),
            Error::InternalFailureException(inner) => inner.source(),
            Error::LimitExceededException(inner) => inner.source(),
            Error::LoopDetectedException(inner) => inner.source(),
            Error::NotAcceptableException(inner) => inner.source(),
            Error::NotFoundException(inner) => inner.source(),
            Error::RequestTimeoutException(inner) => inner.source(),
            Error::UnsupportedMediaTypeException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::BadGatewayException(e) => e.request_id(),
            Self::BadRequestException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::DependencyFailedException(e) => e.request_id(),
            Self::InternalFailureException(e) => e.request_id(),
            Self::LimitExceededException(e) => e.request_id(),
            Self::LoopDetectedException(e) => e.request_id(),
            Self::NotAcceptableException(e) => e.request_id(),
            Self::NotFoundException(e) => e.request_id(),
            Self::RequestTimeoutException(e) => e.request_id(),
            Self::UnsupportedMediaTypeException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
