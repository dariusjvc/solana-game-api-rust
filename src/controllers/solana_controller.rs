use actix_web::{get, web, HttpResponse, Responder};
use crate::services::solana_service;
use crate::config::app_config::AppConfig;

#[utoipa::path(
    get,
    path = "/balance/{account_pubkey}",
    params(
        ("account_pubkey" = String, Path, description = "Public key of the account")
    ),
    responses(
        (status = 200, description = "Account balance retrieved successfully", body = u64),
        (status = 400, description = "Invalid account public key"),
        (status = 500, description = "Error fetching balance"),
    )
)]
#[get("/balance/{account_pubkey}")]
pub async fn get_account_balance(
    app_config: web::Data<AppConfig>,
    account_pubkey: web::Path<String>,
) -> impl Responder {
    let pubkey = account_pubkey.into_inner();

    match solana_service::get_account_balance(&pubkey, &app_config.solana_rpc_url).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_account_balance);
}
