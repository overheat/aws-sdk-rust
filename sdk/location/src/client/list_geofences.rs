// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGeofences`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`collection_name(impl Into<String>)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::collection_name) / [`set_collection_name(Option<String>)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::set_collection_name):<br>required: **true**<br><p>The name of the geofence collection storing the list of geofences.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token specifying which page of results to return in the response. If no token is provided, the default page is the first page. </p>  <p>Default value: <code>null</code> </p><br>
    ///   - [`max_results(i32)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional limit for the number of geofences returned in a single call. </p>  <p>Default value: <code>100</code> </p><br>
    /// - On success, responds with [`ListGeofencesOutput`](crate::operation::list_geofences::ListGeofencesOutput) with field(s):
    ///   - [`entries(Vec::<ListGeofenceResponseEntry>)`](crate::operation::list_geofences::ListGeofencesOutput::entries): <p>Contains a list of geofences stored in the geofence collection.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_geofences::ListGeofencesOutput::next_token): <p>A pagination token indicating there are additional pages available. You can use the token in a following request to fetch the next set of results. </p>
    /// - On failure, responds with [`SdkError<ListGeofencesError>`](crate::operation::list_geofences::ListGeofencesError)
    pub fn list_geofences(&self) -> crate::operation::list_geofences::builders::ListGeofencesFluentBuilder {
        crate::operation::list_geofences::builders::ListGeofencesFluentBuilder::new(self.handle.clone())
    }
}
