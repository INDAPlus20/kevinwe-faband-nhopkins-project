/**
 * KThCCG Frontend.
 * Author: Fabian Andréasson <fab.andreasson@gmail.com>
 * Last updated: 2021-05-17
 */
// std
use std::io;
use std::path;

// TODO: Imports from backend
// This can be fixed in another branch

// ggez
use ggez::event;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};

/// Size of the application window. TODO: Fix when art is made
const SCREEN_SIZE: (f32, f32) = (420.0, 420.0);

/// GUI Structure and logic implementation
struct AppState {
    /// Image of the cards and relevant knowledge for
    /// how it should handle inputs
    sprites: Vec<graphics::Image>,
    /// Implementation of backend board
    // TODO: Implement it :P
    board: i64,
}

/// Implementation of basic functions of the frontend program
impl AppState {
    fn new() -> GameResult<AppState> {
        // TODO: Implement once backend exists
    }
    fn load_images() -> Vec<graphics::Image> {
        // TODO: Implement graphics and proper loading function
        vec![]
    }
}

/// Implementeation of the eventloop in the frontend
impl event::EventHandler for AppState {
    /// Updating function
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    /// For drawing interface and graphical representation
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        // TODO: Implement graphics and locations of things
        Ok(())
    }
    // Implement functions for events of key presses, etc
}

fn main() -> GameResult {
    // Import path for graphical resources
    // TODO: Make graphical resources
    // They do not need to be complex
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

    // TODO: Input handler
}
