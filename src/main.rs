use ggez::{event, GameResult};
use std::env;
use std::path;
use std::fs::{OpenOptions, self};

mod direction;
mod enemy;
mod entity;
mod player;
mod projectile;
mod random;
mod state;
mod tile;
mod utils;
mod world;

use crate::state::State;

// Constants that determine tile size and world size, where the world is a 2 dimensional array of
// tiles

//Offset to leave extra space on top of screen for health/energy indicators
pub const UNIVERSAL_OFFSET: i16 = 5;

// Define the world size which is (width, height)
pub const WORLD_SIZE: (i16, i16) = (50, 50);
// Define the board size; for now, doubled dimensions of WORLD_SIZE
pub const BOARD_SIZE: (i16, i16) = (350, 350);
// define the size of each tile which a square of pixels, size: (x, y) pixels.
pub const TILE_SIZE: (i16, i16) = (16, 16);
// define screen size in pixels. Will be grid size * tile size

pub const SAVE_PATH: &'static str = "./serialization/";

pub const SCREEN_SIZE: (f32, f32) = (
    (WORLD_SIZE.0 as f32) * TILE_SIZE.0 as f32,
    (WORLD_SIZE.1 as f32 + UNIVERSAL_OFFSET as f32) * TILE_SIZE.1 as f32,
);

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("songs");
        path
    } else {
        path::PathBuf::from("./songs")
    };
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("Rust Game", "Ishan Kar")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("RUST!!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .add_resource_path(resource_dir)
        .build()?;

    // let state = if (save) {
    //     let
    //     State::from()
    // }
    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = State::title_screen(&mut ctx)?;

    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
