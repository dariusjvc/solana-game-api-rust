use actix_web::{web, HttpResponse, Responder};
use crate::services::game_service;
use crate::models::CreateGameRequest;


#[utoipa::path(
    post,
    path = "/escrow/create",
    request_body = CreateGameRequest,
    responses(
        (status = 200, description = "Game created successfully", body = String),
        (status = 500, description = "Error creating game")
    )
)]
pub async fn create_game(data: web::Json<CreateGameRequest>) -> impl Responder {
    match game_service::create_game(&data.0).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/escrow")
            .route("/create", web::post().to(create_game))
            // .route("/{game_account}/status", web::get().to(get_game_status))
            // .route("/join", web::post().to(join_game)),
    );
}