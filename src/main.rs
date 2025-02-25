// src/main.rs
mod board;
mod game;
mod piece;

use crate::piece::{Direction, GamePiece};
use game::Game;
use ggez::input::keyboard::KeyInput;
use ggez::input::keyboard::*;
use ggez::{event::run, graphics::Canvas, graphics::DrawParam};
use ggez::{graphics::Color, graphics::Rect, *};
use piece::{get_blocks, get_color};
//use std::env;
//use std::io::prelude::*;
//use std::io::{stdin, stdout, Write}; // Added Write here
//use std::time::Duration;
//use std::time::Instant;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("rustris", "Peter Sparks")
        .window_setup(ggez::conf::WindowSetup::default().title("Rustris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let game = Game::new();
    run(ctx, event_loop, game)
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(500) {
            if let Some(new_piece) = self.current_piece.move_piece(Direction::Down) {
                if self.is_valid(&new_piece) {
                    self.current_piece = new_piece.clone();
                } else {
                    self.add_current_piece();
                    self.current_piece = GamePiece::new_random();
                    if !self.is_valid(&self.current_piece) {
                        self.running = false;
                    }
                }
            } else {
                self.running = false;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let block_size = 20;
        let piece_blocks = get_blocks(&self.current_piece);
        for y in 0..20 {
            for x in 0..10 {
                let (piece_r, piece_g, piece_b) = get_color(self.current_piece.piece_type);
                let piece_color = Color::from_rgb(piece_r, piece_g, piece_b);
                let is_block = piece_blocks.iter().any(|b| b.x == x && b.y == y);
                let color = if is_block {
                    piece_color
                } else {
                    match self.board[y as usize][x as usize] {
                        board::CellState::Occupied { r, g, b } => Color::from_rgb(r, g, b),
                        board::CellState::Empty => Color::BLACK,
                    }
                };
                let rect = Rect::new(
                    x as f32 * (block_size as f32),
                    (19 - y) as f32 * (block_size as f32), // Flip y (0 at bottom)
                    block_size as f32,
                    block_size as f32,
                );
                canvas.draw(
                    &graphics::Quad,
                    DrawParam::default().dest_rect(rect).color(color),
                );
            }
        }
        // Draw score (example)
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        canvas.draw(
            &score_text,
            DrawParam::default().dest([10.0, 10.0]).color(Color::WHITE),
        );

        if !self.running {
            let game_over = graphics::Text::new(format!("Game Over\nScore: {}", self.score));
            canvas.draw(
                &game_over,
                DrawParam::default()
                    .dest([150.0, 300.0])
                    .color(Color::WHITE),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Q => ctx.request_quit(),
                KeyCode::Right => {
                    if let Some(proposed) = self.current_piece.move_piece(Direction::Right) {
                        if self.is_valid(&proposed) {
                            self.current_piece = proposed;
                        }
                    }
                }
                KeyCode::Left => {
                    if let Some(proposed) = self.current_piece.move_piece(Direction::Left) {
                        if self.is_valid(&proposed) {
                            self.current_piece = proposed;
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(proposed) = self.current_piece.move_piece(Direction::Down) {
                        if self.is_valid(&proposed) {
                            self.current_piece = proposed;
                        }
                    }
                }
                KeyCode::Space => {
                    if let Some(proposed) = self.current_piece.move_piece(Direction::Rotate) {
                        if self.is_valid(&proposed) {
                            self.current_piece = proposed;
                        }
                    }
                }
                KeyCode::Return if !self.running => ctx.request_quit(), // Exit on game over
                _ => {}
            }
        }
        Ok(())
    }
}
