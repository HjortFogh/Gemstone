use gemstone::*;

macro_rules! input {
    ($expl:literal => [$parse:ty], $default:expr) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        let mut buf = String::new();
        let _ = write!(stdout, $expl);
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut buf);
        #[cfg(target_os = "windows")]
        {
            buf.replace("\r\n", "")
                .split(",")
                .flat_map(|s| s.parse::<$parse>())
                .collect::<Vec<$parse>>()
        }
        #[cfg(not(target_os = "windows"))]
        {
            buf.replace("\n", "")
                .split(",")
                .flat_map(|s| s.parse::<$parse>())
                .collect::<Vec<$parse>>()
        }
    }};
    ($expl:literal => $parse:ty, $default:expr) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        let mut buf = String::new();
        let _ = write!(stdout, $expl);
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut buf);
        #[cfg(target_os = "windows")]
        {
            buf.replace("\r\n", "")
                .parse::<$parse>()
                .unwrap_or($default)
        }
        #[cfg(not(target_os = "windows"))]
        {
            buf.replace("\n", "").parse::<$parse>().unwrap_or($default)
        }
    }};
}

#[derive(Debug, Default)]
pub struct HumanBehavior {
    name: String,
}

impl HumanBehavior {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn format_cards(cards: impl AsRef<[Card]>) -> String {
        cards
            .as_ref()
            .iter()
            .enumerate()
            .map(|(i, &card)| {
                format!(
                    "{i}=>{}{}",
                    if card.is_leveraged() { "!" } else { "" },
                    GemNotation::format_card(card)
                )
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl PlayerBehavior for HumanBehavior {
    fn bid(&mut self, info: &GameInfo) -> BidValue {
        println!("\n{} ====================", self.name);
        println!("{}\n", GemNotation::from_info(info));

        println!(
            "Make a bid. The current highest bid is {}.",
            info.highest_bid()
        );
        println!("Your capital is {}.", info.my_inventory().iter().capital());

        input!("Enter your bid: " => BidValue, 0)
    }

    fn pick_card(&mut self, info: &GameInfo) -> (usize, CardChoice) {
        println!("\n{} ====================", self.name);
        println!("{}\n", GemNotation::from_info(info));

        println!(
            "Select a card. Available cards are: {}",
            Self::format_cards(info.stack())
        );
        let card = input!("Enter card: " => usize, 0);

        println!(
            "Select payment cards (you bid {}). Your inventory is: {}",
            info.highest_bid(),
            Self::format_cards(info.my_inventory())
        );
        let choice_indices = input!("Enter card choices: " => [usize], []);

        (card, CardChoice::new(&choice_indices))
    }

    fn reinvest(&mut self, info: &GameInfo) -> CardChoice {
        println!("\n{} ====================", self.name);
        println!("{}\n", GemNotation::from_info(info));

        println!(
            "Select cards to flip. Your inventory is: {}",
            Self::format_cards(info.my_inventory())
        );

        let choice_indices = input!("Enter cards: " => [usize], []);
        CardChoice::new(&choice_indices)
    }
}
