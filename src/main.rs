extern crate tic_tac_rust;

use tic_tac_rust::{GameBoard, MoveResult, MoveErrorKind, NUM_TILES};

fn main() {

    let mut gameboard = GameBoard::new(String::from("Hampus"), String::from("Kalle"));
    
    loop {
        print_turn(&gameboard);
        
        let number = get_input(1, NUM_TILES);
        let move_result = gameboard.player_move(number-1);

        match move_result {
            Ok(MoveResult::Win(name)) => { println!("{} is the winner! \n{}", name, gameboard); break; },
            Ok(MoveResult::Tie) => { println!("Game tied"); break; },
            Ok(MoveResult::Continue) => (),

            Err(MoveErrorKind::TileTaken) => println!("Tile already taken!"),
            Err(MoveErrorKind::OutOfBounds) => println!("Out of bounds!") 
        }
    }

    fn print_turn(gameboard: &GameBoard) {
        let current_player = gameboard.current_player();
        println!("\n{}", gameboard);
        println!("Current player is: {} ({})", current_player.name, current_player.tile);

    }

    fn get_input(limit_low: usize, limit_high: usize) -> usize {
        use std::io::{stdin, stdout, Write};
        let mut input = String::new();
        let mut result: usize;
        
        loop {
            input.clear();
            print!("Please enter a tile (1-9): ");
            let _ = stdout().flush();
            stdin().read_line(&mut input).expect("Did not enter a correct string");
            let input = input.trim();
            let input = input.parse::<usize>();
            if input.is_err() { println!("Not a valid number"); continue }
            result = input.unwrap();
            if result < limit_low || result > limit_high { println!("Not within boundaries"); continue }
            break;
        }
        result
    }
}
