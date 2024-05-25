mod player;
mod poker_game;

fn main() {
    let mut game = poker_game::PokerGame::new(vec![1, 2, 3, 4]);

    // Add players
    game.add_player("Alice");
    game.add_player("Bob");
    game.add_player("Charlie");

    // Players draw cards
    game.draw_cards();

    // Reveal the winner
    if let Some(winner) = game.reveal_winner() {
        println!("The winner is: {}", winner);
    } else {
        println!("No winner could be determined.");
    }
}
