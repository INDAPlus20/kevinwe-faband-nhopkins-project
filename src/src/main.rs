/**
 * KThCCG Frontend.
 * Author: Fabian Andréasson <fab.andreasson@gmail.com>
 * Last updated: 2021-04-24
 */
// std
use std::io;
use std::path;

// TODO: Imports from backend

// ggez
use ggez::event;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::GameResult;

/// Size of the application window. TODO: Fix when art is made
const SCREEN_SIZE: (f32, f32) = (420.0, 420.0);

fn main() -> GameResult {
    // Import path for graphical resources
    // TODO: Make graphical resources
    let resource_directory = path::PathBuf::from("./resources");

    // Creates and configures ggez context
    // TODO: Configure correctly
    let context_builder = ggez::ContextBuilder::new("temp", "kevinwe-faband-nhopkins")
        .add_resource_path(resource_directory)
        .window_setup(ggez::conf::WindowSetup::default())
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .resizable(false),
        );
    // Builds context
    let (context, event_loop) = &mut context_builder.build()?;
    Ok(())
}
