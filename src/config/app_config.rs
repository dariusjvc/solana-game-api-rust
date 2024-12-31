use std::env;


#[derive(Clone)] // Derivamos el trait Clone
pub struct AppConfig {
    pub solana_rpc_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
        }
    }
}
