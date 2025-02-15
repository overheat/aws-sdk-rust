// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCostAllocationTagsStatus`](crate::operation::update_cost_allocation_tags_status::builders::UpdateCostAllocationTagsStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cost_allocation_tags_status(CostAllocationTagStatusEntry)`](crate::operation::update_cost_allocation_tags_status::builders::UpdateCostAllocationTagsStatusFluentBuilder::cost_allocation_tags_status) / [`set_cost_allocation_tags_status(Option<Vec::<CostAllocationTagStatusEntry>>)`](crate::operation::update_cost_allocation_tags_status::builders::UpdateCostAllocationTagsStatusFluentBuilder::set_cost_allocation_tags_status):<br>required: **true**<br><p>The list of <code>CostAllocationTagStatusEntry</code> objects that are used to update cost allocation tags status for this request. </p><br>
    /// - On success, responds with [`UpdateCostAllocationTagsStatusOutput`](crate::operation::update_cost_allocation_tags_status::UpdateCostAllocationTagsStatusOutput) with field(s):
    ///   - [`errors(Option<Vec::<UpdateCostAllocationTagsStatusError>>)`](crate::operation::update_cost_allocation_tags_status::UpdateCostAllocationTagsStatusOutput::errors): <p>A list of <code>UpdateCostAllocationTagsStatusError</code> objects with error details about each cost allocation tag that can't be updated. If there's no failure, an empty array returns. </p>
    /// - On failure, responds with [`SdkError<UpdateCostAllocationTagsStatusError>`](crate::operation::update_cost_allocation_tags_status::UpdateCostAllocationTagsStatusError)
    pub fn update_cost_allocation_tags_status(
        &self,
    ) -> crate::operation::update_cost_allocation_tags_status::builders::UpdateCostAllocationTagsStatusFluentBuilder {
        crate::operation::update_cost_allocation_tags_status::builders::UpdateCostAllocationTagsStatusFluentBuilder::new(self.handle.clone())
    }
}
