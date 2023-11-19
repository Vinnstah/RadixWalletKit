use serde::{Serialize, Deserialize};

use crate::{bip44::bip44_like_path::BIP44LikePath, cap26::cap26_path::CAP26Path};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum DerivationPath {
    CAP26(CAP26Path),
    BIP44Like(BIP44LikePath),
}
