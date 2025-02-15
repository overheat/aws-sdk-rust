// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn get_sms_sandbox_account_status_output_correct_errors(
    mut builder: crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusOutputBuilder,
) -> crate::operation::get_sms_sandbox_account_status::builders::GetSmsSandboxAccountStatusOutputBuilder {
    if builder.is_in_sandbox.is_none() {
        builder.is_in_sandbox = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn list_sms_sandbox_phone_numbers_output_correct_errors(
    mut builder: crate::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersOutputBuilder,
) -> crate::operation::list_sms_sandbox_phone_numbers::builders::ListSmsSandboxPhoneNumbersOutputBuilder {
    if builder.phone_numbers.is_none() {
        builder.phone_numbers = Some(Default::default())
    }
    builder
}

pub(crate) fn verification_exception_correct_errors(
    mut builder: crate::types::error::builders::VerificationExceptionBuilder,
) -> crate::types::error::builders::VerificationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = Some(Default::default())
    }
    builder
}

pub(crate) fn batch_result_error_entry_correct_errors(
    mut builder: crate::types::builders::BatchResultErrorEntryBuilder,
) -> crate::types::builders::BatchResultErrorEntryBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.code.is_none() {
        builder.code = Some(Default::default())
    }
    if builder.sender_fault.is_none() {
        builder.sender_fault = Some(Default::default())
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}
