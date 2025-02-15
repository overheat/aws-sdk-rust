// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateLaunchProfileInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The description.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the IDs of the EC2 subnets where streaming sessions will be accessible from. These subnets must support the specified instance types. </p>
    pub ec2_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub launch_profile_protocol_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name for the launch profile.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A configuration for a streaming session.</p>
    pub stream_configuration: ::std::option::Option<crate::types::StreamConfigurationCreate>,
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub studio_component_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The studio ID. </p>
    pub studio_id: ::std::option::Option<::std::string::String>,
    /// <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreateLaunchProfileInput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Specifies the IDs of the EC2 subnets where streaming sessions will be accessible from. These subnets must support the specified instance types. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ec2_subnet_ids.is_none()`.
    pub fn ec2_subnet_ids(&self) -> &[::std::string::String] {
        self.ec2_subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.launch_profile_protocol_versions.is_none()`.
    pub fn launch_profile_protocol_versions(&self) -> &[::std::string::String] {
        self.launch_profile_protocol_versions.as_deref().unwrap_or_default()
    }
    /// <p>The name for the launch profile.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A configuration for a streaming session.</p>
    pub fn stream_configuration(&self) -> ::std::option::Option<&crate::types::StreamConfigurationCreate> {
        self.stream_configuration.as_ref()
    }
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.studio_component_ids.is_none()`.
    pub fn studio_component_ids(&self) -> &[::std::string::String] {
        self.studio_component_ids.as_deref().unwrap_or_default()
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(&self) -> ::std::option::Option<&str> {
        self.studio_id.as_deref()
    }
    /// <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for CreateLaunchProfileInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateLaunchProfileInput");
        formatter.field("client_token", &self.client_token);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("ec2_subnet_ids", &self.ec2_subnet_ids);
        formatter.field("launch_profile_protocol_versions", &self.launch_profile_protocol_versions);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("stream_configuration", &self.stream_configuration);
        formatter.field("studio_component_ids", &self.studio_component_ids);
        formatter.field("studio_id", &self.studio_id);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
impl CreateLaunchProfileInput {
    /// Creates a new builder-style object to manufacture [`CreateLaunchProfileInput`](crate::operation::create_launch_profile::CreateLaunchProfileInput).
    pub fn builder() -> crate::operation::create_launch_profile::builders::CreateLaunchProfileInputBuilder {
        crate::operation::create_launch_profile::builders::CreateLaunchProfileInputBuilder::default()
    }
}

/// A builder for [`CreateLaunchProfileInput`](crate::operation::create_launch_profile::CreateLaunchProfileInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateLaunchProfileInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) ec2_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) launch_profile_protocol_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) stream_configuration: ::std::option::Option<crate::types::StreamConfigurationCreate>,
    pub(crate) studio_component_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) studio_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CreateLaunchProfileInputBuilder {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>The description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `ec2_subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_ec2_subnet_ids`](Self::set_ec2_subnet_ids).
    ///
    /// <p>Specifies the IDs of the EC2 subnets where streaming sessions will be accessible from. These subnets must support the specified instance types. </p>
    pub fn ec2_subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ec2_subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.ec2_subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the IDs of the EC2 subnets where streaming sessions will be accessible from. These subnets must support the specified instance types. </p>
    pub fn set_ec2_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.ec2_subnet_ids = input;
        self
    }
    /// <p>Specifies the IDs of the EC2 subnets where streaming sessions will be accessible from. These subnets must support the specified instance types. </p>
    pub fn get_ec2_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.ec2_subnet_ids
    }
    /// Appends an item to `launch_profile_protocol_versions`.
    ///
    /// To override the contents of this collection use [`set_launch_profile_protocol_versions`](Self::set_launch_profile_protocol_versions).
    ///
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub fn launch_profile_protocol_versions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.launch_profile_protocol_versions.unwrap_or_default();
        v.push(input.into());
        self.launch_profile_protocol_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub fn set_launch_profile_protocol_versions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.launch_profile_protocol_versions = input;
        self
    }
    /// <p>The version number of the protocol that is used by the launch profile. The only valid version is "2021-03-31".</p>
    pub fn get_launch_profile_protocol_versions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.launch_profile_protocol_versions
    }
    /// <p>The name for the launch profile.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name for the launch profile.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name for the launch profile.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>A configuration for a streaming session.</p>
    /// This field is required.
    pub fn stream_configuration(mut self, input: crate::types::StreamConfigurationCreate) -> Self {
        self.stream_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>A configuration for a streaming session.</p>
    pub fn set_stream_configuration(mut self, input: ::std::option::Option<crate::types::StreamConfigurationCreate>) -> Self {
        self.stream_configuration = input;
        self
    }
    /// <p>A configuration for a streaming session.</p>
    pub fn get_stream_configuration(&self) -> &::std::option::Option<crate::types::StreamConfigurationCreate> {
        &self.stream_configuration
    }
    /// Appends an item to `studio_component_ids`.
    ///
    /// To override the contents of this collection use [`set_studio_component_ids`](Self::set_studio_component_ids).
    ///
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub fn studio_component_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.studio_component_ids.unwrap_or_default();
        v.push(input.into());
        self.studio_component_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub fn set_studio_component_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.studio_component_ids = input;
        self
    }
    /// <p>Unique identifiers for a collection of studio components that can be used with this launch profile.</p>
    pub fn get_studio_component_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.studio_component_ids
    }
    /// <p>The studio ID. </p>
    /// This field is required.
    pub fn studio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.studio_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The studio ID. </p>
    pub fn set_studio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.studio_id = input;
        self
    }
    /// <p>The studio ID. </p>
    pub fn get_studio_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.studio_id
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateLaunchProfileInput`](crate::operation::create_launch_profile::CreateLaunchProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_launch_profile::CreateLaunchProfileInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::create_launch_profile::CreateLaunchProfileInput {
            client_token: self.client_token,
            description: self.description,
            ec2_subnet_ids: self.ec2_subnet_ids,
            launch_profile_protocol_versions: self.launch_profile_protocol_versions,
            name: self.name,
            stream_configuration: self.stream_configuration,
            studio_component_ids: self.studio_component_ids,
            studio_id: self.studio_id,
            tags: self.tags,
        })
    }
}
impl ::std::fmt::Debug for CreateLaunchProfileInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateLaunchProfileInputBuilder");
        formatter.field("client_token", &self.client_token);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("ec2_subnet_ids", &self.ec2_subnet_ids);
        formatter.field("launch_profile_protocol_versions", &self.launch_profile_protocol_versions);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("stream_configuration", &self.stream_configuration);
        formatter.field("studio_component_ids", &self.studio_component_ids);
        formatter.field("studio_id", &self.studio_id);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
