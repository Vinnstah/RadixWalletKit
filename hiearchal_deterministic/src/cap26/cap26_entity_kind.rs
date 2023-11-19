use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An account entity type
    Account = 525,

    /// Used by Persona
    Identity = 618,
}

impl Display for CAP26EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl CAP26EntityKind {
    fn description(&self) -> String {
        match self {
            Self::Account => "Account".to_string(),
            Self::Identity => "Identity".to_string(),
        }
    }
}
