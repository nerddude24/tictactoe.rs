use rand::{thread_rng, Rng};
use std::io::stdin;
use std::{thread, time};

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

fn lose() {
    println!("I win!")
}

fn win() {
    println!("you win...")
}

fn check_winner(cpu_choice: Choice, player_choice: Choice) {
    println!("YOU <{:?}>     VS     I <{:?}>", player_choice, cpu_choice);

    // * Artificial delay
    thread::sleep(time::Duration::from_secs(1));

    if cpu_choice == player_choice {
        println!("It's a tie!");
        return;
    }

    match cpu_choice {
        Choice::ROCK => {
            if player_choice == Choice::SCISSORS {
                lose()
            } else if player_choice == Choice::PAPER {
                win()
            }
        }
        Choice::PAPER => {
            if player_choice == Choice::ROCK {
                lose()
            } else if player_choice == Choice::SCISSORS {
                win()
            }
        }
        Choice::SCISSORS => {
            if player_choice == Choice::PAPER {
                lose()
            } else if player_choice == Choice::ROCK {
                win()
            }
        }
    }
}

fn get_player_choice() -> Result<Choice, &'static str> {
    let mut player_input = String::new();

    match stdin().read_line(&mut player_input) {
        Ok(_s) => {}
        Err(_e) => return Err("INVALID INPUT"),
    };

    match player_input.trim().to_lowercase().as_str() {
        "rock" => Ok(Choice::ROCK),
        "paper" => Ok(Choice::PAPER),
        "scissors" => Ok(Choice::SCISSORS),
        _ => {
            return Err("INVALID ANSWER (rock/paper/scissors)");
        }
    }
}

fn main() {
    let mut rng = thread_rng();

    println!("Welcome to TicTacToe.rs!");

    loop {
        println!("Your choice? (rock, paper, scissors)");

        let cpu_choice = match rng.gen_range(0..=2) {
            0 => Choice::PAPER,
            1 => Choice::ROCK,
            _ => Choice::SCISSORS,
        };

        let player_choice = match get_player_choice() {
            Err(err) => {
                println!("{}", err);
                continue;
            }
            Ok(c) => c,
        };

        check_winner(cpu_choice, player_choice);
    }
}
