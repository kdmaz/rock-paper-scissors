use rand::Rng;
use std::io;

pub fn play_game() {
    loop {
        println!("Rock, paper, or scissors?");

        let user_choice = if let Some(choice) = get_user_choice() {
            choice
        } else {
            println!("Ya gotta pick rock, paper, or scissors!");
            continue;
        };

        let computer_choice = get_computer_choice();

        let result = get_result(&user_choice, &computer_choice);

        GameStatus {
            user_choice,
            computer_choice,
            result,
        }
        .print_game_result();

        if !play_again() {
            break;
        }
    }
}

#[derive(PartialEq)]
enum CHOICE {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    UserWin,
    ComputerWin,
    Draw,
}

struct GameStatus {
    user_choice: CHOICE,
    computer_choice: CHOICE,
    result: Result,
}

impl GameStatus {
    fn print_game_result(&self) {
        fn map_choice_to_str(choice: &CHOICE) -> &'static str {
            match choice {
                CHOICE::Rock => "rock",
                CHOICE::Paper => "paper",
                CHOICE::Scissors => "scissors",
            }
        }

        let user_choice = map_choice_to_str(&self.user_choice);
        let computer_choice = map_choice_to_str(&self.computer_choice);

        let message = format!(
            "User chose {} and computer chose {}",
            user_choice, computer_choice,
        );

        match self.result {
            Result::UserWin => println!("{}. You won!", message),
            Result::ComputerWin => println!("{}. You lost!", message),
            Result::Draw => println!("{}. Game was a draw.", message),
        };
    }
}

fn get_user_choice() -> Option<CHOICE> {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Error reading line");
    user_choice = user_choice.trim().to_lowercase();
    match user_choice.as_str() {
        "rock" => Some(CHOICE::Rock),
        "paper" => Some(CHOICE::Paper),
        "scissors" => Some(CHOICE::Scissors),
        _ => None,
    }
}

fn get_computer_choice() -> CHOICE {
    let index = rand::thread_rng().gen_range(0..=2);
    match index {
        0 => CHOICE::Rock,
        1 => CHOICE::Paper,
        2 => CHOICE::Scissors,
        _ => panic!("Computer player didn't pick rock, paper, or scissors!"),
    }
}

fn get_result(user_choice: &CHOICE, computer_choice: &CHOICE) -> Result {
    if user_choice == computer_choice {
        return Result::Draw;
    }

    match (user_choice, computer_choice) {
        (CHOICE::Rock, CHOICE::Scissors) => Result::UserWin,
        (CHOICE::Paper, CHOICE::Rock) => Result::UserWin,
        (CHOICE::Scissors, CHOICE::Paper) => Result::UserWin,
        _ => Result::ComputerWin,
    }
}

fn play_again() -> bool {
    println!("Do you want to play again? (y/n)");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Error reading line");
    ["y", "Y"].contains(&play_again.trim())
}
