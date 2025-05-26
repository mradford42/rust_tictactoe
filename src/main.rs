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

        // Get input from current player
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let parsed_result: Result<Vec<usize>, _> = input.split(',').map(|string_val|
            string_val.trim().parse::<usize>()).collect();
        
        // Make sure passed input consists of numbers
        let nums = match parsed_result {
            Ok(list) => list,
            Err(_) => {
                println!("Error, please input numbers serperated by a comma");
                continue;
            },
        };

        // Make sure there are at least 2 numbers
        if nums.len() < 2 {
            println!("Please type at least 2 numbers!");
            continue;
        }

        // Try to make the play and ensure it is valid
        let play_succeeded = match game.play_turn(nums[0], nums[1]) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };

        // Print the board and make sure the play succeeded. If it did, check for a winner
        game.print_board();
        if play_succeeded {
            if let Some(player) = game.check_for_winner() {
                println!("Congratulations! Player {player} wins!");
                exit(0);
            }
        }
        else {
            println!("{},{} is occupied! Try again", nums[0], nums[1]);
        }
    }
}