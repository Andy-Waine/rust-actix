pub mod controllers;
pub mod models;
pub mod routes;
pub mod server;

pub fn blackjack_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/blackjack")
            .route(web::post().to(controllers::blackjack_controller::start_game))
            .route(web::get().to(controllers::blackjack_controller::get_game_state))
            .route(web::put().to(controllers::blackjack_controller::make_move)),
    );
}

Please provide the full code for models.rs

pub struct Card {
    pub value: u8,
    pub suit: String,
}

pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub score: u8,
    pub is_dealer: bool,
}

pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub status: String,
}
