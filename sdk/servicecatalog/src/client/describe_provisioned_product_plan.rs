// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeProvisionedProductPlan`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::accept_language) / [`set_accept_language(Option<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::set_accept_language):<br>required: **false**<br><p>The language code.</p>  <ul>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul><br>
    ///   - [`plan_id(impl Into<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::plan_id) / [`set_plan_id(Option<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::set_plan_id):<br>required: **true**<br><p>The plan identifier.</p><br>
    ///   - [`page_size(i32)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::page_size) / [`set_page_size(Option<i32>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::set_page_size):<br>required: **false**<br><p>The maximum number of items to return with this call.</p><br>
    ///   - [`page_token(impl Into<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::page_token) / [`set_page_token(Option<String>)`](crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::set_page_token):<br>required: **false**<br><p>The page token for the next set of results. To retrieve the first set of results, use null.</p><br>
    /// - On success, responds with [`DescribeProvisionedProductPlanOutput`](crate::operation::describe_provisioned_product_plan::DescribeProvisionedProductPlanOutput) with field(s):
    ///   - [`provisioned_product_plan_details(Option<ProvisionedProductPlanDetails>)`](crate::operation::describe_provisioned_product_plan::DescribeProvisionedProductPlanOutput::provisioned_product_plan_details): <p>Information about the plan.</p>
    ///   - [`resource_changes(Option<Vec::<ResourceChange>>)`](crate::operation::describe_provisioned_product_plan::DescribeProvisionedProductPlanOutput::resource_changes): <p>Information about the resource changes that will occur when the plan is executed.</p>
    ///   - [`next_page_token(Option<String>)`](crate::operation::describe_provisioned_product_plan::DescribeProvisionedProductPlanOutput::next_page_token): <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
    /// - On failure, responds with [`SdkError<DescribeProvisionedProductPlanError>`](crate::operation::describe_provisioned_product_plan::DescribeProvisionedProductPlanError)
    pub fn describe_provisioned_product_plan(
        &self,
    ) -> crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder {
        crate::operation::describe_provisioned_product_plan::builders::DescribeProvisionedProductPlanFluentBuilder::new(self.handle.clone())
    }
}
