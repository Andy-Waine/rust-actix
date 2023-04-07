use actix_web::{web, HttpResponse};
use crate::models::GameState;

pub fn game_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/game")
            .route(web::post().to(controllers::game_controller::start_game))
            .route(web::get().to(controllers::game_controller::get_game_state))
            .route(web::put().to(controllers::game_controller::make_move))
            .route(web::delete().to(controllers::game_controller::end_game)),
    );
}

pub fn start_game(_data: web::Json<GameState>) -> HttpResponse {
    let state = GameState {
        players: Vec::new(),
        deck: Vec::new(),
        status: String::from("Pending"),
    };

    HttpResponse::Ok().json(state)
}

pub fn get_game_state(state: web::Data<GameState>) -> HttpResponse {
    HttpResponse::Ok().json(state)
}

pub fn make_move(state: web::Data<GameState>, data: web::Json<Player>) -> HttpResponse {
    // Find the player in the game state
    let mut player = state.players.iter_mut().find(|p| p.name == data.name).unwrap();

    // Add a card to the player's hand
    player.cards.push(state.deck.pop().unwrap());

    // Calculate the player's score
    let mut score = 0;
    for card in &player.cards {
        if card.value >= 10 {
            score += 10;
        } else {
            score += card.value;
        }
    }
    player.score = score;

    // Check for a winner
    if player.score == 21 {
        state.status = String::from("Won");
    }

    HttpResponse::Ok().json(state)
}

pub fn end_game(_data: web::Data<GameState>) -> HttpResponse {
    HttpResponse::Ok().json("Game Ended")
}