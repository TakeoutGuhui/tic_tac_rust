use itertools::Itertools;
use std::fmt;

const NUM_TILES: usize = 9;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    X,
    O,
    Empty,
}

pub enum MoveResult {
    Win(String),
    Tie,
    Continue,
}

pub enum MoveError {
    OutOfBounds,
    TileTaken,
}

pub struct Player {
    pub name: String,
    pub tile: Tile,  
}

pub struct GameBoard<> {
    tiles: Vec<Tile>,
    player1: Player,
    player2: Player, 
    current_player: bool,
}

impl GameBoard {
    pub fn new(player1: String, player2: String) -> GameBoard {
        let mut tiles = Vec::with_capacity(NUM_TILES);
        for _ in 0..NUM_TILES {
            tiles.push(Tile::Empty);
        }
        let player1 = Player {name: player1, tile: Tile::X};
        let player2 = Player {name: player2, tile: Tile::O};
        GameBoard {tiles, player1, player2, current_player: true}
    }

    pub fn player_move(&mut self, moves: usize) -> Result<MoveResult, MoveError> {
        if moves >= NUM_TILES {
            return Err(MoveError::OutOfBounds);
        }

        if self.tiles[moves] != Tile::Empty {
            return Err(MoveError::TileTaken);
        }

        self.tiles[moves] = self.current_player().tile;
        
        if self.check_victory(self.current_player()){
            return Ok(MoveResult::Win(self.current_player().name.clone()));
        }

        if self.check_tie() { 
            return Ok(MoveResult::Tie);
        }

        self.current_player = !self.current_player;
        Ok(MoveResult::Continue)
    }

    pub fn current_player(&self) -> &Player {
        if self.current_player {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn check_tie(&self) -> bool {
        self.tiles.iter().filter(|tile| **tile != Tile::Empty).count() == 9
    }

    fn check_victory(&self, player: &Player) -> bool {
        let magic_cube = vec![8, 3, 4, 1, 5, 9, 6, 7, 2];
        let wins = self.tiles.iter().zip(magic_cube.into_iter()) 
            .filter(|(tile, _)| **tile == player.tile) 
            .map(|(_, num)| num) 
            .combinations(3) 
            .any(|combination| combination.iter().sum::<u32>() == 15); 
        wins
    }
}

//TODO: Make a better solution
impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dis = String::new();
        for (number, tile) in self.tiles.iter().enumerate() {
            let mut num_string = number.to_string();
            num_string.push_str(" ");
            let disp_tile = match tile {
                Tile::X => "X ",
                Tile::O => "O ",
                Tile::Empty => &num_string,
            };
            dis.push_str(disp_tile);
            if (number + 1)  % 3 == 0 { 
                dis.push_str("\n")
            }
        }
        write!(f, "{}", dis)
    }
}
