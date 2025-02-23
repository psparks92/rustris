use crossterm::{
    cursor,
    event::{self, KeyCode},
    terminal::{self, Clear, ClearType},
    QueueableCommand,  // Replace ExecutableCommand for queuing
};
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::io::{stdout, Write}; // Added Write here
use std::time::Duration;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let width = 10;
    let height = 20;
    let mut game = Game::new();

    while game.running {
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    game.running = false;
                }
            }
        }
        if game.current_piece.position.y > 0 {
            game.current_piece.position.y -= 1;
        }

        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        let blocks = get_blocks(game.current_piece.clone());
        for y in (0..height).rev() {
            stdout.queue(cursor::MoveTo(0, (height - 1 - y) as u16))?;
            for x in 0..width {
                let is_block = blocks.iter().any(|b| b.x == x && b.y == y);
                stdout.queue(crossterm::style::Print(if is_block { "#" } else { "." }))?;
            }
        }
        stdout.flush()?;

        std::thread::sleep(Duration::from_millis(500));
    }

    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Tetromino {
    O, //position is top left
    T,
    J,
    L,
    I, //position is second from the left
    S,
    Z
}

#[derive(Debug, Clone, Copy)]
enum CellState {
    Empty,
    Occupied { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Clone)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct GamePiece {
    piece_type : Tetromino,
    orientation : Orientation,
    position: Position
}

impl GamePiece {
    fn rotate(&mut self) {
        self.orientation = match self.orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Right => Orientation::Down,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up
        }
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: u8,
    y: u8
}

#[derive(Debug, Clone)]
struct Game {
    board: Vec<Vec<CellState>>,
    current_piece : GamePiece,
    running: bool
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::rng();
        let random_x = rng.random_range(0..7);
        Game {
            board : vec![vec![CellState::Empty;10];20],
            current_piece : GamePiece {
                piece_type : get_random_tetromino(),
                orientation : Orientation::Up,
                position : Position {x:random_x, y:19}
            },
            running : true
        }
    }
    fn is_valid(&self, piece: GamePiece) -> bool {
        for block in get_blocks(piece) {
            // make sure each block is not occupied and is not out of bounds
            if !matches!(self.board[block.y as usize][block.x as usize], CellState::Empty) {
                return false;
            }
            if block.x < 0 || block.x > 9 || block.y < 0 {
                return false;
            }
        }
        true
    }
    fn add_piece(&mut self, piece: GamePiece) {
        let (r, g, b) = get_color(piece.piece_type);
        for block in get_blocks(piece) {
            self.board[block.y as usize][block.x as usize] = CellState::Occupied {r, g, b};
        }
    }
}
fn get_color(piece_type: Tetromino) -> (u8, u8, u8) {
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

fn get_random_tetromino() -> Tetromino {
    let mut rng = rand::rng();
    let types = [Tetromino::O, Tetromino::T, Tetromino::J, Tetromino::L, Tetromino::I, Tetromino::S, Tetromino::Z];
    *types.choose(&mut rng).unwrap()
}

fn get_blocks(piece: GamePiece) -> Vec<Position> {
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
