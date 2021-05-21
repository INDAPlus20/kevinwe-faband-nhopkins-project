/**
 * KThCCG Frontend.
 * Author: Fabian Andréasson <fab.andreasson@gmail.com>
 * Last updated: 2021-05-20
 */
// std
use std::{io, path};

// Local imports from lib ´backend´ folder
#[path = "lib/board.rs"]
mod board;
#[path = "lib/card.rs"]
mod card;
#[path = "lib/player.rs"]
mod player;
#[path = "lib/traits.rs"]
mod traits;
// Local import structs
use board::Board;
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

/// GUI Structure and logic implementation
struct Game {
    /// Image of the cards and relevant knowledge for
    /// how it should handle inputs
    sprites: Vec<(Card, graphics::Image)>,
    deck: Vec<Card>,
    player_one: player::Player,
    player_two: player::Player,
    /// It is an option since logically there does not need to be an active player at all times.
    /// For example; starting out a game, it might be undecided. This also makes it easier to
    /// implement it as a pointer :P
    current_player: PlayerType,
    /// Implementation of backend board
    // TODO: Implement it :P
    board: Board,
    selected_card: Option<(usize, usize)>,
}

/// Implementation of basic functions of the frontend program
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let sprites = Game::load_sprites();
        Ok(Game {
            sprites: sprites
                .iter()
                .map(|sprite| {
                    (
                        sprite.0,
                        graphics::Image::new(ctx, sprite.1.clone()).unwrap(),
                    )
                })
                .collect::<Vec<(Card, graphics::Image)>>(),
            deck: Vec::<Card>::new(),
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
            selected_card: None,
        })
    }
    fn update(&mut self) {
        for i in 0..self.player_one.hand.len() {
            self.board.field[5 + i][6] = Some(self.player_one.hand[i]);
        }
        if self.player_one.hand.len() == 0 {
            println!("GG");
            std::process::exit(0);
        }
    }
    fn load_sprites() -> Vec<(Card, String)> {
        // TODO: Implement graphics and proper loading function7
        let mut sprites: Vec<(Card, String)> = Vec::new();
        // Temporary values for cards
        // make lots of them so you can use the deck properly
        for i in 0..30 {
            sprites.push((
                Card::new(
                    Deck,
                    10,
                    10,
                    (Person, EECS),
                    (Damage, 10),
                    //"Tänk om SM slutade it tid...".to_string(),
                ),
                "/ccg-crab-1.png".to_string(),
            ));
        }
        return sprites;
    }
    /// Fills deck
    pub fn fill_deck(&mut self) {
        self.deck = vec![self.sprites[0].0; 30];
    }
    /// For presentation
    pub fn fill_board(&mut self) {
        self.board.field =
            [[Some(self.sprites[0].0); CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize];
    }
    fn draw_card(&mut self) {
        if self.deck.len() > 0 {
            self.player_one.draw(self.deck.pop().unwrap());
        }
    }
    /*
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
        // Calling for the backend to update the internal logic
        self.update();
        Ok(())
    }
    /// For drawing interface and graphical representation of the current state of the Game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Ordinary background color. TODO: Change or remove
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());
        //self.fill_board();
        // Draw playing field
        for x in 0..CELL_AMOUNT.1 as usize {
            for y in 0..CELL_AMOUNT.0 as usize {
                /*let board = graphics::draw(
                    ctx,
                    &graphics::Image::new(ctx, "/ccg-tile-1.png".to_string()),
                    DrawParam::default(),
                );*/
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
        // Drawing played cards
        for x in 0..CELL_AMOUNT.1 as usize {
            for y in 0..CELL_AMOUNT.0 as usize {
                if let Some(card) = self.board.field[x][y] {
                    let card_for_sprite = card;
                    let sprite: graphics::Image = self
                        .sprites
                        .iter()
                        .find(|&x| x.0.ctype == card_for_sprite.ctype)
                        .unwrap()
                        .1
                        .clone();
                    // This works somehow, lets not poke at it
                    //.map(|x| if x.0.ctype == card_for_sprite.ctype )
                    graphics::draw(
                        ctx,
                        &sprite,
                        DrawParam::default()
                            .scale(ggez::mint::Point2 {
                                x: CELL_SIZE.1 as f32 / sprite.width() as f32,
                                y: CELL_SIZE.0 as f32 / sprite.height() as f32,
                            })
                            .dest(ggez::mint::Point2 {
                                x: (CELL_SIZE.1 as f32) * x as f32,
                                y: (CELL_SIZE.0 as f32) * y as f32,
                            }),
                    )
                    .expect("Draw pls");
                }
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
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let inp_index = ((x / CELL_SIZE.1) as isize, (y / CELL_SIZE.0) as isize);
        // For testing
        println!("{:?}", inp_index);
        println!("{:?}", self.selected_card);
        // Selection of cards
        if ((5..=14).contains(&inp_index.0) && inp_index.1 == 6
            || (5..=14).contains(&inp_index.0) && inp_index.1 == 1)
            && (inp_index.0 < (self.player_one.hand.len() as isize + 5))
        {
            self.selected_card = Some((inp_index.0 as usize, inp_index.1 as usize));
        } else if (5..=14).contains(&inp_index.0) && inp_index.1 == 5
            || (5..=14).contains(&inp_index.0) && inp_index.1 == 2
        {
            if self.board.field[inp_index.0 as usize][inp_index.1 as usize].is_none()
                && self.selected_card.is_some()
            {
                self.board.field[inp_index.0 as usize][inp_index.1 as usize] =
                    self.board.field[self.selected_card.unwrap().0][self.selected_card.unwrap().1];

                self.board.field[4 + self.player_one.hand.len()][self.selected_card.unwrap().1] =
                    None;
                self.player_one.hand.remove(inp_index.1 as usize - 5);
                self.selected_card = None
            } else {
                self.selected_card = None;
            }
        } else {
            self.selected_card = None;
        }
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
    let mut state = Game::new(&mut context)?;
    // TESTING
    state.fill_deck();
    for i in 0..=4 {
        state.draw_card();
    }
    // Run application window
    run(context, event_loop, state)
}
