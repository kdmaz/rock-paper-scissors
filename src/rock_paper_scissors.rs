use rand::Rng;
use std::{fmt, io};

pub fn play() {
    fn play_again() -> bool {
        println!("\nDo you want to play again? (y/n)");
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();
        input_buffer.trim() == "y"
    }

    let mut stats = Stats::new();

    loop {
        let game = Game::new();
        stats.update(&game.game_result);

        println!("{}", stats);
        println!("{}", game);

        if !play_again() {
            break;
        }
    }
}

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let choice = match self {
            Choice::Rock => "rock",
            Choice::Paper => "paper",
            Choice::Scissors => "scissors",
        };
        write!(f, "{}", choice)
    }
}

struct UserChoice(Choice);
impl UserChoice {
    fn get() -> Self {
        loop {
            println!("Rock, paper, or scissors?");
            let mut input_buffer = String::new();
            io::stdin().read_line(&mut input_buffer).unwrap();
            if let Ok(choice) = input_buffer.as_str().try_into() {
                return choice;
            } else {
                println!("Ya gotta pick rock, paper, or scissors!");
                continue;
            }
        }
    }
}
impl TryFrom<&str> for UserChoice {
    type Error = ();
    fn try_from(choice: &str) -> Result<Self, Self::Error> {
        match choice.trim().to_lowercase().as_str() {
            "rock" => Ok(Self(Choice::Rock)),
            "paper" => Ok(Self(Choice::Paper)),
            "scissors" => Ok(Self(Choice::Scissors)),
            _ => Err(()),
        }
    }
}

struct ComputerChoice(Choice);
impl ComputerChoice {
    fn get() -> Self {
        rand::thread_rng().gen_range(0..3).into()
    }
}
impl From<u8> for ComputerChoice {
    fn from(choice: u8) -> Self {
        match choice {
            0 => Self(Choice::Rock),
            1 => Self(Choice::Paper),
            2 => Self(Choice::Scissors),
            _ => Self(Choice::Rock),
        }
    }
}

enum GameResult {
    Win,
    Loss,
    Draw,
}
impl GameResult {
    fn get(user_choice: &UserChoice, computer_choice: &ComputerChoice) -> Self {
        match (&user_choice.0, &computer_choice.0) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => GameResult::Win,
            (_, _) if &user_choice.0 == &computer_choice.0 => GameResult::Draw,
            (_, _) => GameResult::Loss,
        }
    }
}
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Win => "You won!",
            Self::Loss => "You lost!",
            Self::Draw => "The game is a draw.",
        };
        write!(f, "{}", message)
    }
}

struct Stats {
    wins: i32,
    losses: i32,
    draws: i32,
}
impl Stats {
    fn new() -> Self {
        Stats {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn update(&mut self, game_result: &GameResult) {
        match game_result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.losses += 1,
            GameResult::Draw => self.draws += 1,
        };
    }
}
impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stats = format!(
            "\nWins: {}\nLosses: {}\nDraws: {}",
            self.wins, self.losses, self.draws
        );
        write!(f, "{}", stats)
    }
}

struct Game {
    user_choice: UserChoice,
    computer_choice: ComputerChoice,
    game_result: GameResult,
}
impl Game {
    fn new() -> Game {
        let user_choice = UserChoice::get();
        let computer_choice = ComputerChoice::get();
        let game_result = GameResult::get(&user_choice, &computer_choice);
        Game {
            user_choice,
            computer_choice,
            game_result,
        }
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nYou chose {} and computer chose {}. {}",
            self.user_choice.0, self.computer_choice.0, self.game_result
        )
    }
}
