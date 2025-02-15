// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutPermissionsBoundaryToPermissionSet`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_arn(impl Into<String>)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::instance_arn) / [`set_instance_arn(Option<String>)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::set_instance_arn):<br>required: **true**<br><p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p><br>
    ///   - [`permission_set_arn(impl Into<String>)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::permission_set_arn) / [`set_permission_set_arn(Option<String>)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::set_permission_set_arn):<br>required: **true**<br><p>The ARN of the <code>PermissionSet</code>.</p><br>
    ///   - [`permissions_boundary(PermissionsBoundary)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::permissions_boundary) / [`set_permissions_boundary(Option<PermissionsBoundary>)`](crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::set_permissions_boundary):<br>required: **true**<br><p>The permissions boundary that you want to attach to a <code>PermissionSet</code>.</p><br>
    /// - On success, responds with [`PutPermissionsBoundaryToPermissionSetOutput`](crate::operation::put_permissions_boundary_to_permission_set::PutPermissionsBoundaryToPermissionSetOutput)
    /// - On failure, responds with [`SdkError<PutPermissionsBoundaryToPermissionSetError>`](crate::operation::put_permissions_boundary_to_permission_set::PutPermissionsBoundaryToPermissionSetError)
    pub fn put_permissions_boundary_to_permission_set(
        &self,
    ) -> crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder {
        crate::operation::put_permissions_boundary_to_permission_set::builders::PutPermissionsBoundaryToPermissionSetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
