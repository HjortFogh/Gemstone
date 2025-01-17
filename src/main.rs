mod human_player;

use gemstone::*;
use human_player::HumanBehavior;

fn main() -> Result<()> {
    let mut game_setup = GameSetup::default();
    game_setup.insert_player(HumanBehavior::new("Alice"))?;
    game_setup.insert_player(HumanBehavior::new("Charlie"))?;
    game_setup.shuffle_players();

    let mut game = game_setup.finish()?;
    match game.run() {
        Ok(scores) => println!("final scores: {scores:?}"),
        Err(err) => {
            println!("{}", GemNotation::from_info(game.info_ref()));
            return Err(err);
        }
    }

    Ok(())
}
