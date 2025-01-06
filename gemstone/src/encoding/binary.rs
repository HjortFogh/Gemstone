use std::fmt::Display;

use crate::game::Game;

#[derive(Default, Clone, Copy)]
struct Bits128(u128);

impl Bits128 {
    pub fn take<const N: usize>(self) -> [u8; N] {
        let mut bytes = [0; N];
        bytes.copy_from_slice(&self.0.to_be_bytes()[..N]);
        bytes
    }

    pub fn push_uint(mut self, value: impl Into<u128>, n: usize) -> Self {
        self.0 <<= n;
        self.0 |= value.into() & (1 << n - 1);
        self
    }
}

// 2 bits - number of players
// 2 bits - current player
// 2 bits - highest bidder
// 6 bits - highest bid
// 12 bits - coin cards
// 72 bits - gem cards
// 96 bits = 12 bytes

#[derive(Clone, Copy)]
pub struct GemGameBin([u8; 10]);

impl GemGameBin {
    pub fn from_game(game: &Game) -> Self {
        let mut bits = Bits128::default();
        bits = bits
            .push_uint(3, 2) // num players
            .push_uint(0, 2) // current player
            .push_uint(0, 2); // starting player
        Self(bits.take())
    }
}

impl Display for GemGameBin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
