use chopsticks::{
    chopsticks::Chopsticks,
    player_controller::{
        controllers::{command_prompt::CommandPrompt, random::Random},
        PlayerController,
    },
};

fn main() {
    let game = Chopsticks::<u128>::default();
    let player_1 = &mut Random {};
    let player_2 = &mut CommandPrompt {};
    let players: Vec<&mut dyn PlayerController> = vec![player_1, player_2];
    game.play_game(players)
}
