// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains asset property information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssetProperty {
    /// <p>The ID of the asset property.</p>
    pub id: ::std::string::String,
    /// <p>The name of the property.</p>
    pub name: ::std::string::String,
    /// <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub alias: ::std::option::Option<::std::string::String>,
    /// <p>The asset property's notification topic and state. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_UpdateAssetProperty.html">UpdateAssetProperty</a>.</p>
    pub notification: ::std::option::Option<crate::types::PropertyNotification>,
    /// <p>The data type of the asset property.</p>
    pub data_type: crate::types::PropertyDataType,
    /// <p>The data type of the structure for this property. This parameter exists on properties that have the <code>STRUCT</code> data type.</p>
    pub data_type_spec: ::std::option::Option<::std::string::String>,
    /// <p>The unit (such as <code>Newtons</code> or <code>RPM</code>) of the asset property.</p>
    pub unit: ::std::option::Option<::std::string::String>,
}
impl AssetProperty {
    /// <p>The ID of the asset property.</p>
    pub fn id(&self) -> &str {
        use std::ops::Deref;
        self.id.deref()
    }
    /// <p>The name of the property.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn alias(&self) -> ::std::option::Option<&str> {
        self.alias.as_deref()
    }
    /// <p>The asset property's notification topic and state. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_UpdateAssetProperty.html">UpdateAssetProperty</a>.</p>
    pub fn notification(&self) -> ::std::option::Option<&crate::types::PropertyNotification> {
        self.notification.as_ref()
    }
    /// <p>The data type of the asset property.</p>
    pub fn data_type(&self) -> &crate::types::PropertyDataType {
        &self.data_type
    }
    /// <p>The data type of the structure for this property. This parameter exists on properties that have the <code>STRUCT</code> data type.</p>
    pub fn data_type_spec(&self) -> ::std::option::Option<&str> {
        self.data_type_spec.as_deref()
    }
    /// <p>The unit (such as <code>Newtons</code> or <code>RPM</code>) of the asset property.</p>
    pub fn unit(&self) -> ::std::option::Option<&str> {
        self.unit.as_deref()
    }
}
impl AssetProperty {
    /// Creates a new builder-style object to manufacture [`AssetProperty`](crate::types::AssetProperty).
    pub fn builder() -> crate::types::builders::AssetPropertyBuilder {
        crate::types::builders::AssetPropertyBuilder::default()
    }
}

/// A builder for [`AssetProperty`](crate::types::AssetProperty).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssetPropertyBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) alias: ::std::option::Option<::std::string::String>,
    pub(crate) notification: ::std::option::Option<crate::types::PropertyNotification>,
    pub(crate) data_type: ::std::option::Option<crate::types::PropertyDataType>,
    pub(crate) data_type_spec: ::std::option::Option<::std::string::String>,
    pub(crate) unit: ::std::option::Option<::std::string::String>,
}
impl AssetPropertyBuilder {
    /// <p>The ID of the asset property.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the asset property.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the asset property.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The name of the property.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the property.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the property.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.alias = input;
        self
    }
    /// <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_alias(&self) -> &::std::option::Option<::std::string::String> {
        &self.alias
    }
    /// <p>The asset property's notification topic and state. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_UpdateAssetProperty.html">UpdateAssetProperty</a>.</p>
    pub fn notification(mut self, input: crate::types::PropertyNotification) -> Self {
        self.notification = ::std::option::Option::Some(input);
        self
    }
    /// <p>The asset property's notification topic and state. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_UpdateAssetProperty.html">UpdateAssetProperty</a>.</p>
    pub fn set_notification(mut self, input: ::std::option::Option<crate::types::PropertyNotification>) -> Self {
        self.notification = input;
        self
    }
    /// <p>The asset property's notification topic and state. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_UpdateAssetProperty.html">UpdateAssetProperty</a>.</p>
    pub fn get_notification(&self) -> &::std::option::Option<crate::types::PropertyNotification> {
        &self.notification
    }
    /// <p>The data type of the asset property.</p>
    /// This field is required.
    pub fn data_type(mut self, input: crate::types::PropertyDataType) -> Self {
        self.data_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data type of the asset property.</p>
    pub fn set_data_type(mut self, input: ::std::option::Option<crate::types::PropertyDataType>) -> Self {
        self.data_type = input;
        self
    }
    /// <p>The data type of the asset property.</p>
    pub fn get_data_type(&self) -> &::std::option::Option<crate::types::PropertyDataType> {
        &self.data_type
    }
    /// <p>The data type of the structure for this property. This parameter exists on properties that have the <code>STRUCT</code> data type.</p>
    pub fn data_type_spec(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_type_spec = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The data type of the structure for this property. This parameter exists on properties that have the <code>STRUCT</code> data type.</p>
    pub fn set_data_type_spec(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_type_spec = input;
        self
    }
    /// <p>The data type of the structure for this property. This parameter exists on properties that have the <code>STRUCT</code> data type.</p>
    pub fn get_data_type_spec(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_type_spec
    }
    /// <p>The unit (such as <code>Newtons</code> or <code>RPM</code>) of the asset property.</p>
    pub fn unit(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.unit = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unit (such as <code>Newtons</code> or <code>RPM</code>) of the asset property.</p>
    pub fn set_unit(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.unit = input;
        self
    }
    /// <p>The unit (such as <code>Newtons</code> or <code>RPM</code>) of the asset property.</p>
    pub fn get_unit(&self) -> &::std::option::Option<::std::string::String> {
        &self.unit
    }
    /// Consumes the builder and constructs a [`AssetProperty`](crate::types::AssetProperty).
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](crate::types::builders::AssetPropertyBuilder::id)
    /// - [`name`](crate::types::builders::AssetPropertyBuilder::name)
    /// - [`data_type`](crate::types::builders::AssetPropertyBuilder::data_type)
    pub fn build(self) -> ::std::result::Result<crate::types::AssetProperty, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AssetProperty {
            id: self.id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "id",
                    "id was not specified but it is required when building AssetProperty",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building AssetProperty",
                )
            })?,
            alias: self.alias,
            notification: self.notification,
            data_type: self.data_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data_type",
                    "data_type was not specified but it is required when building AssetProperty",
                )
            })?,
            data_type_spec: self.data_type_spec,
            unit: self.unit,
        })
    }
}
