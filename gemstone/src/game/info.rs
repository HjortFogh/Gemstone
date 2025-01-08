use rand::thread_rng;

use crate::player::PlayerInventory;

use super::{Card, CardChoice, CardCollection, CardIterator, GemType};

// TODO: figure out whether to use u8/usize, i8/i32/custom bid struct

/// Represents the final game scores for each of the possible players.
#[derive(Default, Debug, Clone, Copy)]
#[allow(unused)]
pub struct GameScores([i32; 4]);

/// The `GameInfo` holds all variables necessary to represent a unqiue
/// game-state, but unlike the `Game`-struct this does not have any functions
/// to autonomously progress the state of the game and is only meant for
/// querying.
#[derive(Clone)]
pub struct GameInfo {
    /// The total number of players in range `[2..5)`.
    num_players: u8,
    /// The round index in range `[0..6)` while the game is still ongoing, and
    /// `[6..)` if the game has ended.
    round_index: u8,
    /// The current player index in range `[0..num_players)`.
    current_player: u8,
    /// The index of the starting player in the current (both auction /
    /// reinvestment) round.
    starting_player: u8,
    /// The index of the highest player.
    highest_bidder: u8,
    /// The highest bid made by any player, or `-1` if no bids have been made.
    highest_bid: i8,
    /// Inventories of all the players.
    /// TODO: If less than four players all non-used inventories [...]
    inventories: [PlayerInventory; 4],
    /// The current order of all cards in the deck.
    deck: CardCollection<18>,
    /// The current stack of cards. The game is in the reinvstment phase if all
    /// cards in the stack are considered [`null`](`Card::NULL`), otherwise the
    /// game is in the auction phase.
    stack: CardCollection<4>,
}

impl GameInfo {
    /// Returns the number of players in range `[2..5)`.
    #[inline]
    pub fn num_players(&self) -> u8 {
        self.num_players
    }

    #[inline]
    pub fn round_index(&self) -> u8 {
        self.round_index
    }

    /// Returns the current player.
    #[inline]
    pub fn current_player(&self) -> u8 {
        self.current_player
    }

    #[inline]
    pub fn set_current_player(&mut self, idx: u8) {
        self.current_player = idx;
    }

    #[inline]
    pub fn starting_player(&self) -> u8 {
        self.starting_player
    }

    #[inline]
    pub fn set_starting_player(&mut self, idx: u8) {
        self.starting_player = idx;
    }

    #[inline]
    pub fn highest_bidder(&self) -> u8 {
        self.highest_bidder
    }

    /// Returns the highest current bid.
    #[inline]
    pub fn highest_bid(&self) -> i8 {
        self.highest_bid
    }

    /// TODO: docs
    #[inline]
    pub fn set_highest_bid(&mut self, bid: i8, idx: u8) {
        self.highest_bidder = idx;
        self.highest_bid = bid;
    }

    /// Returns the inventories of all players.
    #[inline]
    pub fn inventories(&self) -> &[PlayerInventory] {
        &self.inventories[..self.num_players as usize]
    }

    pub fn stack(&self) -> &CardCollection<4> {
        &self.stack
    }
}

impl GameInfo {
    /// Creates a new `GameInfo` given a number of players. This also
    /// initialises all fields to default.
    pub fn new(num_players: u8) -> Self {
        let mut deck = Card::gem_deck();
        deck.shuffle(&mut thread_rng());
        Self {
            num_players,
            round_index: 0,
            current_player: 0,
            starting_player: 0,
            highest_bidder: num_players - 1,
            highest_bid: -1,
            inventories: Default::default(),
            deck,
            stack: Default::default(),
        }
    }

    #[inline]
    pub fn next_clockwise_player(&self, idx: u8) -> u8 {
        (idx + 1) % self.num_players
    }

    /// Sets the active player index to the next clockwise player.
    #[inline]
    pub fn next_player(&mut self) {
        self.current_player = self.next_clockwise_player(self.current_player);
    }

    /// Returns whether the game is currently in the auction phase.
    #[inline]
    pub fn is_auction_phase(&self) -> bool {
        self.stack.iter().any(|card| !card.is_null())
    }

    /// Returns whether the game is currently in the reinvestment phase.
    #[inline]
    pub fn is_reinvestment_phase(&self) -> bool {
        !self.is_auction_phase()
    }

    /// Returns whether the game has ended.
    #[inline]
    pub fn is_game_over(&self) -> bool {
        self.round_index > 5
    }

    /// Returns whether the round has just ended. This is the equivalent of
    /// checking `current_player == starting_player`.
    #[inline]
    pub fn is_end_of_round(&self) -> bool {
        self.current_player == self.starting_player
    }

    pub fn prepare_auction(&mut self) {
        let stack_sizes = match self.num_players {
            3 => [3, 3, 3, 3, 3, 3],
            _ => [4, 3, 3, 3, 3, 2],
        };
        let round_index = self.round_index as usize;
        let i: usize = stack_sizes[..round_index].iter().sum();
        let s = stack_sizes[round_index];
        self.stack.copy_from(&self.deck, i..i + s, 0..s);
        self.prepare_new_round();
    }

    pub fn prepare_new_round(&mut self) {
        self.starting_player = self.next_clockwise_player(self.highest_bidder);
        self.current_player = self.starting_player;
        self.highest_bid = -1;
    }

    pub fn prepare_reinvestment(&mut self) {
        self.starting_player = self.highest_bidder;
        self.current_player = self.starting_player;
    }

    /// Calculates the current game scores.
    pub fn scores(&self) -> GameScores {
        let mut scores = [0; 4];
        // one point for each non-leveraged gem
        for (i, inv) in self.inventories.iter().enumerate() {
            scores[i] = inv
                .iter()
                .non_leveraged()
                .gem_cards()
                .map(|card| card.archtype().num_gems() as i32)
                .sum();
        }

        // two points for each shared majority
        // three points for each owned majority
        // for gem_type in GemType::iter() {
        //     let mut players = [0; 4];
        // }
        GameScores(scores)
    }

    #[inline]
    pub fn current_inventory(&self) -> &PlayerInventory {
        &self.inventories[self.current_player as usize]
    }

    pub fn buy_card(&mut self, selected_card: u8, current_player: u8, payment_choices: CardChoice) {
        let card = self.stack.pop(selected_card as usize);
        self.inventories[current_player as usize].push_back(card);
        self.inventories[current_player as usize]
            .choose_mut(payment_choices)
            .for_each(|card| *card = card.with_leverage(true));
    }

    pub fn flip_cards(&mut self, player: usize, choices: CardChoice) {
        self.inventories[player]
            .choose_mut(choices)
            .for_each(|card| *card = card.with_leverage(!card.is_leveraged()));
    }

    pub fn reset_coin_cards(&mut self) {
        self.inventories.iter_mut().for_each(|inv| {
            inv.iter_mut()
                .coin_cards()
                .for_each(|card| *card = card.with_leverage(false))
        });
    }

    pub fn next_round(&mut self) {
        self.round_index += 1;
    }
}
