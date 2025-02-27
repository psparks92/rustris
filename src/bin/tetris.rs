// src/main.rs
use ggez::context::ContextBuilder;
use ggez::event::run;
use ggez::GameResult;
use rustris::game::Game;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("rustris", "Peter Sparks")
        .window_setup(ggez::conf::WindowSetup::default().title("Rustris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 900.0))
        .build()?;
    let game = Game::new();
    run(ctx, event_loop, game)
}
