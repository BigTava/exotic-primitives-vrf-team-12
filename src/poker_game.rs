use std::collections::HashMap;
use crate::player::Player;

struct PokerGame {
    players: HashMap<String, Player>,
    input: Vec<u8>,
}

impl PokerGame {
    fn new(input: Vec<u8>) -> Self {
        PokerGame {
            players: HashMap::new(),
            input,
        }
    }

    fn add_player(&mut self, name: &str) {
        let player = Player::new();
        self.players.insert(name.to_string(), player);
    }
}
