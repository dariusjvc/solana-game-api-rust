use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod controllers;
mod services;
mod models;
mod config;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::solana_controller::get_account_balance,  // Añade las rutas de Solana
        controllers::game_controller::create_game,
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // Carga las variables de entorno desde el archivo .env
    env_logger::init(); // Inicializa el logger para depuración

    let app_config = config::app_config::AppConfig::from_env(); // Carga la configuración desde el entorno
    let api_doc = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_config.clone())) // Comparte la configuración con los controladores
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", api_doc.clone())
            )
            .configure(controllers::solana_controller::init_routes) // Rutas de Solana
            .configure(controllers::game_controller::init_routes) // Rutas de Solana
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
