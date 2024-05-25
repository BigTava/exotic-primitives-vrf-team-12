use std::collections::HashMap;
use crate::player::Player;
use schnorrkel::vrf::VRFInOut;

pub struct PokerGame {
    players: HashMap<String, Player>,
    input: Vec<u8>,
}

impl PokerGame {
    pub fn new(input: Vec<u8>) -> Self {
        PokerGame {
            players: HashMap::new(),
            input,
        }
    }

    // Add a player to the game
    pub fn add_player(&mut self, name: &str) {
        let player = Player::new();
        self.players.insert(name.to_string(), player);
    }

    // Players draw cards
    pub fn draw_cards(&mut self) {
        for player in self.players.values_mut() {
            player.draw_card(&self.input);
        }
    }

    // Players reveal cards and determine the winner
    pub fn reveal_winner(&mut self) -> Option<&str> {
        let mut highest_vrf: Option<(&str, VRFInOut)> = None;

        for (name, player) in &mut self.players {
            if let Some(vrf_in_out) = &player.reveal_card() {
                match highest_vrf {
                    Some((_, highest)) if
                        vrf_in_out.output.to_bytes() > highest.output.to_bytes()
                    => {
                        highest_vrf = Some((name, vrf_in_out.clone()));
                    }
                    None => {
                        highest_vrf = Some((name, vrf_in_out.clone()));
                    }
                    _ => {}
                }
            }
        }

        highest_vrf.map(|(name, _)| name)
    }
}
