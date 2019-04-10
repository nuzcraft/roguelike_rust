/// this file will hold the constants used by the program
extern crate tcod;
use tcod::colors::{Color};
use tcod::map::{FovAlgorithm};


// actual size of the screen
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const LIMIT_FPS: i32 = 20; // limit frames per second

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

// sizes and coordinates relvant for the GUI
pub const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub const MSG_X: i32 = BAR_WIDTH + 2;
pub const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
pub const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

pub const INVENTORY_WIDTH: i32 = 50;

// parameters for dungeon generator
pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;

pub const COLOR_DARK_WALL: Color = Color{r: 0, g: 0, b: 100};
pub const COLOR_LIGHT_WALL: Color = Color{r: 130, g: 110, b: 50};
pub const COLOR_DARK_GROUND: Color = Color{r: 50, g: 50, b: 150};
pub const COLOR_LIGHT_GROUND: Color = Color{r: 200, g: 180, b: 50};

//fov
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGHT_WALLS: bool = true; // light walls or not
pub const TORCH_RADIUS: i32 = 10;

// items
pub const HEAL_AMOUNT: i32 = 40;
pub const LIGHTNING_DAMAGE: i32 = 40;
pub const LIGHTNING_RANGE: i32 = 5;
pub const CONFUSE_RANGE: i32 = 8;
pub const CONFUSE_NUM_TURNS: i32 = 10;
pub const FIREBALL_RADIUS: i32 = 3;
pub const FIREBALL_DAMAGE: i32 = 25;

// leveling up
pub const LEVEL_UP_BASE: i32 = 200;
pub const LEVEL_UP_FACTOR: i32 = 150;
pub const LEVEL_SCREEN_WIDTH: i32 = 40;

// character screen
pub const CHARACTER_SCREEN_WIDTH: i32 = 30;

// player will always be the first object
pub const PLAYER: usize = 0;