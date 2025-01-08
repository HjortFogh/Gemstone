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
pub struct HumanBehavior;

impl PlayerBehavior for HumanBehavior {
    fn bid(&mut self, info: &GameInfo) -> i8 {
        println!("\n\n");
        println!("{}", GemNotation::from_info(info));
        input!("make a bid: " => i8, 0)
    }

    fn pick_card(&mut self, info: &GameInfo) -> (u8, CardChoice) {
        println!("\n\n");
        println!("pick card");
        println!("stack: {:?}", info.stack());
        let card = input!("select card to buy: " => u8, 0);
        println!("inventory: {:?}", info.current_inventory());
        let choice_indices = input!("select payment cards: " => [usize], []);
        (card, CardChoice::new(&choice_indices))
    }

    fn reinvest(&mut self, info: &GameInfo) -> CardChoice {
        println!("\n\n");
        println!("inventory: {:?}", info.current_inventory());
        let choice_indices = input!("select cards to flip: " => [usize], []);
        println!("choice indices: {choice_indices:?}");
        dbg!(CardChoice::new(&choice_indices))
    }
}
