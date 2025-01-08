use crate::BidValue;

/// The gem archtype is one of 16 unique gem cards represented as a single byte:
/// `0'XXX'0'YYY`, where `XXX` and `YYY` represent the first and second gem
/// respectivly. In the case of the diamond archtype the second gem will also
/// be set as the diamond type, even though this archtype only has one gem.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GemArchtype(u8);

impl GemArchtype {
    const ARCHTYPES: [GemArchtype; 16] = [
        /*0 */ GemArchtype((GemType::Diamond as u8) << 4 | GemType::Diamond as u8),
        /*1 */ GemArchtype((GemType::Amethyst as u8) << 4 | GemType::Amethyst as u8),
        /*2 */ GemArchtype((GemType::Amethyst as u8) << 4 | GemType::Emerald as u8),
        /*3 */ GemArchtype((GemType::Amethyst as u8) << 4 | GemType::Sapphire as u8),
        /*4 */ GemArchtype((GemType::Emerald as u8) << 4 | GemType::Emerald as u8),
        /*5 */ GemArchtype((GemType::Emerald as u8) << 4 | GemType::Ruby as u8),
        /*6 */ GemArchtype((GemType::Emerald as u8) << 4 | GemType::Topaz as u8),
        /*7 */ GemArchtype((GemType::Ruby as u8) << 4 | GemType::Amethyst as u8),
        /*8 */ GemArchtype((GemType::Ruby as u8) << 4 | GemType::Ruby as u8),
        /*9 */ GemArchtype((GemType::Ruby as u8) << 4 | GemType::Topaz as u8),
        /*10*/ GemArchtype((GemType::Sapphire as u8) << 4 | GemType::Emerald as u8),
        /*11*/ GemArchtype((GemType::Sapphire as u8) << 4 | GemType::Ruby as u8),
        /*12*/ GemArchtype((GemType::Sapphire as u8) << 4 | GemType::Sapphire as u8),
        /*13*/ GemArchtype((GemType::Topaz as u8) << 4 | GemType::Amethyst as u8),
        /*14*/ GemArchtype((GemType::Topaz as u8) << 4 | GemType::Sapphire as u8),
        /*15*/ GemArchtype((GemType::Topaz as u8) << 4 | GemType::Topaz as u8),
    ];
}

impl GemArchtype {
    /// Creates a `GemArchtype` from the archtype index in range `[0..16)`.
    pub fn from_index(idx: u8) -> Self {
        Self::ARCHTYPES[idx as usize]
    }

    /// Returns the index of this `GemArchtype` in range [0..16).
    pub fn index(self) -> u8 {
        Self::ARCHTYPES
            .iter()
            .position(|&item| item == self)
            .unwrap() as u8
    }

    /// Returns the number of gems on this card.
    pub fn num_gems(self) -> u8 {
        u8::from(self.0 & 0x0f != GemType::Diamond as u8) + 1
    }

    fn get_gems(self) -> (GemType, GemType) {
        (
            GemType::from_index(self.0 >> 4),
            GemType::from_index(self.0 & 0x0f),
        )
    }

    pub fn value(self) -> BidValue {
        match self.get_gems() {
            (GemType::Diamond, _) => 2,
            (g1, g2) if g1 != g2 => 3,
            _ => 4,
        }
    }
}

/// Represents all the unqiue gems in the game.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GemType {
    Amethyst = 0,
    Diamond,
    Emerald,
    Ruby,
    Sapphire,
    Topaz,
}

impl GemType {
    pub fn from_index(idx: u8) -> Self {
        match idx {
            0 => Self::Amethyst,
            1 => Self::Diamond,
            2 => Self::Emerald,
            3 => Self::Ruby,
            4 => Self::Sapphire,
            5 => Self::Topaz,
            _ => unreachable!(),
        }
    }

    pub fn iter() -> impl Iterator<Item = GemType> {
        (0..6).map(|i| Self::from_index(i))
    }
}
