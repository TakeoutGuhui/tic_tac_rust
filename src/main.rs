extern crate tic_tac_rust;

use tic_tac_rust::{GameBoard, MoveResult, MoveErrorKind, NUM_TILES};

fn main() {
    let mut gameboard = GameBoard::new(String::from("Hampus"), String::from("Kalle"));
    
    loop {
        print_turn(&gameboard);
        
        let number = get_num_input(1, NUM_TILES);
        let move_result = gameboard.player_move(number-1);

        match move_result {
            Ok(MoveResult::Win(name)) => { println!("\n{} is the winner!", name); print_board(&gameboard); break; },
            Ok(MoveResult::Tie) => { println!("Game tied"); break; },
            Ok(MoveResult::Continue) => (),

            Err(MoveErrorKind::TileTaken) => println!("Tile already taken!"),
            Err(MoveErrorKind::OutOfBounds) => println!("Out of bounds!") 
        }
        println!();
    }

    fn print_turn(gameboard: &GameBoard) {
        let current_player = gameboard.current_player();
        print_board(&gameboard);
        println!("Current player is: {} ({})", current_player.name, current_player.tile);

    }

    fn get_num_input(limit_low: usize, limit_high: usize) -> usize {
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

    fn print_board(gameboard: &GameBoard) {
        let tiles = &gameboard.tiles;
        println!(" {} | {} | {}", tiles[0], tiles[1], tiles[2]);
        println!("===========");
        println!(" {} | {} | {}", tiles[3], tiles[4], tiles[5]);
        println!("===========");
        println!(" {} | {} | {}", tiles[6], tiles[7], tiles[8]);
    }
}
