// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_acknowledge_order_receipt_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::acknowledge_order_receipt::AcknowledgeOrderReceiptInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.order_arn {
        object.key("orderArn").string(var_1.as_str());
    }
    Ok(())
}
