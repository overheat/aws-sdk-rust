// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSite`](crate::operation::update_site::builders::UpdateSiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::global_network_id) / [`set_global_network_id(Option<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::set_global_network_id):<br>required: **true**<br><p>The ID of the global network.</p><br>
    ///   - [`site_id(impl Into<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::site_id) / [`set_site_id(Option<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::set_site_id):<br>required: **true**<br><p>The ID of your site.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::set_description):<br>required: **false**<br><p>A description of your site.</p>  <p>Constraints: Maximum length of 256 characters.</p><br>
    ///   - [`location(Location)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::location) / [`set_location(Option<Location>)`](crate::operation::update_site::builders::UpdateSiteFluentBuilder::set_location):<br>required: **false**<br><p>The site location:</p>  <ul>   <li> <p> <code>Address</code>: The physical address of the site.</p> </li>   <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>   <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>  </ul><br>
    /// - On success, responds with [`UpdateSiteOutput`](crate::operation::update_site::UpdateSiteOutput) with field(s):
    ///   - [`site(Option<Site>)`](crate::operation::update_site::UpdateSiteOutput::site): <p>Information about the site.</p>
    /// - On failure, responds with [`SdkError<UpdateSiteError>`](crate::operation::update_site::UpdateSiteError)
    pub fn update_site(&self) -> crate::operation::update_site::builders::UpdateSiteFluentBuilder {
        crate::operation::update_site::builders::UpdateSiteFluentBuilder::new(self.handle.clone())
    }
}
