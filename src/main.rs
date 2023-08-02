use hex::{FromHex, FromHexError};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Any {
    pub type_name: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct PackageMetadata {
    pub name: String,
    pub upgrade_policy: UpgradePolicy,
    pub upgrade_number: u64,
    pub source_digest: String,
    #[serde(with = "serde_bytes")]
    pub manifest: Vec<u8>,
    pub modules: Vec<ModuleMetadata>,
    pub deps: Vec<PackageDep>,
    pub extension: MoveOption<Any>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AccountAddress([u8; 32]);

fn from_account_address_string<'de, D>(deserializer: D) -> Result<AccountAddress, D::Error>
    where
        D: Deserializer<'de>,
{
    let account_str = String::deserialize(deserializer)?;
    let account_bytes = hex::decode(account_str).map_err(serde::de::Error::custom)?;
    let mut account_address = [0u8; 32];
    account_address.copy_from_slice(&account_bytes[..32]);
    Ok(AccountAddress(account_address))
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct PackageDep {
    #[serde(deserialize_with = "from_account_address_string")]
    pub account: AccountAddress,
    pub package_name: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ModuleMetadata {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub source: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub source_map: Vec<u8>,
    pub extension: MoveOption<Any>,
}


#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct MoveOption<T> {
    pub value: Vec<T>,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UpgradePolicy {
    pub policy: u8,
}

impl AccountAddress {
    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<AccountAddress, FromHexError> {
        <[u8; 32]>::from_hex(hex)
            .map(Self)
    }
}


fn bsc_hex() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    // skip the first argument, its the name
    let json_hex = args.get(1).unwrap();
    let json_str = String::from_utf8(hex::decode(json_hex).unwrap()).unwrap();
    let json: PackageMetadata = serde_json::from_str(&json_str).unwrap();
    let res = bcs::to_bytes(&json).unwrap();
    let ret = format!("{}", hex::encode(res));
    println!("{}", ret);
}

fn main() { bsc_hex() }

