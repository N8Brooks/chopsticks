use chopsticks::{chopsticks::Chopsticks, player_controller::controllers::random::Random};

fn main() {
    let game = Chopsticks::<u128>::default();
    let players = vec![
        Random {},
        Random {},
    ];
    game.play_game(players)
}
