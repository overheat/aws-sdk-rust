// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `CmafSegmentLengthControl`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let cmafsegmentlengthcontrol = unimplemented!();
/// match cmafsegmentlengthcontrol {
///     CmafSegmentLengthControl::Exact => { /* ... */ },
///     CmafSegmentLengthControl::GopMultiple => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `cmafsegmentlengthcontrol` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `CmafSegmentLengthControl::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `CmafSegmentLengthControl::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `CmafSegmentLengthControl::NewFeature` is defined.
/// Specifically, when `cmafsegmentlengthcontrol` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `CmafSegmentLengthControl::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// Specify how you want MediaConvert to determine the segment length. Choose Exact to have the encoder use the exact length that you specify with the setting Segment length. This might result in extra I-frames. Choose Multiple of GOP to have the encoder round up the segment lengths to match the next GOP boundary.
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum CmafSegmentLengthControl {
    #[allow(missing_docs)] // documentation missing in model
    Exact,
    #[allow(missing_docs)] // documentation missing in model
    GopMultiple,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for CmafSegmentLengthControl {
    fn from(s: &str) -> Self {
        match s {
            "EXACT" => CmafSegmentLengthControl::Exact,
            "GOP_MULTIPLE" => CmafSegmentLengthControl::GopMultiple,
            other => CmafSegmentLengthControl::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for CmafSegmentLengthControl {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(CmafSegmentLengthControl::from(s))
    }
}
impl CmafSegmentLengthControl {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CmafSegmentLengthControl::Exact => "EXACT",
            CmafSegmentLengthControl::GopMultiple => "GOP_MULTIPLE",
            CmafSegmentLengthControl::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["EXACT", "GOP_MULTIPLE"]
    }
}
impl ::std::convert::AsRef<str> for CmafSegmentLengthControl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
