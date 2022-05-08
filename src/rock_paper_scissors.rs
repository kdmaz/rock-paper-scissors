use rand::Rng;
use std::{fmt, io};

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
            let mut user_choice = String::new();
            io::stdin().read_line(&mut user_choice).unwrap();
            if let Ok(choice) = user_choice.as_str().try_into() {
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
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Win => "You won!",
            Self::Loss => "You lost!",
            Self::Draw => "The game was a draw.",
        };
        write!(f, "{}", message)
    }
}

pub struct Game {
    user_choice: UserChoice,
    computer_choice: ComputerChoice,
}
impl Game {
    pub fn new() {
        loop {
            Self {
                user_choice: UserChoice::get(),
                computer_choice: ComputerChoice::get(),
            }
            .print_result();

            if !Self::play_again() {
                break;
            }
        }
    }

    fn print_result(&self) {
        println!(
            "You chose {} and the computer chose {}. {}",
            self.user_choice.0,
            self.computer_choice.0,
            self.get_game_result()
        );
    }

    fn get_game_result(&self) -> GameResult {
        let user_choice = &self.user_choice.0;
        let computer_choice = &self.computer_choice.0;
        match (user_choice, computer_choice) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => GameResult::Win,
            (_, _) if user_choice == computer_choice => GameResult::Draw,
            (_, _) => GameResult::Loss,
        }
    }

    fn play_again() -> bool {
        println!("Do you want to play again? (y/n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).unwrap();
        play_again.contains("y")
    }
}
