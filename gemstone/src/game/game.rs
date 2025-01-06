use std::cell::RefCell;

use crate::{errors::Result, player::PlayerBehavior, GemError};

use super::{CardIterator, GameInfo, GameScores};

/// The `Game` struct represents a current active game of Gem.
pub struct Game {
    info: GameInfo,
    behaviors: RefCell<Vec<Box<dyn PlayerBehavior>>>,
}

impl Game {
    /// Create a new `Game` given a number of [`PlayerBehavior`]-implementers.
    /// There must exist at least two and at most four behaviors.
    pub fn new(behaviors: Vec<Box<dyn PlayerBehavior>>) -> Self {
        assert!(behaviors.len() >= 2 && behaviors.len() <= 4);
        let mut game_info = GameInfo::new(behaviors.len() as u8);
        game_info.prepare_auction();
        Self {
            info: game_info,
            behaviors: RefCell::new(behaviors),
        }
    }

    /// This function will [`step`](`Self::step`) through the game and only
    /// return once this game has concluded. If no errors were encountered
    /// then this function will return the final game scores.
    pub fn run(&mut self) -> Result<GameScores> {
        Ok(loop {
            if let Some(score) = self.step()? {
                break score;
            }
        })
    }

    /// Take one step in the game, were a step corresponds to a single desicion
    /// made by any player. If the game has concluded and no errors were
    /// encountered this will return `Some(GameScores)`, otherwise `None` for a
    /// game still in progress.
    pub fn step(&mut self) -> Result<Option<GameScores>> {
        if self.info.is_game_over() {
            return Ok(Some(self.info.scores()));
        }
        match self.info.is_auction_phase() {
            true => self.step_auction()?,
            false => self.step_reinvestment()?,
        }
        Ok(None)
    }

    // TODO: implement, docs
    fn step_auction(&mut self) -> Result<()> {
        // query the current player for their bid and clamp negative bids to 0
        let current_player = self.info.current_player();
        let bid = self.behaviors.borrow_mut()[current_player as usize]
            .bid(self.info_ref())
            .max(0);
        if self.info.current_inventory().iter().capital() < bid {
            return Err(GemError::CannotAffordBid);
        }

        if bid > self.info.highest_bid() {
            self.info.set_highest_bid(bid, current_player);
        }

        self.info.next_player();

        // check if this is the end of a round
        // on average any auction phase will have an average of three rounds
        if self.info.is_end_of_round() {
            self.info.set_current_player(self.info.highest_bidder());
            let current_player = self.info.current_player();
            let current_inv = self.info.current_inventory();

            let (selected_card, payment_choices) =
                self.behaviors.borrow_mut()[current_player as usize].pick_card(self.info_ref());

            if current_inv.choose(payment_choices).leveraged().count() != 0 {
                return Err(GemError::TriedToUseLeveragedCard);
            }
            if current_inv.choose(payment_choices).capital() < self.info.highest_bid() {
                return Err(GemError::CannotAffordBid);
            }

            self.info
                .buy_card(selected_card, current_player, payment_choices);

            self.info.prepare_auction();
        }

        // check if this is the end of the auction phase
        if self.info.is_reinvestment_phase() {
            // set the starting reinvester to the last buyer
        }

        Ok(())
    }

    // TODO: implement
    fn step_reinvestment(&self) -> Result<()> {
        todo!()
    }

    /// Returns a reference to the [`GameInfo`].
    pub fn info_ref(&self) -> &GameInfo {
        &self.info
    }
}
