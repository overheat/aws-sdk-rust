// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PurchaseOfferingOutput {
    /// A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.
    pub reservation: ::std::option::Option<crate::types::Reservation>,
    _request_id: Option<String>,
}
impl PurchaseOfferingOutput {
    /// A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.
    pub fn reservation(&self) -> ::std::option::Option<&crate::types::Reservation> {
        self.reservation.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for PurchaseOfferingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PurchaseOfferingOutput {
    /// Creates a new builder-style object to manufacture [`PurchaseOfferingOutput`](crate::operation::purchase_offering::PurchaseOfferingOutput).
    pub fn builder() -> crate::operation::purchase_offering::builders::PurchaseOfferingOutputBuilder {
        crate::operation::purchase_offering::builders::PurchaseOfferingOutputBuilder::default()
    }
}

/// A builder for [`PurchaseOfferingOutput`](crate::operation::purchase_offering::PurchaseOfferingOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PurchaseOfferingOutputBuilder {
    pub(crate) reservation: ::std::option::Option<crate::types::Reservation>,
    _request_id: Option<String>,
}
impl PurchaseOfferingOutputBuilder {
    /// A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.
    pub fn reservation(mut self, input: crate::types::Reservation) -> Self {
        self.reservation = ::std::option::Option::Some(input);
        self
    }
    /// A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.
    pub fn set_reservation(mut self, input: ::std::option::Option<crate::types::Reservation>) -> Self {
        self.reservation = input;
        self
    }
    /// A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.
    pub fn get_reservation(&self) -> &::std::option::Option<crate::types::Reservation> {
        &self.reservation
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PurchaseOfferingOutput`](crate::operation::purchase_offering::PurchaseOfferingOutput).
    pub fn build(self) -> crate::operation::purchase_offering::PurchaseOfferingOutput {
        crate::operation::purchase_offering::PurchaseOfferingOutput {
            reservation: self.reservation,
            _request_id: self._request_id,
        }
    }
}
