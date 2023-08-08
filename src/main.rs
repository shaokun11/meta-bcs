use hex::{FromHex, FromHexError};
use serde::{Deserialize, Deserializer, Serialize};
use std::env;
use serde_json::{json, Value};

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
    let args: Vec<String> = env::args().collect();
    // skip the first argument, its the name
    let json_hex = args.get(2).unwrap();
    let json_str = String::from_utf8(hex::decode(json_hex).unwrap()).unwrap();
    let json: PackageMetadata = serde_json::from_str(&json_str).unwrap();
    let res = bcs::to_bytes(&json).unwrap();
    let ret = format!("{}", hex::encode(res));
    println!("{}", ret);
}


fn disassemble() {
    let args: Vec<String> = env::args().collect();
    let bytecode_hex = args.get(2).unwrap();
    let code = hex::decode(bytecode_hex).unwrap();
    // let code = hex::decode("a11ceb0b060000000b01000202020403060f05150c072137085820067822109a01260ac001050cc501470d8c0202000000010800000200010000030200000004030000000103010c01060c010708000764656d6f31303807436f756e7465720b6765745f636f756e74657208696e6372656173650b696e69745f6d6f64756c6505636f756e74a272d39841bac0be1e8a8f5f4ab3185be67527cd7f5d8f3973df6140961299f90520a272d39841bac0be1e8a8f5f4ab3185be67527cd7f5d8f3973df6140961299f9126170746f733a3a6d657461646174615f7631120000010b6765745f636f756e74657201010000020105030001000100000507002b00100014020101040100040c07002a000c010a01100014060100000000000000160b010f0015020200000000050b0006010000000000000012002d0002000000").unwrap();
    let result = move_binary_format::CompiledModule::deserialize(&code).unwrap();
    let res = format!("{:#?}", result);
    println!("{}", res);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).unwrap();
    match cmd.as_str() {
        "bcs" => bsc_hex(),
        "disassemble" => disassemble(),
        _ => {}
    }
}

