extern crate tic_tac_rust;

use tic_tac_rust::{GameBoard, MoveResult, MoveError};

fn main() {
    use std::io::{stdin,stdout,Write};

    let mut gameboard = GameBoard::new(String::from("Hampus"), String::from("Kalle"));
    let mut input = String::new();
    loop {
        input.clear();
        println!("{}", gameboard);
        println!("Current player is: {} ({:?})", gameboard.current_player().name, gameboard.current_player().tile);
        print!("Please enter a tile (1-9): ");
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        let input = input.trim();
        let number = match input.parse::<usize>(){
            Ok(num) => num,
            Err(_) => { println!("Input must be a number!"); continue },
        };
        let move_result = gameboard.player_move(number-1);
        match move_result {
            Ok(MoveResult::Win(name)) => { println!("{} is the winner! \n{}", name, gameboard); break; },
            Ok(MoveResult::Tie) => { println!("Game tied"); break; },
            Ok(MoveResult::Continue) => (),

            Err(MoveError::TileTaken) => println!("Tile already taken!"),
            Err(MoveError::OutOfBounds) => println!("Out of bounds!") 
        }
    }

    fn print_turn(gameboard: &GameBoard) {
        let current_player = gameboard.current_player();
        println!("{}", gameboard);
        println!("Current player is: {} ({})", current_player.name, current_player.tile);

    }

    fn get_input() -> usize {
        use std::io::{Stdin, stdout, Write};
        2
    }
    
}
