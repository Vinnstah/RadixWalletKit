use crate::prelude::*;

use crate::NetworkID::{self, *};

/// A version of the Radix Network, for a NetworkID with an identifier (name) and display description (display name)
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{} ({})", self.display_description, self.id.discriminant())]
pub struct RadixNetwork {
    /// A String identifier (always lowercase) with the name of the Network that MUST match what Gateway returns.
    #[serde(rename = "name")]
    pub logical_name: String,

    /// The canonical identifier of this network.
    pub id: NetworkID,

    /// A name of the network intended for display purposes only.
    pub display_description: String,
}

impl Default for RadixNetwork {
    fn default() -> Self {
        Self::mainnet()
    }
}

impl RadixNetwork {
    fn declare(id: NetworkID, display: &str) -> Self {
        Self {
            logical_name: id.network_definition().logical_name,
            id,
            display_description: display.to_string(),
        }
    }
}

impl RadixNetwork {
    /// The Radix mainnet, the "real" Network on which all launched Dapps and
    /// assets with any real value resides.
    pub fn mainnet() -> Self {
        Self::declare(Mainnet, "Mainnet")
    }

    /// The primary public testnet of the Radix ecosystem, used by Dapp Developers
    /// and RDX Works alike to test new features.
    pub fn stokenet() -> Self {
        Self::declare(Stokenet, "Stokenet")
    }

    /// A Betanet.
    pub fn nebunet() -> Self {
        Self::declare(Nebunet, "Betanet")
    }

    /// Was a Release Candidate for Babylon launch.
    pub fn kisharnet() -> Self {
        Self::declare(Kisharnet, "RCnet (Test Network)")
    }

    /// Was the second Release Candidate for Babylon launch.
    pub fn ansharnet() -> Self {
        Self::declare(Ansharnet, "RCnet-V2 (Test Network)")
    }

    /// Was the third Release Candidate for Babylon launch.
    pub fn zabanet() -> Self {
        Self::declare(Zabanet, "RCnet-V3 (Test Network)")
    }

    /// A testnet.
    pub fn hammunet() -> Self {
        Self::declare(Hammunet, "Hammunet (Test Network)")
    }

    /// A testnet.
    pub fn enkinet() -> Self {
        Self::declare(Enkinet, "Enkinet (Test Network)")
    }

    /// A testnet.
    pub fn nergalnet() -> Self {
        Self::declare(NetworkID::Nergalnet, "Nergalnet (Test Network)")
    }

    /// A testnet.
    pub fn mardunet() -> Self {
        Self::declare(NetworkID::Mardunet, "Mardunet (Test Network)")
    }
}

impl HasPlaceholder for RadixNetwork {
    fn placeholder() -> Self {
        Self::mainnet()
    }

    fn placeholder_other() -> Self {
        Self::stokenet()
    }
}

impl RadixNetwork {
    pub fn lookup_by_id(id: NetworkID) -> Result<Self> {
        let map = Self::lookup_map();
        let Some(network) = map.get(&id) else {
            return Err(CommonError::UnknownNetworkForID(id.discriminant()));
        };
        Ok(network.clone())
    }

    pub fn lookup_by_name(logical_name: &str) -> Result<Self> {
        let map = Self::lookup_map();

        map.iter()
            .find(|p| p.1.logical_name == logical_name)
            .map(|p| p.0)
            .ok_or_else(|| {
                CommonError::UnknownNetworkWithName(logical_name.to_string())
            })
            .and_then(|id| Self::lookup_by_id(*id))
    }

    fn lookup_map() -> HashMap<NetworkID, Self> {
        use NetworkID::*;
        HashMap::from([
            (Mainnet, Self::mainnet()),
            (Stokenet, Self::stokenet()),
            (Nebunet, Self::nebunet()),
            (Kisharnet, Self::kisharnet()),
            (Ansharnet, Self::ansharnet()),
            (Zabanet, Self::zabanet()),
            (Hammunet, Self::hammunet()),
            (Enkinet, Self::enkinet()),
            (Mardunet, Self::mardunet()),
            (Nergalnet, Self::nergalnet()),
        ])
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(RadixNetwork::placeholder(), RadixNetwork::placeholder());
        assert_eq!(
            RadixNetwork::placeholder_other(),
            RadixNetwork::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            RadixNetwork::placeholder(),
            RadixNetwork::placeholder_other()
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", RadixNetwork::mainnet()), "Mainnet (1)");
    }

    #[test]
    fn placeholder() {
        assert_eq!(RadixNetwork::placeholder().logical_name, "mainnet");
    }

    #[test]
    fn default_is_mainnet() {
        assert_eq!(RadixNetwork::default(), RadixNetwork::mainnet());
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = RadixNetwork::mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "mainnet",
                "id": 1,
                "displayDescription": "Mainnet"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = RadixNetwork::stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "stokenet",
                "id": 2,
                "displayDescription": "Stokenet"
            }
            "#,
        )
    }

    #[test]
    fn lookup_by_name_error() {
        assert_eq!(
            RadixNetwork::lookup_by_name("x"),
            Err(CommonError::UnknownNetworkWithName("x".to_string()))
        );
    }

    #[test]
    fn lookup_by_id_error() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Simulator),
            Err(CommonError::UnknownNetworkForID(
                NetworkID::Simulator.discriminant()
            ))
        );
    }

    #[test]
    fn lookup_by_id_mainnet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Mainnet),
            Ok(RadixNetwork::mainnet())
        );
    }

    #[test]
    fn lookup_by_id_stokenet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Stokenet),
            Ok(RadixNetwork::stokenet())
        );
    }

    #[test]
    fn lookup_by_name_mainnet() {
        assert_eq!(
            RadixNetwork::lookup_by_name("mainnet"),
            Ok(RadixNetwork::mainnet())
        );
    }

    #[test]
    fn lookup_by_name_stokenet() {
        assert_eq!(
            RadixNetwork::lookup_by_name("stokenet"),
            Ok(RadixNetwork::stokenet())
        );
    }

    #[test]
    fn lookup_by_id_nergalnet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Nergalnet),
            Ok(RadixNetwork::nergalnet())
        );
    }

    #[test]
    fn lookup_by_id_mardunet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Mardunet),
            Ok(RadixNetwork::mardunet())
        );
    }

    #[test]
    fn lookup_by_id_enkinet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Enkinet),
            Ok(RadixNetwork::enkinet())
        );
    }

    #[test]
    fn lookup_by_id_hammunet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Hammunet),
            Ok(RadixNetwork::hammunet())
        );
    }

    #[test]
    fn lookup_by_id_zabanet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Zabanet),
            Ok(RadixNetwork::zabanet())
        );
    }

    #[test]
    fn lookup_by_id_ansharnet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Ansharnet),
            Ok(RadixNetwork::ansharnet())
        );
    }

    #[test]
    fn lookup_by_id_kisharnet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Kisharnet),
            Ok(RadixNetwork::kisharnet())
        );
    }

    #[test]
    fn lookup_by_id_nebunet() {
        assert_eq!(
            RadixNetwork::lookup_by_id(NetworkID::Nebunet),
            Ok(RadixNetwork::nebunet())
        );
    }
}
