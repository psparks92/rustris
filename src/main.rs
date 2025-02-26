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
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 900.0))
        .build()?;
    let game = Game::new();
    run(ctx, event_loop, game)
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.running && ctx.time.check_update_time(self.speed) {
            if let Some(new_piece) = self.current_piece.move_piece(Direction::Down) {
                if self.is_valid(&new_piece) {
                    self.current_piece = new_piece.clone();
                } else {
                    self.add_current_piece();
                    self.current_piece = GamePiece::new_random();
                    if !self.overlaps_occupied(&self.current_piece) {
                        self.running = false;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let block_size = 40.0;
        let piece_blocks = get_blocks(&self.current_piece);
        let outline_color = Color::from_rgb(200, 200, 200); // Light gray

        // Draw board outline
        let board_rect = Rect::new(0.0, 0.0, 10.0 * block_size, 20.0 * block_size);
        let board_outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0), // 2px thick
            board_rect,
            outline_color,
        )?;
        canvas.draw(&board_outline, DrawParam::default().dest([0.0, 0.0])); // Top-left corner
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
        // Score text
        let score_text = graphics::Text::new(
            graphics::TextFragment::new(format!("Score: {}, Speed: {}", self.score, self.speed))
                .scale(graphics::PxScale::from(32.0)), // ~2x default size
        );
        canvas.draw(
            &score_text,
            DrawParam::default().dest([480.0, 20.0]).color(Color::WHITE),
        );

        if !self.running {
            let game_over = graphics::Text::new(
                graphics::TextFragment::new(format!("Game over! Score: {}", self.score))
                    .scale(graphics::PxScale::from(48.0)), // ~2x default size
            );
            canvas.draw(
                &game_over,
                DrawParam::default()
                    .dest([520.0, 300.0])
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
