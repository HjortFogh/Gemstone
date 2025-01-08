use rand::thread_rng;

use crate::{player::PlayerInventory, GemNotation};

use super::{BidValue, Card, CardChoice, CardCollection, CardIterator};

/// Represents the final game scores for each of the possible players.
#[derive(Default, Debug, Clone, Copy)]
#[allow(unused)]
pub struct GameScores([i32; 4]);

/// The `GameInfo` holds all variables necessary to represent a unqiue
/// game-state, but unlike the `Game`-struct this does not have any functions
/// to autonomously progress the state of the game and is only meant for
/// book-keeping.
#[derive(Clone)]
pub struct GameInfo {
    /// The total number of players in range `[2..5)`.
    num_players: usize,
    /// The round index in range `[0..6)` while the game is still ongoing, and
    /// `[6..)` if the game has ended.
    round_index: usize,
    /// The current player index in range `[0..num_players)`.
    current_player: usize,
    /// The index of the starting player in the current (both auction /
    /// reinvestment) round.
    starting_player: usize,
    /// The index of the highest player.
    highest_bidder: usize,
    /// Indicates whether this round has ended.
    round_over: bool,
    /// The highest bid made by any player, or `-1` if no bids have been made.
    highest_bid: BidValue,
    /// Inventories of all the players. If less than four players all non-used
    /// inventories will be filled with [`Card::NULL`].
    inventories: [PlayerInventory; 4],
    /// The current order of all cards in the deck.
    deck: CardCollection<18>,
    /// The current stack of cards. The game is in the reinvestment phase if
    /// all cards in the stack are considered [`null`](`Card::NULL`), otherwise
    /// the game is in the auction phase.
    stack: CardCollection<4>,
}

//
// Constructors
//

impl GameInfo {
    /// Creates a new `GameInfo` given a number of players. This also
    /// initialises all fields to their respective defaults, such as setting
    /// the inventories to have coins and creating a shuffled deck of 18 cards.
    pub fn new(num_players: usize) -> Self {
        let mut deck = Card::gem_deck();
        deck.shuffle(&mut thread_rng());
        Self {
            num_players,
            round_index: 0,
            current_player: 0,
            starting_player: 0,
            highest_bidder: num_players - 1,
            round_over: false,
            highest_bid: -1,
            inventories: Default::default(),
            deck,
            stack: Default::default(),
        }
    }

    pub fn from_notation(notation: GemNotation) -> Self {
        notation.to_info()
    }
}

//
// Basic getters/setters
//

impl GameInfo {
    /// Returns the number of players in range `[2..5)`.
    #[inline]
    pub fn num_players(&self) -> usize {
        self.num_players
    }

    #[inline]
    pub fn round_index(&self) -> usize {
        self.round_index
    }

    #[inline]
    pub fn increment_round_index(&mut self) {
        self.round_index += 1;
    }

    /// Returns the current player.
    #[inline]
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    #[inline]
    pub fn set_current_player(&mut self, idx: usize) {
        self.current_player = idx;
    }

    #[inline]
    pub fn starting_player(&self) -> usize {
        self.starting_player
    }

    #[inline]
    pub fn highest_bidder(&self) -> usize {
        self.highest_bidder
    }

    #[inline]
    pub fn round_over(&self) -> bool {
        self.round_over
    }

    /// Returns the highest current bid.
    #[inline]
    pub fn highest_bid(&self) -> BidValue {
        self.highest_bid
    }

    /// TODO: docs
    #[inline]
    pub fn set_highest_bid(&mut self, bid: BidValue, idx: usize) {
        self.highest_bidder = idx;
        self.highest_bid = bid;
    }

    /// Returns the inventories of all players.
    #[inline]
    pub fn inventories(&self) -> &[PlayerInventory] {
        &self.inventories[..self.num_players]
    }

    #[inline]
    pub fn inventory_at(&self, idx: usize) -> &PlayerInventory {
        &self.inventories[idx]
    }

    #[inline]
    pub fn stack(&self) -> &CardCollection<4> {
        &self.stack
    }

    #[inline]
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }
}

//
// Round/phase managing
//

impl GameInfo {
    pub fn start_step_cycle(&mut self, player_idx: usize) {
        self.round_over = false;
        self.starting_player = player_idx;
        self.current_player = player_idx;
        self.highest_bid = -1;
    }

    /// Sets the active player index to the next clockwise player.
    pub fn increment_player(&mut self) {
        self.current_player = self.next_clockwise_player(self.current_player);
        if self.current_player == self.starting_player {
            self.round_over = true;
        }
    }

    pub fn prepare_auction(&mut self) {
        let stack_sizes = match self.num_players {
            3 => [3, 3, 3, 3, 3, 3],
            _ => [4, 3, 3, 3, 3, 2],
        };
        let round_index = self.round_index;
        let idx = stack_sizes[..round_index].iter().sum::<usize>();
        let size = stack_sizes[round_index];
        self.stack.copy_from(&self.deck, idx..idx + size, 0..size);
    }

    #[inline]
    pub fn next_clockwise_player(&self, idx: usize) -> usize {
        (idx + 1) % self.num_players
    }

    /// Returns whether the game is currently in the auction phase.
    #[inline]
    pub fn is_auction_phase(&self) -> bool {
        !self.stack.is_empty()
    }

    /// Returns whether the game is currently in the reinvestment phase.
    #[inline]
    pub fn is_reinvestment_phase(&self) -> bool {
        !self.is_auction_phase()
    }

    /// Returns whether the game has ended.
    #[inline]
    pub fn game_over(&self) -> bool {
        self.round_index > 5
    }
}

//
// Miscellaneous helper functions
//

impl GameInfo {
    pub fn buy_card(&mut self, card_idx: usize, player_idx: usize, payment_choice: CardChoice) {
        let card = self.stack.pop(card_idx);
        self.inventories[player_idx].push_back(card);
        self.inventories[player_idx]
            .choose_mut(payment_choice)
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
}

//
// Scoring
//

impl GameInfo {
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

        todo!();

        // two points for each shared majority
        // three points for each owned majority
        // for gem_type in GemType::iter() {
        //     let mut players = [0; 4];
        // }
        GameScores(scores)
    }
}

//
// Behavior interface
//

impl GameInfo {
    #[inline]
    pub fn my_inventory(&self) -> &PlayerInventory {
        &self.inventories[self.current_player]
    }
}
