// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDBClusterParameterGroup`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_cluster_parameter_group_name(impl Into<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::db_cluster_parameter_group_name) / [`set_db_cluster_parameter_group_name(Option<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::set_db_cluster_parameter_group_name):<br>required: **true**<br><p>The name of the cluster parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must not match the name of an existing <code>DBClusterParameterGroup</code>.</p> </li>  </ul> <note>   <p>This value is stored as a lowercase string.</p>  </note><br>
    ///   - [`db_parameter_group_family(impl Into<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::db_parameter_group_family) / [`set_db_parameter_group_family(Option<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::set_db_parameter_group_family):<br>required: **true**<br><p>The cluster parameter group family name.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::set_description):<br>required: **true**<br><p>The description for the cluster parameter group.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::set_tags):<br>required: **false**<br><p>The tags to be assigned to the cluster parameter group.</p><br>
    /// - On success, responds with [`CreateDbClusterParameterGroupOutput`](crate::operation::create_db_cluster_parameter_group::CreateDbClusterParameterGroupOutput) with field(s):
    ///   - [`db_cluster_parameter_group(Option<DbClusterParameterGroup>)`](crate::operation::create_db_cluster_parameter_group::CreateDbClusterParameterGroupOutput::db_cluster_parameter_group): <p>Detailed information about a cluster parameter group. </p>
    /// - On failure, responds with [`SdkError<CreateDBClusterParameterGroupError>`](crate::operation::create_db_cluster_parameter_group::CreateDBClusterParameterGroupError)
    pub fn create_db_cluster_parameter_group(
        &self,
    ) -> crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder {
        crate::operation::create_db_cluster_parameter_group::builders::CreateDBClusterParameterGroupFluentBuilder::new(self.handle.clone())
    }
}
