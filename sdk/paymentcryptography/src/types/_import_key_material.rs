// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Parameter information for key material import.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum ImportKeyMaterial {
    /// <p>Parameter information for root public key certificate import.</p>
    RootCertificatePublicKey(crate::types::RootCertificatePublicKey),
    /// <p>Parameter information for key material import using TR-31 standard.</p>
    Tr31KeyBlock(crate::types::ImportTr31KeyBlock),
    /// <p>Parameter information for key material import using TR-34 standard.</p>
    Tr34KeyBlock(crate::types::ImportTr34KeyBlock),
    /// <p>Parameter information for trusted public key certificate import.</p>
    TrustedCertificatePublicKey(crate::types::TrustedCertificatePublicKey),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl ImportKeyMaterial {
    /// Tries to convert the enum instance into [`RootCertificatePublicKey`](crate::types::ImportKeyMaterial::RootCertificatePublicKey), extracting the inner [`RootCertificatePublicKey`](crate::types::RootCertificatePublicKey).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_root_certificate_public_key(&self) -> ::std::result::Result<&crate::types::RootCertificatePublicKey, &Self> {
        if let ImportKeyMaterial::RootCertificatePublicKey(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`RootCertificatePublicKey`](crate::types::ImportKeyMaterial::RootCertificatePublicKey).
    pub fn is_root_certificate_public_key(&self) -> bool {
        self.as_root_certificate_public_key().is_ok()
    }
    /// Tries to convert the enum instance into [`Tr31KeyBlock`](crate::types::ImportKeyMaterial::Tr31KeyBlock), extracting the inner [`ImportTr31KeyBlock`](crate::types::ImportTr31KeyBlock).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_tr31_key_block(&self) -> ::std::result::Result<&crate::types::ImportTr31KeyBlock, &Self> {
        if let ImportKeyMaterial::Tr31KeyBlock(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Tr31KeyBlock`](crate::types::ImportKeyMaterial::Tr31KeyBlock).
    pub fn is_tr31_key_block(&self) -> bool {
        self.as_tr31_key_block().is_ok()
    }
    /// Tries to convert the enum instance into [`Tr34KeyBlock`](crate::types::ImportKeyMaterial::Tr34KeyBlock), extracting the inner [`ImportTr34KeyBlock`](crate::types::ImportTr34KeyBlock).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_tr34_key_block(&self) -> ::std::result::Result<&crate::types::ImportTr34KeyBlock, &Self> {
        if let ImportKeyMaterial::Tr34KeyBlock(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Tr34KeyBlock`](crate::types::ImportKeyMaterial::Tr34KeyBlock).
    pub fn is_tr34_key_block(&self) -> bool {
        self.as_tr34_key_block().is_ok()
    }
    /// Tries to convert the enum instance into [`TrustedCertificatePublicKey`](crate::types::ImportKeyMaterial::TrustedCertificatePublicKey), extracting the inner [`TrustedCertificatePublicKey`](crate::types::TrustedCertificatePublicKey).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_trusted_certificate_public_key(&self) -> ::std::result::Result<&crate::types::TrustedCertificatePublicKey, &Self> {
        if let ImportKeyMaterial::TrustedCertificatePublicKey(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`TrustedCertificatePublicKey`](crate::types::ImportKeyMaterial::TrustedCertificatePublicKey).
    pub fn is_trusted_certificate_public_key(&self) -> bool {
        self.as_trusted_certificate_public_key().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
