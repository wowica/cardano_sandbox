use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


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

        let network_params: NetworkParams =
            serde_json::from_str(&json_data).expect("Error parsing JSON");

        return network_params;
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
