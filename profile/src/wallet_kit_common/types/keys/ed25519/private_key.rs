use crate::prelude::*;

use radix_engine_common::crypto::IsHash;
use transaction::signing::ed25519::{
    Ed25519PrivateKey as EngineEd25519PrivateKey, Ed25519Signature,
};

/// An Ed25519 private key used to create cryptographic signatures, using
/// EdDSA scheme.
#[derive(derive_more::Debug)]
#[debug("{}", self.to_hex())]
pub struct Ed25519PrivateKey(EngineEd25519PrivateKey);

impl Ed25519PrivateKey {
    /// Generates a new `Ed25519PrivateKey` from random bytes
    /// generated by a CSRNG, note that this is typically never
    /// used by wallets, which tend to rather use a Mnemonic and
    /// derive hierarchical deterministic keys.
    pub fn generate() -> Self {
        Self::from_hex32_bytes(Hex32Bytes::generate())
            .expect("Should be able to generate 32 bytes")
    }
}

impl PartialEq for Ed25519PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for Ed25519PrivateKey {}

impl IsPrivateKey<Ed25519PublicKey> for Ed25519PrivateKey {
    fn curve() -> SLIP10Curve {
        SLIP10Curve::Curve25519
    }

    type Signature = Ed25519Signature;

    fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey::from_engine(self.0.public_key()).expect(
            "Public Key from EC scalar multiplication should always be valid.",
        )
    }

    fn sign(&self, msg_hash: &impl IsHash) -> Ed25519Signature {
        self.0.sign(msg_hash)
    }
}

impl Ed25519PrivateKey {
    pub fn from_engine(engine: EngineEd25519PrivateKey) -> Self {
        Self(engine)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn from_bytes(slice: &[u8]) -> Result<Self> {
        EngineEd25519PrivateKey::from_bytes(slice)
            .map_err(|_| {
                CommonError::InvalidEd25519PrivateKeyFromBytes(slice.to_owned())
            })
            .map(Self::from_engine)
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes(bytes.as_slice())
    }

    pub fn from_hex32_bytes(bytes: Hex32Bytes) -> Result<Self> {
        Self::from_vec(bytes.to_vec())
    }
}

impl TryFrom<&[u8]> for Ed25519PrivateKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Ed25519PrivateKey, Self::Error> {
        Ed25519PrivateKey::from_bytes(slice)
    }
}

impl FromStr for Ed25519PrivateKey {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hex32Bytes::from_hex(s)
            .map_err(|_| {
                CommonError::InvalidEd25519PrivateKeyFromString(s.to_owned())
            })
            .and_then(|b| Self::from_bytes(&b.to_vec()))
    }
}

impl HasPlaceholder for Ed25519PrivateKey {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_bob()
    }
}

impl Ed25519PrivateKey {
    /// `833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42`
    ///
    /// expected public key:
    /// `ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf`
    ///
    /// https://github.com/dalek-cryptography/ed25519-dalek/blob/main/tests/ed25519.rs#L103
    pub fn placeholder_alice() -> Self {
        Self::from_str(
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42",
        )
        .unwrap()
    }

    /// `1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93``

    /// expected public key:
    /// `b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde`
    ///
    /// https://cryptobook.nakov.com/digital-signatures/eddsa-sign-verify-examples
    pub fn placeholder_bob() -> Self {
        Self::from_str(
            "1498b5467a63dffa2dc9d9e069caf075d16fc33fdd4c3b01bfadae6433767d93",
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use transaction::signing::ed25519::Ed25519Signature;

    #[test]
    fn equality() {
        assert_eq!(
            Ed25519PrivateKey::placeholder(),
            Ed25519PrivateKey::placeholder()
        );
        assert_eq!(
            Ed25519PrivateKey::placeholder_other(),
            Ed25519PrivateKey::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Ed25519PrivateKey::placeholder(),
            Ed25519PrivateKey::placeholder_other()
        );
    }

    #[test]
    fn curve() {
        assert_eq!(Ed25519PrivateKey::curve(), SLIP10Curve::Curve25519);
    }

    #[test]
    fn sign_and_verify() {
        let msg = hash("Test");
        let sk: Ed25519PrivateKey =
            "0000000000000000000000000000000000000000000000000000000000000001"
                .parse()
                .unwrap();
        let pk = sk.public_key();
        assert_eq!(
            pk.to_hex(),
            "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"
        );
        let sig = Ed25519Signature::from_str("cf0ca64435609b85ab170da339d415bbac87d678dfd505969be20adc6b5971f4ee4b4620c602bcbc34fd347596546675099d696265f4a42a16df343da1af980e").unwrap();

        assert_eq!(sk.sign(&msg), sig);
        assert!(pk.is_valid(&sig, &msg))
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes = hex_decode(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )
        .unwrap();
        assert_eq!(
            Ed25519PrivateKey::from_bytes(bytes.as_slice())
                .unwrap()
                .to_bytes(),
            bytes.as_slice()
        );
    }

    #[test]
    fn hex_roundtrip() {
        let hex =
            "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(Ed25519PrivateKey::from_str(hex).unwrap().to_hex(), hex);
    }

    #[test]
    fn invalid_hex() {
        assert_eq!(
            Ed25519PrivateKey::from_str("not hex"),
            Err(CommonError::InvalidEd25519PrivateKeyFromString(
                "not hex".to_owned()
            ))
        );
    }

    #[test]
    fn invalid_hex_too_short() {
        assert_eq!(
            Ed25519PrivateKey::from_str("dead"),
            Err(CommonError::InvalidEd25519PrivateKeyFromString(
                "dead".to_owned()
            ))
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Ed25519PrivateKey::from_bytes(&[0u8] as &[u8]),
            Err(CommonError::InvalidEd25519PrivateKeyFromBytes(vec![0]))
        );
    }

    #[test]
    fn debug() {
        let hex =
            "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(
            format!("{:?}", Ed25519PrivateKey::from_str(hex).unwrap()),
            hex
        );
    }

    #[test]
    fn from_vec() {
        let hex =
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        assert_eq!(
            Ed25519PrivateKey::from_vec(hex_decode(hex).unwrap())
                .unwrap()
                .to_hex(),
            hex
        );
    }

    #[test]
    fn from_hex32() {
        let hex =
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        assert_eq!(
            Ed25519PrivateKey::from_hex32_bytes(
                Hex32Bytes::from_hex(hex).unwrap()
            )
            .unwrap()
            .to_hex(),
            hex
        );
    }

    #[test]
    fn generate_new() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let key = Ed25519PrivateKey::generate();
            let bytes = key.to_bytes();
            assert_eq!(bytes.len(), 32);
            set.insert(bytes);
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn from_hex32_bytes() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000001";
        let hex32 = Hex32Bytes::from_hex(str).unwrap();
        let key = Ed25519PrivateKey::from_hex32_bytes(hex32).unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn try_from_bytes() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000001";
        let vec = hex_decode(str).unwrap();
        let key = Ed25519PrivateKey::try_from(vec.as_slice()).unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            Ed25519PrivateKey::placeholder().public_key().to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }
}
