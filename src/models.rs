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
