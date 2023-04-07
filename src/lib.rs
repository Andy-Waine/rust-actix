pub mod blackjack_controller;
pub mod models;
pub mod routes;
pub mod server;

pub fn blackjack_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/blackjack")
            .route(web::post().to(blackjack_controller::start_game))
            .route(web::get().to(blackjack_controller::get_game_state))
            .route(web::put().to(blackjack_controller::make_move)),
    );
}
