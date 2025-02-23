// src/piece.rs
use rand::Rng;
use rand::prelude::IndexedRandom;


#[derive(Debug, Clone, Copy)]
pub enum Tetromino {
    O, //position is top left
    T,
    J,
    L,
    I, //position is second from the left
    S,
    Z
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub struct GamePiece {
    pub piece_type : Tetromino,
    pub orientation : Orientation,
    pub position: Position
}

impl GamePiece {
    pub fn rotate(&mut self) {
        self.orientation = match self.orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Right => Orientation::Down,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up
        }
    }
    fn move_down(&mut self){
        self.position.y += 1
    }
    pub fn new_random() -> GamePiece {
        let mut rng = rand::rng();
        let random_x = rng.random_range(1..8);
        GamePiece {
            piece_type: get_random_tetromino(),
            orientation: Orientation::Up,
            position: Position{x:random_x, y:19}
        }
    }
}


#[derive(Debug, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8
}

pub fn get_color(piece_type: Tetromino) -> (u8, u8, u8) {
        match piece_type {
            Tetromino::O => (255, 255, 0),   // Yellow
            Tetromino::T => (128, 0, 128), // Purple
            Tetromino::J => (0, 0, 255),     // Blue
            Tetromino::L => (255, 165, 0),   // Orange
            Tetromino::I => (0, 255, 255),   // Cyan
            Tetromino::S => (0, 255, 0),     // Green
            Tetromino::Z => (255, 0, 0),     // Red
        }
}

pub fn get_random_tetromino() -> Tetromino {
    let mut rng = rand::rng();
    let types = [Tetromino::O, Tetromino::T, Tetromino::J, Tetromino::L, Tetromino::I, Tetromino::S, Tetromino::Z];
    *types.choose(&mut rng).unwrap()
}

pub fn get_blocks(piece: &GamePiece) -> Vec<Position> {
    let x = piece.position.x;
    let y = piece.position.y;
    match piece.piece_type {
        Tetromino::O => vec![
            Position{x,y},
            Position{x:x+1,y},
            Position{x,y:y+1},
            Position{x:x+1,y:y+1},],
        Tetromino::T => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y},Position{x,y:y-1},Position{x:x+1,y}],
        Orientation::Right => vec![Position{x,y},Position{x,y:y+1},Position{x,y:y-1},Position{x:x+1,y}],
        Orientation::Down => vec![Position{x,y},Position{x,y:y+1},Position{x,y:y-1},Position{x:x-1,y}],
        Orientation::Left => vec![Position{x,y},Position{x:x-1,y},Position{x,y:y-1},Position{x:x+1,y}],
        },
        Tetromino::J => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y},Position{x:x-1,y:y+1},Position{x:x+1,y}],
        Orientation::Right => vec![Position{x,y},Position{x,y:y-1},Position{x,y:y+1},Position{x:x+1,y:y+1}],
        Orientation::Down => vec![Position{x,y},Position{x:x-1,y},Position{x:x+1,y:y-1},Position{x:x+1,y}],
        Orientation::Left => vec![Position{x,y},Position{x:x-1,y:y-1},Position{x,y:y-1},Position{x,y:y+1}],
        },
        Tetromino::L => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y},Position{x:x-1,y:y+1},Position{x:x+1,y}],
        Orientation::Right => vec![Position{x,y},Position{x,y:y-1},Position{x,y:y+1},Position{x:x+1,y:y+1}],
        Orientation::Down => vec![Position{x,y},Position{x:x-1,y},Position{x:x-1,y:y-1},Position{x:x+1,y}],
        Orientation::Left => vec![Position{x,y},Position{x:x-1,y:y+1},Position{x,y:y-1},Position{x,y:y+1}],
        },
        Tetromino::I => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y},Position{x:x+1,y},Position{x:x+2,y}],
        Orientation::Right => vec![Position{x:x+1,y:y-1},Position{x:x+1,y},Position{x:x+1,y:y+1},Position{x:x+1,y:y+2}],
        Orientation::Down => vec![Position{x:x-1,y:y-1},Position{x,y:y-1},Position{x:x+1,y:y-1},Position{x:x+2,y:y-1}],
        Orientation::Left => vec![Position{x:x+1,y:y-1},Position{x:x+1,y},Position{x:x+1,y:y+1},Position{x:x+1,y:y+2}],
        },
        Tetromino::S => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y},Position{x,y:y+1},Position{x:x+1,y:y+1}],
        Orientation::Right => vec![Position{x,y},Position{x,y:y+1},Position{x:x+1,y},Position{x:x+1,y:y-1}],
        Orientation::Down => vec![Position{x,y},Position{x:x-1,y:y-1},Position{x,y:y-1},Position{x:x+1,y}],
        Orientation::Left => vec![Position{x,y},Position{x:x-1,y:y+1},Position{x:x-1,y},Position{x,y:y-1}],
        },
        Tetromino::Z => match piece.orientation {
        Orientation::Up => vec![Position{x,y},Position{x:x-1,y:y+1},Position{x,y:y+1},Position{x:x+1,y}],
        Orientation::Right => vec![Position{x,y},Position{x,y:y-1},Position{x:x+1,y:y+1},Position{x:x+1,y:y}],
        Orientation::Down => vec![Position{x,y},Position{x:x-1,y},Position{x,y:y-1},Position{x:x+1,y:y-1}],
        Orientation::Left => vec![Position{x,y},Position{x:x-1,y:y-1},Position{x:x-1,y},Position{x,y:y+1}],
        },
    }
}
