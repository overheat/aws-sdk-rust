// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRuleGroup`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rule_group_name(impl Into<String>)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::rule_group_name) / [`set_rule_group_name(Option<String>)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::set_rule_group_name):<br>required: **false**<br><p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p><br>
    ///   - [`rule_group_arn(impl Into<String>)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::rule_group_arn) / [`set_rule_group_arn(Option<String>)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::set_rule_group_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the rule group.</p>  <p>You must specify the ARN or the name, and you can specify both. </p><br>
    ///   - [`r#type(RuleGroupType)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::type) / [`set_type(Option<RuleGroupType>)`](crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::set_type):<br>required: **false**<br><p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules. </p> <note>   <p>This setting is required for requests that do not include the <code>RuleGroupARN</code>.</p>  </note><br>
    /// - On success, responds with [`DeleteRuleGroupOutput`](crate::operation::delete_rule_group::DeleteRuleGroupOutput) with field(s):
    ///   - [`rule_group_response(Option<RuleGroupResponse>)`](crate::operation::delete_rule_group::DeleteRuleGroupOutput::rule_group_response): <p>The high-level properties of a rule group. This, along with the <code>RuleGroup</code>, define the rule group. You can retrieve all objects for a rule group by calling <code>DescribeRuleGroup</code>. </p>
    /// - On failure, responds with [`SdkError<DeleteRuleGroupError>`](crate::operation::delete_rule_group::DeleteRuleGroupError)
    pub fn delete_rule_group(&self) -> crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder {
        crate::operation::delete_rule_group::builders::DeleteRuleGroupFluentBuilder::new(self.handle.clone())
    }
}
