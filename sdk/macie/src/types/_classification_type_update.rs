// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>(Discontinued) The classification type that Amazon Macie Classic applies to the associated S3 resources. At least one of the classification types (<code>oneTime</code> or <code>continuous</code>) must be specified. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClassificationTypeUpdate {
    /// <p>(Discontinued) A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    pub one_time: ::std::option::Option<crate::types::S3OneTimeClassificationType>,
    /// <p>(Discontinued) A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie Classic begins performing continuous classification after a bucket is successfully associated with Macie Classic. </p>
    pub continuous: ::std::option::Option<crate::types::S3ContinuousClassificationType>,
}
impl ClassificationTypeUpdate {
    /// <p>(Discontinued) A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    pub fn one_time(&self) -> ::std::option::Option<&crate::types::S3OneTimeClassificationType> {
        self.one_time.as_ref()
    }
    /// <p>(Discontinued) A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie Classic begins performing continuous classification after a bucket is successfully associated with Macie Classic. </p>
    pub fn continuous(&self) -> ::std::option::Option<&crate::types::S3ContinuousClassificationType> {
        self.continuous.as_ref()
    }
}
impl ClassificationTypeUpdate {
    /// Creates a new builder-style object to manufacture [`ClassificationTypeUpdate`](crate::types::ClassificationTypeUpdate).
    pub fn builder() -> crate::types::builders::ClassificationTypeUpdateBuilder {
        crate::types::builders::ClassificationTypeUpdateBuilder::default()
    }
}

/// A builder for [`ClassificationTypeUpdate`](crate::types::ClassificationTypeUpdate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ClassificationTypeUpdateBuilder {
    pub(crate) one_time: ::std::option::Option<crate::types::S3OneTimeClassificationType>,
    pub(crate) continuous: ::std::option::Option<crate::types::S3ContinuousClassificationType>,
}
impl ClassificationTypeUpdateBuilder {
    /// <p>(Discontinued) A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    pub fn one_time(mut self, input: crate::types::S3OneTimeClassificationType) -> Self {
        self.one_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Discontinued) A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    pub fn set_one_time(mut self, input: ::std::option::Option<crate::types::S3OneTimeClassificationType>) -> Self {
        self.one_time = input;
        self
    }
    /// <p>(Discontinued) A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    pub fn get_one_time(&self) -> &::std::option::Option<crate::types::S3OneTimeClassificationType> {
        &self.one_time
    }
    /// <p>(Discontinued) A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie Classic begins performing continuous classification after a bucket is successfully associated with Macie Classic. </p>
    pub fn continuous(mut self, input: crate::types::S3ContinuousClassificationType) -> Self {
        self.continuous = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Discontinued) A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie Classic begins performing continuous classification after a bucket is successfully associated with Macie Classic. </p>
    pub fn set_continuous(mut self, input: ::std::option::Option<crate::types::S3ContinuousClassificationType>) -> Self {
        self.continuous = input;
        self
    }
    /// <p>(Discontinued) A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie Classic begins performing continuous classification after a bucket is successfully associated with Macie Classic. </p>
    pub fn get_continuous(&self) -> &::std::option::Option<crate::types::S3ContinuousClassificationType> {
        &self.continuous
    }
    /// Consumes the builder and constructs a [`ClassificationTypeUpdate`](crate::types::ClassificationTypeUpdate).
    pub fn build(self) -> crate::types::ClassificationTypeUpdate {
        crate::types::ClassificationTypeUpdate {
            one_time: self.one_time,
            continuous: self.continuous,
        }
    }
}
