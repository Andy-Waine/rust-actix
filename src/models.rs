
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