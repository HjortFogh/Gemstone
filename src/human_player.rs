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
        buf.replace("\r\n", "")
            .split(",")
            .flat_map(|s| s.parse::<$parse>())
            .collect::<Vec<$parse>>()
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
        println!("{}", GemNotation::from_info(info));
        input!("make a bid: " => i8, 0)
    }

    fn pick_card(&mut self, _info: &GameInfo) -> (u8, CardChoice) {
        (0, CardChoice::NONE)
    }

    fn reinvest(&mut self, _info: &GameInfo) -> CardChoice {
        CardChoice::NONE
    }
}
