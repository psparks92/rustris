// src/main.rs
mod game;
mod piece;
mod board;

use crossterm::{
    cursor,
    event::{self, KeyCode},
    terminal::{self, Clear, ClearType},
    QueueableCommand,  // Replace ExecutableCommand for queuing
};
use std::io::{stdout, Write}; // Added Write here
use std::time::Duration;
use game::Game;
use piece::get_blocks;
use crate::piece::GamePiece;

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
        let mut proposed_piece = game.current_piece.clone();
        proposed_piece.position.y -= 1;
        if game.is_valid(&proposed_piece) {
            game.current_piece = proposed_piece;
        }
        else {
            game.add_current_piece();
            game.current_piece = GamePiece::new_random();
            // create new piece
        }

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
        stdout.flush()?;

        std::thread::sleep(Duration::from_millis(500));
    }

    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}





