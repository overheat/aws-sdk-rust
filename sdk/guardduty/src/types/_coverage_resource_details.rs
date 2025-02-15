// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the resource for each individual EKS cluster.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CoverageResourceDetails {
    /// <p>EKS cluster details involved in the coverage statistics.</p>
    pub eks_cluster_details: ::std::option::Option<crate::types::CoverageEksClusterDetails>,
    /// <p>The type of Amazon Web Services resource.</p>
    pub resource_type: ::std::option::Option<crate::types::ResourceType>,
}
impl CoverageResourceDetails {
    /// <p>EKS cluster details involved in the coverage statistics.</p>
    pub fn eks_cluster_details(&self) -> ::std::option::Option<&crate::types::CoverageEksClusterDetails> {
        self.eks_cluster_details.as_ref()
    }
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
}
impl CoverageResourceDetails {
    /// Creates a new builder-style object to manufacture [`CoverageResourceDetails`](crate::types::CoverageResourceDetails).
    pub fn builder() -> crate::types::builders::CoverageResourceDetailsBuilder {
        crate::types::builders::CoverageResourceDetailsBuilder::default()
    }
}

/// A builder for [`CoverageResourceDetails`](crate::types::CoverageResourceDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CoverageResourceDetailsBuilder {
    pub(crate) eks_cluster_details: ::std::option::Option<crate::types::CoverageEksClusterDetails>,
    pub(crate) resource_type: ::std::option::Option<crate::types::ResourceType>,
}
impl CoverageResourceDetailsBuilder {
    /// <p>EKS cluster details involved in the coverage statistics.</p>
    pub fn eks_cluster_details(mut self, input: crate::types::CoverageEksClusterDetails) -> Self {
        self.eks_cluster_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>EKS cluster details involved in the coverage statistics.</p>
    pub fn set_eks_cluster_details(mut self, input: ::std::option::Option<crate::types::CoverageEksClusterDetails>) -> Self {
        self.eks_cluster_details = input;
        self
    }
    /// <p>EKS cluster details involved in the coverage statistics.</p>
    pub fn get_eks_cluster_details(&self) -> &::std::option::Option<crate::types::CoverageEksClusterDetails> {
        &self.eks_cluster_details
    }
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::ResourceType>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::ResourceType> {
        &self.resource_type
    }
    /// Consumes the builder and constructs a [`CoverageResourceDetails`](crate::types::CoverageResourceDetails).
    pub fn build(self) -> crate::types::CoverageResourceDetails {
        crate::types::CoverageResourceDetails {
            eks_cluster_details: self.eks_cluster_details,
            resource_type: self.resource_type,
        }
    }
}
