use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize)]
pub struct AccountBalance {
    pub balance: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]

pub struct CreateGameRequest {
    pub payer_keypair: String,
    pub escrow_token_account: String,
    pub entry_price: u64,
    pub player1_choice: bool,
    pub program_id: String, // Nuevo campo para program_id
}