// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLoaderJobStatus`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_id(impl Into<String>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::load_id) / [`set_load_id(Option<String>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::set_load_id):<br>required: **true**<br><p>The load ID of the load job to get the status of.</p><br>
    ///   - [`details(bool)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::details) / [`set_details(Option<bool>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::set_details):<br>required: **false**<br><p>Flag indicating whether or not to include details beyond the overall status (<code>TRUE</code> or <code>FALSE</code>; the default is <code>FALSE</code>).</p><br>
    ///   - [`errors(bool)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::errors) / [`set_errors(Option<bool>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::set_errors):<br>required: **false**<br><p>Flag indicating whether or not to include a list of errors encountered (<code>TRUE</code> or <code>FALSE</code>; the default is <code>FALSE</code>).</p>  <p>The list of errors is paged. The <code>page</code> and <code>errorsPerPage</code> parameters allow you to page through all the errors.</p><br>
    ///   - [`page(i32)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::page) / [`set_page(Option<i32>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::set_page):<br>required: **false**<br><p>The error page number (a positive integer; the default is <code>1</code>). Only valid when the <code>errors</code> parameter is set to <code>TRUE</code>.</p><br>
    ///   - [`errors_per_page(i32)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::errors_per_page) / [`set_errors_per_page(Option<i32>)`](crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::set_errors_per_page):<br>required: **false**<br><p>The number of errors returned in each page (a positive integer; the default is <code>10</code>). Only valid when the <code>errors</code> parameter set to <code>TRUE</code>.</p><br>
    /// - On success, responds with [`GetLoaderJobStatusOutput`](crate::operation::get_loader_job_status::GetLoaderJobStatusOutput) with field(s):
    ///   - [`status(String)`](crate::operation::get_loader_job_status::GetLoaderJobStatusOutput::status): <p>The HTTP response code for the request.</p>
    ///   - [`payload(Document)`](crate::operation::get_loader_job_status::GetLoaderJobStatusOutput::payload): <p>Status information about the load job, in a layout that could look like this:</p>
    /// - On failure, responds with [`SdkError<GetLoaderJobStatusError>`](crate::operation::get_loader_job_status::GetLoaderJobStatusError)
    pub fn get_loader_job_status(&self) -> crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder {
        crate::operation::get_loader_job_status::builders::GetLoaderJobStatusFluentBuilder::new(self.handle.clone())
    }
}
