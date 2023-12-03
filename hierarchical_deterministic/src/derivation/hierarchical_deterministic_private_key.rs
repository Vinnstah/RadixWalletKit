use wallet_kit_common::types::keys::{
    ed25519::private_key::Ed25519PrivateKey, private_key::PrivateKey,
};

use crate::cap26::{cap26_path::paths::account_path::AccountPath, cap26_repr::CAP26Repr};

use super::{
    derivation_path::DerivationPath,
    hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey,
};

/// An ephemeral (never persisted) HD PrivateKey which contains
/// the derivation path used to derive it.
pub struct HierarchicalDeterministicPrivateKey {
    /// The PrivateKey derived from some HD FactorSource using `derivation_path`.
    pub private_key: PrivateKey,
    /// Derivation path used to derive the `PrivateKey` from some HD FactorSource.
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPrivateKey {
    /// Instantiates a new `HierarchicalDeterministicPrivateKey` from a PrivateKey and
    /// the derivation path used to derive it.
    pub fn new(private_key: PrivateKey, derivation_path: DerivationPath) -> Self {
        Self {
            private_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicPrivateKey {
    /// Returns the public key of the private key with the derivation path intact.
    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(
            self.private_key.public_key(),
            self.derivation_path.clone(),
        )
    }

    /// The PrivateKey as hex string.
    pub fn to_hex(&self) -> String {
        self.private_key.to_hex()
    }
}

impl HierarchicalDeterministicPrivateKey {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::new(
            Ed25519PrivateKey::from_str(
                "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            )
            .unwrap()
            .into(),
            AccountPath::from_str("m/44H/1022H/1H/525H/1460H/0H")
                .unwrap()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::HierarchicalDeterministicPrivateKey;

    #[test]
    fn publickey_of_placeholder() {
        let sut = HierarchicalDeterministicPrivateKey::placeholder();
        assert_eq!(
            sut.public_key().to_hex(),
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
        );
    }
}