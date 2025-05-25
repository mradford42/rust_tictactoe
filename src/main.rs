use tictactoe::Game;
use std::{io, process::exit};

fn main() {
    let mut game: Game = Game::new();
    game.print_board();
    println!("Welcome to Tic Tac Toe!");
    println!("On your turn, enter the row and column you would like to place your symbol");
    println!("Example typing 1,2 puts your symbol at row 1, column 2");
    loop {
        println!("Player {}, where would you like to play?", game.current_player());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let nums: Vec<usize> = input.split(',').map(|string_val|
            string_val.trim().parse().expect("Please type numbers!")).collect();
        if nums.len() < 2 {
            println!("Please type at least 2 numbers!");
            continue;
        }

        let play_succeeded;
        match game.play_turn(nums[0], nums[1]) {
            Ok(result) => play_succeeded = result,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        }

        game.print_board();
        if play_succeeded {
            match game.check_for_winner() {
                Some(player) => {
                    println!("Congratulations! Player {player} wins!");
                    exit(0);
                },
                None => (),
            }
        }
        else {
            println!("{},{} is occupied! Try again", nums[0], nums[1]);
        }
    }
}