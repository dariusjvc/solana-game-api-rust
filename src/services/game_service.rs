use tokio::task;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use serde::{Serialize};
use std::{env, fs, str::FromStr};
use dotenv::dotenv;

use crate::models::CreateGameRequest;
use serde_json;

#[derive(Serialize)]
pub struct GameStatusResponse {
    pub game_active: bool,
    pub player1: String,
    pub player2: Option<String>,
    pub entry_price: u64,
    pub last_price: u64,
    pub winner: Option<String>,
}

pub async fn create_game(request: &CreateGameRequest) -> Result<String, String> {
    dotenv().ok(); // Load variables from .env

    // Load necessary data
    let program_id = request.program_id.clone();
    let escrow_token_account = request.escrow_token_account.clone();
    let player1_choice = request.player1_choice;
    //let entry_price = request.entry_price;

    let result = task::spawn_blocking(move || {
        // Read payer.json
        let payer_path = env::var("PAYER_JSON_PATH").map_err(|_| "PAYER_JSON_PATH not set in .env")?;
        let payer_json = fs::read_to_string(payer_path).map_err(|e| format!("Failed to read payer.json: {}", e))?;
        let payer_bytes: Vec<u8> = serde_json::from_str(&payer_json)
            .map_err(|e| format!("Invalid payer.json content: {}", e))?;
        let payer = Keypair::from_bytes(&payer_bytes)
            .map_err(|e| format!("Invalid payer_keypair bytes: {}", e))?;

        // Create solana client
        let client = RpcClient::new("https://api.devnet.solana.com");
        let program_id = Pubkey::from_str(&program_id).map_err(|e| format!("Invalid program_id: {}", e))?;
        let escrow_token_account = Pubkey::from_str(&escrow_token_account).map_err(|e| e.to_string())?;

        // Create transaction
        let instruction_data = vec![
            0, // Create game instruction code
            player1_choice as u8,
        ];

        let transaction = Transaction::new_signed_with_payer(
            &[solana_sdk::instruction::Instruction {
                program_id,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
                    solana_sdk::instruction::AccountMeta::new(escrow_token_account, false),
                ],
                data: instruction_data,
            }],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().map_err(|e| e.to_string())?,
        );

        // Send transaction
        client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        Ok("Game created successfully".to_string())
    })
    .await // Wait for the result
    .map_err(|e| format!("Failed to execute blocking operation: {}", e))?;

    result
}
