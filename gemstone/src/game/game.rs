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
        let mut game_info = GameInfo::new(behaviors.len());
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
        if self.info.game_over() {
            return Ok(Some(self.info.scores()));
        }
        match self.info.is_auction_phase() {
            true => self.step_auction()?,
            false => self.step_reinvestment()?,
        }
        Ok(None)
    }

    // TODO: docs
    fn step_auction(&mut self) -> Result<()> {
        if !self.info.round_over() {
            let idx = self.info.current_player();
            let bid = self.behaviors.borrow_mut()[idx].bid(self.info_ref()).max(0);

            if self.info.inventory_at(idx).iter().capital() < bid {
                return Err(GemError::CannotAffordBid);
            }
            if bid > self.info.highest_bid() {
                self.info.set_highest_bid(bid, idx);
            }

            self.info.increment_player();
        } else {
            let idx = self.info.highest_bidder();
            let inv = self.info.inventory_at(idx);
            let (mut card_idx, payment_choice) =
                self.behaviors.borrow_mut()[idx].pick_card(self.info_ref());
            card_idx = card_idx.min(self.info.stack_size());

            if inv.choose(payment_choice).leveraged().count() != 0 {
                return Err(GemError::TriedToUseLeveragedCard);
            }
            if inv.choose(payment_choice).scalar_value() < self.info.highest_bid() {
                return Err(GemError::CannotAffordBid);
            }

            self.info.buy_card(card_idx, idx, payment_choice);
            self.info
                .start_step_cycle(self.info.next_clockwise_player(self.info.highest_bidder()));
        }

        if self.info.is_reinvestment_phase() {
            self.info.start_step_cycle(self.info.highest_bidder());
        }

        Ok(())
    }

    // TODO: docs
    fn step_reinvestment(&mut self) -> Result<()> {
        let idx = self.info.current_player() as usize;
        let choices = self.behaviors.borrow_mut()[idx].reinvest(self.info_ref());

        let current_inv = self.info.current_inventory();
        if current_inv.choose(choices).scalar_value() < 0 {
            return Err(GemError::CannotAffortToFlip);
        }
        self.info.flip_cards(idx, choices);

        self.info.increment_player();

        // reinvestment phase only has one round
        if self.info.round_over() {
            self.info.reset_coin_cards();

            self.info.increment_round_index();
            self.info
                .start_step_cycle(self.info.next_clockwise_player(self.info.highest_bidder()));

            if !self.info.game_over() {
                self.info.prepare_auction();
            }
        }

        Ok(())
    }

    /// Returns a reference to the [`GameInfo`].
    pub fn info_ref(&self) -> &GameInfo {
        &self.info
    }
}
