use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use pallas_addresses::Address;

pub fn print_utxo_cost_per_byte(json_path: PathBuf) {
    let network_params: NetworkParams = NetworkParams::builder().new_from_json(json_path);
    println!("UTXO Cost Per Byte: {}", network_params.utxo_cost_per_byte);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkParams {
    pub utxo_cost_per_byte: u32,
    pub tx_fee_per_byte: u32,
}

impl NetworkParams {
    pub fn builder() -> NetworkParamsBuilder {
        NetworkParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct NetworkParamsBuilder {}

impl NetworkParamsBuilder {
    pub fn new_from_json(&self, json_path: PathBuf) -> NetworkParams {
        let mut file = File::open(json_path).expect("Error opening JSON file");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Error reading JSON");

        serde_json::from_str(&json_data).expect("Error parsing JSON")
    }
}

#[test]
fn test_load_params_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = std::env::current_dir()?;
    path.push("./src/mainnet-protocol-params.json");

    let network_params: NetworkParams = NetworkParams::builder().new_from_json(path);

    assert_eq!(network_params.utxo_cost_per_byte, 4310);
    assert_eq!(network_params.tx_fee_per_byte, 44);

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct PubKeyHash(Vec<u8>);

impl PubKeyHash {
    pub fn new(inner: &[u8]) -> Self {
        PubKeyHash(inner.to_vec())
    }
}

pub fn derive_pkh_from_address(address: Address) -> Option<PubKeyHash> {
    match address {
        Address::Shelley(shelley_address) => {
            let hash = shelley_address.payment().as_hash().to_vec();
            // Run tests with `cargo test -- --nocapture` in
            // order to print the statement below
            println!("Vec<u8> {:?}", hash);
            let pkh = PubKeyHash::new(&hash);
            Some(pkh)
        }
        _ => None,
    }
}

#[test]
fn test_derive_pkh_from_address() {
    // Valid Address
    let bech32_address = "addr_test1qpmtp5t0t5y6cqkaz7rfsyrx7mld77kpvksgkwm0p7en7qum7a589n30e80tclzrrnj8qr4qvzj6al0vpgtnmrkkksnqd8upj0";

    // Style #1
    let address = Address::from_bech32(bech32_address).unwrap();
    let _pkh = derive_pkh_from_address(address).unwrap();
    assert!(true);

    // Style #2
    let address = Address::from_bech32(bech32_address).unwrap();
    if let Some(_address) = derive_pkh_from_address(address) {
        assert!(true)
    } else {
        assert!(false);
    }

    // Style #3
    match Address::from_bech32(bech32_address) {
        Ok(address) => {
            if let Some(_address) = derive_pkh_from_address(address) {
                assert!(true)
            } else {
                assert!(false);
            }
        }
        Err(_) => assert!(false),
    }
}
