use schnorrkel::{ Keypair, vrf::{ VRFInOut, VRFProof } };
use merlin::Transcript;

pub struct Player {
    keypair: Keypair,
    vrf_in_out: Option<(VRFInOut, VRFProof)>,
}

impl Player {
    pub fn new() -> Self {
        let keypair = Keypair::generate();
        Player {
            keypair,
            vrf_in_out: None,
        }
    }

    fn draw_card(&mut self, input: &[u8]) {
        let mut transcript = Transcript::new(b"VRF");
        transcript.append_message(b"input", input);
        let (io, proof, _) = self.keypair.vrf_sign(transcript);
        self.vrf_in_out = Some((io, proof));
    }

    fn reveal_card(&mut self) -> Option<VRFInOut> {
        self.vrf_in_out.clone().map(|(io, _)| io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_player() {
        let player = Player::new();
        assert_ne!(player.keypair.public, Keypair::generate().public)
    }

    #[test]
    fn player_draws_and_reveals_card() {
        let mut player = Player::new();
        let input = b"test input";
        player.draw_card(input);

        let vrf_in_out = player.reveal_card();
        assert!(vrf_in_out.is_some());
    }
}
