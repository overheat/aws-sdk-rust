// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateOpsItemRelatedItem`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ops_item_id(impl Into<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::ops_item_id) / [`set_ops_item_id(Option<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::set_ops_item_id):<br>required: **true**<br><p>The ID of the OpsItem to which you want to associate a resource as a related item.</p><br>
    ///   - [`association_type(impl Into<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::association_type) / [`set_association_type(Option<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::set_association_type):<br>required: **true**<br><p>The type of association that you want to create between an OpsItem and a resource. OpsCenter supports <code>IsParentOf</code> and <code>RelatesTo</code> association types.</p><br>
    ///   - [`resource_type(impl Into<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::resource_type) / [`set_resource_type(Option<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::set_resource_type):<br>required: **true**<br><p>The type of resource that you want to associate with an OpsItem. OpsCenter supports the following types:</p>  <p> <code>AWS::SSMIncidents::IncidentRecord</code>: an Incident Manager incident. </p>  <p> <code>AWS::SSM::Document</code>: a Systems Manager (SSM) document.</p><br>
    ///   - [`resource_uri(impl Into<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::resource_uri) / [`set_resource_uri(Option<String>)`](crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::set_resource_uri):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Amazon Web Services resource that you want to associate with the OpsItem.</p><br>
    /// - On success, responds with [`AssociateOpsItemRelatedItemOutput`](crate::operation::associate_ops_item_related_item::AssociateOpsItemRelatedItemOutput) with field(s):
    ///   - [`association_id(Option<String>)`](crate::operation::associate_ops_item_related_item::AssociateOpsItemRelatedItemOutput::association_id): <p>The association ID.</p>
    /// - On failure, responds with [`SdkError<AssociateOpsItemRelatedItemError>`](crate::operation::associate_ops_item_related_item::AssociateOpsItemRelatedItemError)
    pub fn associate_ops_item_related_item(
        &self,
    ) -> crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder {
        crate::operation::associate_ops_item_related_item::builders::AssociateOpsItemRelatedItemFluentBuilder::new(self.handle.clone())
    }
}
