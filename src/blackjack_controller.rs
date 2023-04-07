use actix_web::{web, HttpResponse};
use crate::models::{GameState, Player};

pub fn start_game(data: web::Json<Player>) -> HttpResponse {
    let mut player = Player {
        name: data.name.clone(),
        cards: Vec::new(),
        score: 0,
        is_dealer: false,
    };
    let mut deck = Vec::new();

    // Create a standard 52 card deck
    for i in 0..4 {
        for j in 1..14 {
            match i {
                0 => {
                    let card = Card{
                        value: j,
                        suit: String::from("Spades")
                    };
                    deck.push(card);
                },
                1 => {
                    let card = Card{
                        value: j,
                        suit: String::from("Hearts")
                    };
                    deck.push(card);
                },
                2 => {
                    let card = Card{
                        value: j,
                        suit: String::from("Clubs")
                    };
                    deck.push(card);
                },
                3 => {
                    let card = Card{
                        value: j,
                        suit: String::from("Diamonds")
                    };
                    deck.push(card);
                },
                _ => (),
            }
        }
    }

    // Shuffle the deck
    deck.shuffle(&mut thread_rng());

    // Deal two cards to the player
    player.cards.push(deck.pop().unwrap());
    player.cards.push(deck.pop().unwrap());

    // Create a new game state
    let mut game_state = GameState {
        players: vec![player],
        deck,
        status: String::from("Pending"),
    };

    // Return the game state
    HttpResponse::Ok().json(game_state)
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