// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPriceListFileUrl`](crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`price_list_arn(impl Into<String>)`](crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder::price_list_arn) / [`set_price_list_arn(Option<String>)`](crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder::set_price_list_arn):<br>required: **true**<br><p>The unique identifier that maps to where your Price List files are located. <code>PriceListArn</code> can be obtained from the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_pricing_ListPriceLists.html">ListPriceLists</a> response. </p><br>
    ///   - [`file_format(impl Into<String>)`](crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder::file_format) / [`set_file_format(Option<String>)`](crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder::set_file_format):<br>required: **true**<br><p>The format that you want to retrieve your Price List files in. The <code>FileFormat</code> can be obtained from the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_pricing_ListPriceLists.html">ListPriceLists</a> response. </p><br>
    /// - On success, responds with [`GetPriceListFileUrlOutput`](crate::operation::get_price_list_file_url::GetPriceListFileUrlOutput) with field(s):
    ///   - [`url(Option<String>)`](crate::operation::get_price_list_file_url::GetPriceListFileUrlOutput::url): <p>The URL to download your Price List file from. </p>
    /// - On failure, responds with [`SdkError<GetPriceListFileUrlError>`](crate::operation::get_price_list_file_url::GetPriceListFileUrlError)
    pub fn get_price_list_file_url(&self) -> crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder {
        crate::operation::get_price_list_file_url::builders::GetPriceListFileUrlFluentBuilder::new(self.handle.clone())
    }
}
