// src/main.rs
mod board;
mod game;
mod piece;

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
use std::io::prelude::*;
use std::io::{stdin, stdout, Write}; // Added Write here
use std::time::Duration;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.contains(&"--test".to_string());
    let mut stdin = stdin();
    //todo: test mode logs instead of renders
    //bug: pieces are saved to the left edge when added to the board
    if !is_test_mode {
        terminal::enable_raw_mode()?;
    }
    let mut stdout = stdout();
    let width = 10;
    let height = 20;
    let mut game = Game::new();
    let drop_interval = Duration::from_millis(500);
    let mut last_drop = Instant::now();
    let move_cooldown = Duration::from_millis(200);
    let mut last_input = Instant::now();

    while game.running {
        let now = Instant::now();
        if event::poll(Duration::from_millis(1))? {
            // && now - last_input >= move_cooldown {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            game.running = false;
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            if let Some(proposed_piece) =
                                game.current_piece.move_piece(Direction::Left)
                            {
                                if game.is_valid(&proposed_piece) {
                                    game.current_piece = proposed_piece;
                                }
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            if let Some(proposed_piece) =
                                game.current_piece.move_piece(Direction::Right)
                            {
                                if game.is_valid(&proposed_piece) {
                                    game.current_piece = proposed_piece;
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(proposed_piece) =
                                game.current_piece.move_piece(Direction::Down)
                            {
                                if game.is_valid(&proposed_piece) {
                                    game.current_piece = proposed_piece;
                                }
                            }
                        }
                        KeyCode::Char(' ') | KeyCode::Char('k') => {
                            if let Some(proposed_piece) =
                                game.current_piece.move_piece(Direction::Rotate)
                            {
                                if game.is_valid(&proposed_piece) {
                                    game.current_piece = proposed_piece;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                last_input = now;
            }
        }
        let now = Instant::now();
        if now - last_drop >= drop_interval {
            last_drop = now;
            if let Some(proposed_piece) = game.current_piece.move_piece(Direction::Down) {
                if game.is_valid(&proposed_piece) {
                    game.current_piece = proposed_piece;
                } else {
                    let lines_cleared = game.add_current_piece();
                    if is_test_mode {
                        println!("NEW PIECE");
                    }
                    game.current_piece = GamePiece::new_random();
                    if !game.overlaps_occupied(&game.current_piece.clone()) {
                        game.running = false;
                    }
                }
            } else {
                game.add_current_piece();
                game.current_piece = GamePiece::new_random();
                let lines_cleared = game.add_current_piece();
                if is_test_mode {
                    println!("NEW PIECE");
                }
                if !game.overlaps_occupied(&game.current_piece.clone()) {
                    game.running = false;
                }
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
            let piece_blocks = get_blocks(&game.current_piece);
            let game_blocks = game.get_occupied_blocks();
            for y in (0..height).rev() {
                stdout.queue(cursor::MoveTo(0, (height - 1 - y) as u16))?;
                for x in 0..width {
                    let is_block = piece_blocks.iter().any(|b| b.x == x && b.y == y);
                    let is_occupied = game_blocks.iter().any(|b| b.x == x && b.y == y);
                    stdout.queue(crossterm::style::Print(if is_block {
                        "#"
                    } else if is_occupied {
                        "%"
                    } else {
                        "."
                    }))?;
                }
                // print current piece type
                // stdout.queue(crossterm::style::Print(
                //     game.current_piece.piece_type.print(),
                // ))?;
            }
            stdout.queue(crossterm::style::Print(format!("   Score: {}", game.score)))?;
            stdout.flush()?;
        }

        std::thread::sleep(Duration::from_millis(1));
    }
    stdout.queue(cursor::MoveTo(0, (height - 1 + 1) as u16))?;
    stdout.queue(crossterm::style::Print(format!(
        "Game over! Score: {}",
        game.score
    )))?;
    stdout.flush()?;
    let _ = stdin.read(&mut [0u8]).unwrap();
    if !is_test_mode {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.flush()?;
        terminal::disable_raw_mode()?;
    }
    Ok(())
}
