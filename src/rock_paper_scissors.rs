use rand::Rng;
use std::{fmt, io};

pub fn play_game() {
    loop {
        println!("Rock, paper, or scissors?");

        let user_choice = if let Ok(choice) = get_user_choice() {
            choice
        } else {
            println!("Ya gotta pick rock, paper, or scissors!");
            continue;
        };

        let computer_choice = if let Ok(choice) = get_computer_choice() {
            choice
        } else {
            panic!("Computer didn't pick a valid choice!");
        };

        let result = GameResult::get_result(&user_choice, &computer_choice);

        print_game_result(user_choice, computer_choice, result);

        if !play_again() {
            break;
        }
    }
}

#[derive(PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Into<String> for Choice {
    fn into(self) -> String {
        match self {
            Choice::Rock => "rock".to_string(),
            Choice::Paper => "paper".to_string(),
            Choice::Scissors => "scissors".to_string(),
        }
    }
}

impl TryFrom<&str> for Choice {
    type Error = ();
    fn try_from(choice: &str) -> Result<Self, Self::Error> {
        match choice {
            "rock" => Ok(Choice::Rock),
            "paper" => Ok(Choice::Paper),
            "scissors" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for Choice {
    type Error = ();
    fn try_from(choice: i32) -> Result<Self, Self::Error> {
        match choice {
            0 => Ok(Choice::Rock),
            1 => Ok(Choice::Paper),
            2 => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

enum GameResult {
    UserWin,
    ComputerWin,
    Draw,
}

impl GameResult {
    fn get_result(user_choice: &Choice, computer_choice: &Choice) -> GameResult {
        match (user_choice, computer_choice) {
            (Choice::Rock, Choice::Scissors) => GameResult::UserWin,
            (Choice::Paper, Choice::Rock) => GameResult::UserWin,
            (Choice::Scissors, Choice::Paper) => GameResult::UserWin,
            (_, _) if user_choice == computer_choice => GameResult::Draw,
            (_, _) => GameResult::ComputerWin,
        }
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            GameResult::UserWin => "You won!",
            GameResult::ComputerWin => "You lost!",
            GameResult::Draw => "Game was a draw.",
        };
        write!(f, "{}", message)
    }
}

fn print_game_result(user_choice: Choice, computer_choice: Choice, result: GameResult) {
    let user_choice: String = user_choice.into();
    let computer_choice: String = computer_choice.into();
    println!(
        "User chose {} and computer chose {}. {}",
        user_choice, computer_choice, result
    );
}

fn get_user_choice() -> Result<Choice, ()> {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Error reading line");
    let user_choice = &*user_choice.trim().to_lowercase();
    Choice::try_from(user_choice)
}

fn get_computer_choice() -> Result<Choice, ()> {
    let rand_index = rand::thread_rng().gen_range(0..=2);
    Choice::try_from(rand_index)
}

fn play_again() -> bool {
    println!("Do you want to play again? (y/n)");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Error reading line");
    play_again.trim().contains("y")
}
