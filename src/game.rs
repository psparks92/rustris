// src/game.rs
use crate::board::CellState;
use crate::piece::{get_blocks, get_color, get_random_tetromino, GamePiece, Orientation, Position};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Vec<Vec<CellState>>,
    pub current_piece: GamePiece,
    pub running: bool,
    pub score: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: vec![vec![CellState::Empty; 10]; 20],
            current_piece: GamePiece {
                piece_type: get_random_tetromino(),
                orientation: Orientation::Up,
                position: Position { x: 4, y: 19 },
            },
            running: true,
            score: 0,
        }
    }
    pub fn is_valid(&self, piece: &GamePiece) -> bool {
        //todo - switch Position to i8 and just check for no negative values
        for block in get_blocks(piece) {
            // make sure each block is not occupied and is not out of bounds
            if block.x < 0 || block.x > 9 || block.y < 0 || block.y > 19 {
                return false;
            }
            if !matches!(
                self.board[block.y as usize][block.x as usize],
                CellState::Empty
            ) {
                return false;
            }
        }
        true
    }
    pub fn overlaps_occupied(&self, piece: &GamePiece) -> bool {
        for block in get_blocks(piece) {
            // make sure each block is not occupied and is not out of bounds
            if block.y < 20
                && !matches!(
                    self.board[block.y as usize][block.x as usize],
                    CellState::Empty
                )
            {
                return false;
            }
        }
        true
    }
    pub fn add_current_piece(&mut self) {
        let (r, g, b) = get_color(self.current_piece.piece_type);
        let mut lines_cleared = 0;
        for block in get_blocks(&self.current_piece) {
            self.board[block.y as usize][block.x as usize] = CellState::Occupied { r, g, b };
        }
        let mut y_to_check = 0;
        while y_to_check < 20 {
            if self.board[y_to_check as usize]
                .iter()
                .all(|c| matches!(c, CellState::Occupied { .. }))
            {
                lines_cleared += 1;
                self.clear_line(y_to_check);
            } else {
                y_to_check += 1;
            }
        }
        self.score += lines_cleared * lines_cleared;
    }
    fn clear_line(&mut self, row: u8) {
        for y in row..19 {
            for x in 0..10 {
                self.board[y as usize][x as usize] = self.board[(y + 1) as usize][x as usize]
            }
        }
        for x in 0..10 {
            self.board[19][x] = CellState::Empty;
        }
    }
    pub fn get_occupied_blocks(&self) -> Vec<Position> {
        let mut occupied_blocks = Vec::new();
        for y in (0..20).rev() {
            for x in 0..10 {
                match self.board[y as usize][x as usize] {
                    CellState::Occupied { .. } => occupied_blocks.push(Position { x, y }),
                    _ => (),
                }
            }
        }
        return occupied_blocks;
    }
}
