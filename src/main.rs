use chopsticks::controller::*;
use chopsticks::Chopsticks;

fn main() {
    let game = Chopsticks::<u128>::default();
    let player_1 = &mut command_prompt::CommandPrompt {};
    let player_2 = &mut random::Random {};
    let players: Vec<&mut dyn Controller> = vec![player_1, player_2];
    game.play_game(players)
}
