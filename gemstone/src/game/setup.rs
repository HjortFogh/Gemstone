use rand::{seq::SliceRandom, thread_rng};

use crate::{
    errors::{GemError, Result},
    player::PlayerBehavior,
};

use super::Game;

/// A struct representing the setup-phase of the game.
#[derive(Default)]
pub struct GameSetup {
    behaviors: Vec<Box<dyn PlayerBehavior>>,
}

impl GameSetup {
    /// Adds a [`PlayerBehavior`] to the game. This function will return an
    /// error if the current number of players already equal or exceed four.
    pub fn add_player<T: PlayerBehavior + Default + 'static>(&mut self) -> Result<()> {
        self.insert_player(T::default())?;
        Ok(())
    }

    pub fn insert_player(&mut self, player: impl PlayerBehavior + 'static) -> Result<()> {
        if self.behaviors.len() >= 4 {
            return Err(GemError::ReachedPlayerLimit);
        }
        self.behaviors.push(Box::new(player));
        Ok(())
    }

    /// Shuffles the playing order, such that any player have an equal chance to start.
    pub fn shuffle_players(&mut self) {
        self.behaviors.shuffle(&mut thread_rng());
    }

    /// Finish the setup-phase and get the actual [`Game`]-struct. This
    /// function will return an error if the number of players is less
    /// than two.
    pub fn finish(self) -> Result<Game> {
        if self.behaviors.len() < 2 {
            return Err(GemError::TooFewPlayers);
        }
        Ok(Game::new(self.behaviors))
    }
}
