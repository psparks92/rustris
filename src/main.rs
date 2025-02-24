// src/main.rs
mod board;
mod game;
mod piece;

use crate::board::CellState;
use crate::piece::{Direction, GamePiece};
use crossterm::{
    cursor,
    event::{self, KeyCode},
    terminal::{self, Clear, ClearType},
    QueueableCommand, // Replace ExecutableCommand for queuing
};
use game::Game;
use piece::get_blocks;
use std::env;
use std::io::{stdout, Write}; // Added Write here
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.contains(&"--test".to_string());
    //todo: test mode logs instead of renders
    //bug: pieces are saved to the left edge when added to the board
    if !is_test_mode {
        terminal::enable_raw_mode()?;
    }
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
        if let Some(proposed_piece) = game.current_piece.move_piece(Direction::Down) {
            if game.is_valid(&proposed_piece) {
                game.current_piece = proposed_piece;
            } else {
                game.add_current_piece();
                game.current_piece = GamePiece::new_random();
                if !game.overlaps_occupied(&game.current_piece.clone()) {
                    game.running = false;
                }
            }
        } else {
            game.add_current_piece();
            game.current_piece = GamePiece::new_random();
            if !game.overlaps_occupied(&game.current_piece.clone()) {
                game.running = false;
            }
        }

        if is_test_mode {
            // simply print out which blocks are occupied on the game board
            println!("----------------------------------------");
            println!("Occupied blocks:");
            for block in game.get_occupied_blocks() {
                block.print();
            }
            // simply print out which blocks are in the current piece
            println!("Current piece blocks:");
            for block in get_blocks(&game.current_piece) {
                block.print();
            }
        } else {
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(cursor::MoveTo(0, 0))?;
            let blocks = get_blocks(&game.current_piece);
            for y in (0..height).rev() {
                stdout.queue(cursor::MoveTo(0, (height - 1 - y) as u16))?;
                for x in 0..width {
                    let is_block = blocks.iter().any(|b| b.x == x && b.y == y);
                    stdout.queue(crossterm::style::Print(if is_block { "#" } else { "." }))?;
                }
            }
            for y in (0..height).rev() {
                stdout.queue(cursor::MoveTo(0, (height - 1 - y) as u16))?;
                for x in 0..width {
                    if !matches!(game.board[y as usize][x as usize], CellState::Empty) {
                        stdout.queue(crossterm::style::Print("%"))?;
                    }
                }
            }
            stdout.flush()?;
        }

        std::thread::sleep(Duration::from_millis(200));
    }

    if !is_test_mode {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.flush()?;
        terminal::disable_raw_mode()?;
    }
    Ok(())
}
