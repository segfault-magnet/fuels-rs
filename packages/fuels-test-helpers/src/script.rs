use fuel_core::service::{Config, FuelService};
use fuel_gql_client::client::FuelClient;
use fuel_gql_client::fuel_tx::{Receipt, Transaction};
use fuels_contract::script::Script;
use std::fs::read;

/// Helper function to reduce boilerplate code in tests.
/// Used to run a script which returns a boolean value.0
pub async fn run_script(bin_path: &str) -> Vec<Receipt> {
    let bin = read(bin_path);
    let server = FuelService::new_node(Config::local_node()).await.unwrap();
    let client = FuelClient::from(server.bound_address);

    let tx = Transaction::Script {
        gas_price: 0,
        gas_limit: 1000000,
        maturity: 0,
        byte_price: 0,
        receipts_root: Default::default(),
        script: bin.unwrap(), // Here we pass the compiled script into the transaction
        script_data: vec![],
        inputs: vec![],
        outputs: vec![],
        witnesses: vec![vec![].into()],
        metadata: None,
    };

    let script = Script::new(tx);
    let receipts = script.call(&client).await.unwrap();

    receipts
}
