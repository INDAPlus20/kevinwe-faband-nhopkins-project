/**
 * KThCCG Frontend.
 * Author: Fabian Andréasson <fab.andreasson@gmail.com>
 * Last updated: 2021-05-20
 */
// std
use std::io;
use std::path;

// Local imports from lib ´backend´ folder
#[path = "lib/card.rs"]
mod card;
#[path = "lib/Player.rs"]
mod player;
#[path = "lib/traits.rs"]
mod traits;

// Local import structs
use card::Card;

// ggez
use ggez::event;
use ggez::event::{run, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};

/// Here we define the size of the playingfield and the screen. The screen size depends
/// on 2 variables; size of cells and size of cards. This is to make it scale properly.
///
/// ## Cell size
/// Size of a cell of which the screen depends on. It is a tuple conataining the float type.
const CELL_SIZE: (f32, f32) = (160.0, 90.0);
/// Here we define the size of the playingfield and the screen. The screen size depends
/// on 2 variables; size of cells and size of cards. This is to make it scale properly.
///
/// ## Card size
/// Size of a card of which the screen depends on. It is a tuple conataining the float type.
const CARD_SIZE: (f32, f32) = (160.0, 90.0);
/// Here we define the size of the playingfield and the screen. The screen size depends
/// on 2 variables; size of cells and size of cards. This is to make it scale properly.
///
/// ## Amount of Cells
/// How many cells that the screen will contain.
const AMOUNT_OF_CELLS: i32 = 12;
/// ## Screen size
/// Decides the size of the screen that will be rendered by ggez. Depends on Cell size and
/// Amount of Cells.
const SCREEN_SIZE: (f32, f32) = (
    CELL_SIZE.0 * AMOUNT_OF_CELLS as f32,
    CELL_SIZE.1 * AMOUNT_OF_CELLS as f32,
);

/// ## Board
/// Layout of the board that the entire game plays by. This holds the logic for where cards are
/// stored and it is necessary for generation of graphics.
struct Board {
    field: [[Option<Card>; AMOUNT_OF_CELLS as usize]; AMOUNT_OF_CELLS as usize],
    cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [[None; AMOUNT_OF_CELLS as usize]; AMOUNT_OF_CELLS as usize],
        }
    }
}

/// GUI Structure and logic implementation
struct Game {
    /// Image of the cards and relevant knowledge for
    /// how it should handle inputs
    sprites: Vec<(Card, graphics::Image)>,
    current_player: player::Player,
    focused_player: player::Player,
    /// Implementation of backend board
    // TODO: Implement it :P
    board: Board,
}

/// Implementation of basic functions of the frontend program
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let state = Game {
            board: Board::new(),
        };
        let player1 = Game::new_player();
        Game {
            sprites: Game::load_images(),
            current_player: player1,
            focused_player: player1,
            board: Board::new(),
        }
        Ok(state)
    }
    fn new_player() -> player::Player {
        // TODO: Generate new player
    }
    fn load_sprites() -> Vec<(Card, graphics::Image)> {
        // TODO: Implement graphics and proper loading function7
        let mut sprites = Vec::new();
        sprites.push((Card::new(), "/ccg-test-1.png".to_string()));
    }
    /// play plays a card in the player's hand, optionally supply targets
    ///
    /// card : the
    /// card_targets is either a Card, Player, or ... which inherits the Target trait
    pub fn play(card: Option<Card>, card_targets: Option<&Target>) -> Result<T, E> {
        // pass
    }
}

/// Implementeation of the eventloop in the frontend
impl event::EventHandler for Game {
    /// Updating function for game logic, which currently is not handeled by the frontend
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    /// For drawing interface and graphical representation of the current state of the Game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Ordinary background color. TODO: Change or remove
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // Draw playing field
        for i in 0..(AMOUNT_OF_CELLS * AMOUNT_OF_CELLS) {
            let board = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    i % AMOUNT_OF_CELLS * CELL_SIZE.0 as i32,
                    i / AMOUNT_OF_CELLS * CELL_SIZE.1 as i32,
                    CELL_SIZE.0 as i32,
                    CELL_SIZE.1 as i32,
                ),
                Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0),
            )?;
            graphics::draw(ctx, &board, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }
        // TODO: Implement graphics and locations of things
        // TODO: Draw more
        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        //g
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        //f
    }
    // Implement functions for events of key presses, etc
    // TODO: Input handler
}
/// Here is all necessary functions from GGEZ to compile and run the frontend of the program.
fn main() -> GameResult {
    // Import path for graphical resources
    // TODO: Make graphical resources
    // They do not need to be complex
    let resource_directory = path::PathBuf::from("./resources");

    // Creates and configures ggez context
    // TODO: Configure correctly
    let context_builder = ggez::ContextBuilder::new("temp", "kevinwe-faband-nhopkins")
        .add_resource_path(resource_directory)
        .window_setup(ggez::conf::WindowSetup::default().title("KThCCG"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .resizable(false),
        );
    // As a sidenote, the documentation for this is outdated- took a bit to figure it out
    // Builds context
    let (mut context, event_loop) = context_builder.build()?;
    // Initiates game
    let state = Game::new(&mut context)?;
    // Run application window
    run(context, event_loop, state);
    Ok(())
}
