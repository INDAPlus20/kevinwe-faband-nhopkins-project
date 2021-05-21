/**
 * KThCCG Frontend.
 * Author: Fabian Andréasson <fab.andreasson@gmail.com>
 * Last updated: 2021-05-20
 */
// std
use std::{io, path};

// Local imports from lib ´backend´ folder
#[path = "lib/card.rs"]
mod card;
#[path = "lib/player.rs"]
mod player;
#[path = "lib/traits.rs"]
mod traits;
// Local import structs
use card::CType::*;
use card::CardPosition::*;
use card::{CType, Card, CardPosition};
use player::Player;
use traits::Effect::*;
use traits::PlayerType;

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
const CELL_SIZE: (f32, f32) = (135.0, 96.0);
/// Here we define the size of the playingfield and the screen. The screen size depends
/// on 2 variables; size of cells and size of cards. This is to make it scale properly.
///
/// ## Card size
/// Size of a card of which the screen depends on. It is a tuple conataining the float type.
const CARD_SIZE: (f32, f32) = (135.0, 96.0);
/// Here we define the size of the playingfield and the screen. The screen size depends
/// on 2 variables; size of cells and size of cards. This is to make it scale properly.
///
/// ## Amount of Cells
/// How many cells that the screen will contain.
const CELL_AMOUNT: (i32, i32) = (8, 20);
/// ## Screen size
/// Decides the size of the screen that will be rendered by ggez. Depends on Cell size and
/// Amount of Cells.
const SCREEN_SIZE: (f32, f32) = (
    CELL_SIZE.0 * CELL_AMOUNT.0 as f32,
    CELL_SIZE.1 * CELL_AMOUNT.1 as f32,
);

/// ## Board
/// Layout of the board that the entire game plays by. This holds the logic for where cards are
/// stored and it is necessary for generation of graphics.
struct Board {
    field: [[Option<Card>; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
    //cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [[None; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
        }
    }
}

/// GUI Structure and logic implementation
struct Game {
    /// Image of the cards and relevant knowledge for
    /// how it should handle inputs
    deck: Vec<(Card, graphics::Image)>,
    player_one: player::Player,
    player_two: player::Player,
    /// It is an option since logically there does not need to be an active player at all times.
    /// For example; starting out a game, it might be undecided. This also makes it easier to
    /// implement it as a pointer :P
    current_player: PlayerType,
    /// Implementation of backend board
    // TODO: Implement it :P
    board: Board,
}

/// Implementation of basic functions of the frontend program
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let sprites = Game::load_sprites();
        Ok(Game {
            deck: sprites
                .iter()
                .map(|sprite| {
                    (
                        sprite.0,
                        graphics::Image::new(ctx, sprite.1.clone()).unwrap(),
                    )
                })
                .collect::<Vec<(Card, graphics::Image)>>(),
            player_one: Player {
                health: 100,
                hand: Vec::<Card>::new(),
                special_ability: (ModStrength, 0),
            },
            player_two: Player {
                health: 100,
                hand: Vec::<Card>::new(),
                special_ability: (ModStrength, 0),
            },
            current_player: PlayerType::One,
            board: Board::new(),
        })
    }
    fn load_sprites() -> Vec<(Card, String)> {
        // TODO: Implement graphics and proper loading function7
        let mut sprites: Vec<(Card, String)> = Vec::new();
        // Temporary values for cards
        // make lots of them so you can use the deck properly
        for i in 0..30{
            sprites.push((
                Card::new(
                    Deck,
                    10,
                    10,
                    (Person, EECS),
                    (Damage, 10),
                    //"Tänk om SM slutade it tid...".to_string(),
                ),
                "/ccg-test-1.png".to_string(),
            ));
        }
        return sprites;
    } /*
      // TODO: Implement Target trait for respective functions
      /// play plays a card in the player's hand, optionally supply targets
      ///
      /// card : the
      /// card_targets is either a Card, Player, or ... which inherits the Target trait
      pub fn play(card: Option<Card>, card_targets: Option<&Target>) -> Result<T, E> {
          // pass
      }*/
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
        for x in 0..CELL_AMOUNT.1 as usize {
            for y in 0..CELL_AMOUNT.0 as usize {
                let board = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        x as i32 * CELL_SIZE.1 as i32,
                        y as i32 * CELL_SIZE.0 as i32,
                        CELL_SIZE.1 as i32,
                        CELL_SIZE.0 as i32,
                    ),
                    // TODO: Better graphics generation
                    match y {
                        1 | 6 => match x {
                            5..=14 => match x % 2 {
                                1 => Color::new(87.0 / 255.0, 19.0 / 255.0, 39.0 / 255.0, 1.0),
                                _ => Color::new(87.0 / 255.0, 19.0 / 255.0, 39.0 / 255.0, 0.5),
                            },
                            _ => Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0),
                        },
                        2 | 5 => match x {
                            5..=14 => match x % 2 {
                                1 => Color::new(255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 1.0),
                                _ => Color::new(255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 0.5),
                            },
                            _ => Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0),
                        },
                        _ => Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0),
                    },
                )?;
                graphics::draw(ctx, &board, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }
        // TODO: Implement graphics and locations of things
        // TODO: Draw more
        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        //g
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let out_index = ((x / CELL_SIZE.1) as isize, (y / CELL_SIZE.0) as isize);
        println!("{:?}", out_index);
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
    let context_builder = ggez::ContextBuilder::new("KThCCG", "kevinwe-faband-nhopkins")
        .add_resource_path(resource_directory)
        .window_setup(ggez::conf::WindowSetup::default().title("KThCCG"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.1, SCREEN_SIZE.0)
                .resizable(false),
        );
    // As a sidenote, the documentation for this is outdated- took a bit to figure it out
    // Builds context
    let (mut context, event_loop) = context_builder.build()?;
    // Initiates game
    let state = Game::new(&mut context)?;
    // Run application window
    run(context, event_loop, state)
}
