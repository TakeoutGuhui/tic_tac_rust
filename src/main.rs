extern crate itertools;

mod tictactoe;
use tictactoe::{GameBoard, MoveResult, MoveError};

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
    
}
