// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRecommender`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::set_name):<br>required: **true**<br><p>The name of the recommender.</p><br>
    ///   - [`dataset_group_arn(impl Into<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::dataset_group_arn) / [`set_dataset_group_arn(Option<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::set_dataset_group_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the destination domain dataset group for the recommender.</p><br>
    ///   - [`recipe_arn(impl Into<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::recipe_arn) / [`set_recipe_arn(Option<String>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::set_recipe_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the recipe that the recommender will use. For a recommender, a recipe is a Domain dataset group use case. Only Domain dataset group use cases can be used to create a recommender. For information about use cases see <a href="https://docs.aws.amazon.com/personalize/latest/dg/domain-use-cases.html">Choosing recommender use cases</a>. </p><br>
    ///   - [`recommender_config(RecommenderConfig)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::recommender_config) / [`set_recommender_config(Option<RecommenderConfig>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::set_recommender_config):<br>required: **false**<br><p>The configuration details of the recommender.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::set_tags):<br>required: **false**<br><p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the recommender.</p><br>
    /// - On success, responds with [`CreateRecommenderOutput`](crate::operation::create_recommender::CreateRecommenderOutput) with field(s):
    ///   - [`recommender_arn(Option<String>)`](crate::operation::create_recommender::CreateRecommenderOutput::recommender_arn): <p>The Amazon Resource Name (ARN) of the recommender.</p>
    /// - On failure, responds with [`SdkError<CreateRecommenderError>`](crate::operation::create_recommender::CreateRecommenderError)
    pub fn create_recommender(&self) -> crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder {
        crate::operation::create_recommender::builders::CreateRecommenderFluentBuilder::new(self.handle.clone())
    }
}
