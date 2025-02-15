// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_destination_output_correct_errors(
    mut builder: crate::operation::create_destination::builders::CreateDestinationOutputBuilder,
) -> crate::operation::create_destination::builders::CreateDestinationOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::DestinationState>().ok()
    }
    builder
}

pub(crate) fn create_site_output_correct_errors(
    mut builder: crate::operation::create_site::builders::CreateSiteOutputBuilder,
) -> crate::operation::create_site::builders::CreateSiteOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn create_worker_output_correct_errors(
    mut builder: crate::operation::create_worker::builders::CreateWorkerOutputBuilder,
) -> crate::operation::create_worker::builders::CreateWorkerOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    builder
}

pub(crate) fn create_worker_fleet_output_correct_errors(
    mut builder: crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder,
) -> crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_destination_output_correct_errors(
    mut builder: crate::operation::get_destination::builders::GetDestinationOutputBuilder,
) -> crate::operation::get_destination::builders::GetDestinationOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::DestinationState>().ok()
    }
    builder
}

pub(crate) fn get_site_output_correct_errors(
    mut builder: crate::operation::get_site::builders::GetSiteOutputBuilder,
) -> crate::operation::get_site::builders::GetSiteOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.country_code.is_none() {
        builder.country_code = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_worker_output_correct_errors(
    mut builder: crate::operation::get_worker::builders::GetWorkerOutputBuilder,
) -> crate::operation::get_worker::builders::GetWorkerOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.fleet.is_none() {
        builder.fleet = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn get_worker_fleet_output_correct_errors(
    mut builder: crate::operation::get_worker_fleet::builders::GetWorkerFleetOutputBuilder,
) -> crate::operation::get_worker_fleet::builders::GetWorkerFleetOutputBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn update_destination_output_correct_errors(
    mut builder: crate::operation::update_destination::builders::UpdateDestinationOutputBuilder,
) -> crate::operation::update_destination::builders::UpdateDestinationOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::DestinationState>().ok()
    }
    builder
}

pub(crate) fn update_site_output_correct_errors(
    mut builder: crate::operation::update_site::builders::UpdateSiteOutputBuilder,
) -> crate::operation::update_site::builders::UpdateSiteOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn update_worker_output_correct_errors(
    mut builder: crate::operation::update_worker::builders::UpdateWorkerOutputBuilder,
) -> crate::operation::update_worker::builders::UpdateWorkerOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.fleet.is_none() {
        builder.fleet = Some(Default::default())
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn update_worker_fleet_output_correct_errors(
    mut builder: crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder,
) -> crate::operation::update_worker_fleet::builders::UpdateWorkerFleetOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn vendor_properties_correct_errors(
    mut builder: crate::types::builders::VendorPropertiesBuilder,
) -> crate::types::builders::VendorPropertiesBuilder {
    if builder.vendor_worker_id.is_none() {
        builder.vendor_worker_id = Some(Default::default())
    }
    builder
}

pub(crate) fn cartesian_coordinates_correct_errors(
    mut builder: crate::types::builders::CartesianCoordinatesBuilder,
) -> crate::types::builders::CartesianCoordinatesBuilder {
    if builder.x.is_none() {
        builder.x = Some(Default::default())
    }
    if builder.y.is_none() {
        builder.y = Some(Default::default())
    }
    builder
}

pub(crate) fn destination_correct_errors(mut builder: crate::types::builders::DestinationBuilder) -> crate::types::builders::DestinationBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::DestinationState>().ok()
    }
    builder
}

pub(crate) fn site_correct_errors(mut builder: crate::types::builders::SiteBuilder) -> crate::types::builders::SiteBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.country_code.is_none() {
        builder.country_code = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn worker_correct_errors(mut builder: crate::types::builders::WorkerBuilder) -> crate::types::builders::WorkerBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.fleet.is_none() {
        builder.fleet = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    builder
}

pub(crate) fn worker_fleet_correct_errors(mut builder: crate::types::builders::WorkerFleetBuilder) -> crate::types::builders::WorkerFleetBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.site.is_none() {
        builder.site = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.updated_at.is_none() {
        builder.updated_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}
