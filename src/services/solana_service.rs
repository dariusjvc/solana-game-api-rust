use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use actix_web::web;

pub async fn get_account_balance(account_pubkey: &str, rpc_url: &str) -> Result<u64, String> {
    let pubkey_result = Pubkey::from_str(account_pubkey);
    if pubkey_result.is_err() {
        return Err("Invalid public key format".to_string());
    }
    let pubkey = pubkey_result.unwrap();
    let rpc_url = rpc_url.to_string();

    // Use web::block for blocking operations
    web::block(move || {
        let client = RpcClient::new(&rpc_url);
        client.get_balance(&pubkey)
    })
    .await
    .map_err(|err| format!("Error fetching balance: {}", err))?
    .map_err(|err| format!("Failed to fetch balance: {}", err))
}
