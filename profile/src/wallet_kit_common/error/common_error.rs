use thiserror::Error;

use super::{bytes_error::BytesError, hdpath_error::HDPathError, key_error::KeyError};

pub type Result<T> = std::result::Result<T, CommonError>;

#[derive(Clone, Debug, Error, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum CommonError {
    ///
    /// NESTED
    ///
    #[error("Hierarchical Deterministic Path error")]
    HDPath(#[from] HDPathError),

    #[error("EC key error")]
    Key(#[from] KeyError),

    #[error("Bytes error")]
    Bytes(#[from] BytesError),

    ///
    /// UN-NESTED
    ///

    #[error("Appearance id not recognized.")]
    InvalidAppearanceID,

    #[error("Invalid Account Address '{0}'.")]
    InvalidAccountAddress(String),

    #[error("Unsupported engine entity type.")]
    UnsupportedEntityType,

    #[error("Failed to decode address from bech32.")]
    FailedToDecodeAddressFromBech32,

    #[error("Failed to decode address mismatching entity type")]
    MismatchingEntityTypeWhileDecodingAddress,

    #[error("Failed to decode address mismatching HRP")]
    MismatchingHRPWhileDecodingAddress,

    #[error("Unknown network ID '{0}'")]
    UnknownNetworkID(u8),

    #[error("Failed to parse InvalidNonFungibleGlobalID from str.")]
    InvalidNonFungibleGlobalID,

    #[error("Supported SLIP10 curves in FactorSource crypto parameters is either empty or contains more elements than allowed.")]
    FactorSourceCryptoParametersSupportedCurvesInvalidSize,

    #[error("Failed to convert FactorInstance into HierarchicalDeterministicFactorInstance, badge is not virtual HD.")]
    BadgeIsNotVirtualHierarchicalDeterministic,

    #[error("Failed to create FactorSourceIDFromHash from FactorSourceID")]
    FactorSourceIDNotFromHash,

    #[error("Expected AccountPath but got something else.")]
    ExpectedAccountPathButGotSomethingElse,

    #[error("Wrong entity kind in path of FactorInstance")]
    WrongEntityKindOfInFactorInstancesPath,

    #[error("Wrong key kind of FactorInstance - expected transaction signing")]
    WrongKeyKindOfTransactionSigningFactorInstance,

    #[error("Wrong key kind of FactorInstance - expected authentication signing")]
    WrongKeyKindOfAuthenticationSigningFactorInstance,

    #[error("Expected DeviceFactorSource")]
    ExpectedDeviceFactorSourceGotSomethingElse,

    #[error("Expected LedgerHardwareWalletFactorSource")]
    ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse,

    #[error("No network found with name: '{0}'")]
    UnknownNetworkWithName(String),

    #[error("No network found with id: '{0}'")]
    UnknownNetworkForID(u8),

    #[error("Gateway discrepancy, 'other' should not contain 'current'.")]
    GatewaysDiscrepancyOtherShouldNotContainCurrent,

    #[error("Gateways discrepancy, invalid JSON, current not found amongst saved.")]
    InvalidGatewaysJSONCurrentNotFoundAmongstSaved,

    #[error("Invalid URL: '{0}'")]
    InvalidURL(String),

    #[error("Accounts on different networks.")]
    AccountOnWrongNetwork,

    #[error("FactorSources must not be empty.")]
    FactorSourcesMustNotBeEmpty,

    #[error("Failed to update FactorSource, error while mutating.")]
    UpdateFactorSourceMutateFailed,

    #[error("Failed to cast factor source, wrong kind.")]
    CastFactorSourceWrongKind,

    #[error("Length check failed.")]
    InvalidLength {
        expected: usize,
        actual: usize,
        data: Vec<u8>,
    },

    #[error("Invalid NonFungibleLocalID::String")]
    InvalidNonFungibleLocalIDString,

    #[error("Invalid NonFungibleLocalID::Bytes")]
    InvalidNonFungibleLocalIDBytes,

    #[error("Invalid Decimal")]
    DecimalError,

    #[error("Invalid BIP39 Index")]
    InvalidBIP39Index,

    #[error("Invalid DisplayName cannot be empty.")]
    InvalidDisplayNameEmpty,

    #[error("Invalid DisplayName too long.")]
    InvalidDisplayNameTooLong,

    #[error("Invalid ISO8601 Time string.")]
    InvalidISO8601String,

    #[error("Unknown account.")]
    UnknownAccount,
}