pub mod ui;

use ggez;
use ggez::error::GameResult;
use ggez::{event, Context};

const FONT_PATH: &'static str = "/roguelike-font-16.png";
const FONT_MAP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÀÀÀÀÇÈÉÈÈÒÒÒÒÙÙÙÙabcdefghijklmnopqrstuvwxyzààààçè#%&@$.,!?:;'\"()[]*/\\+-<=>0123456789 ";
const FONT_WIDTH: u16 = 8;
const FONT_HEIGHT: u16 = 16;

const TILESET_PATH: &'static str = "/roguelike-tiles.png";
const TILESET_ALT_PATH: &'static str = "/unreleased-mcnoodlor.png";
const TILE_SIZE: u16 = 16;

#[derive(Copy, Clone, Debug)]
pub enum TileId {
    Chars = 1,
    Chars2,
    Chars3,
    Expr = 6,
    Fauna = 9,
    Trolls = 12,
    Unliving = 15,
    Creatures = 18,
    Building = 21,
    Building2,
    Building3,
    Devices = 26,
    Overworld = 29,
    Explore = 32,
    Food = 36,
    Food2,
    Outfit = 40,
    Outfit2,
    Magick = 44,
    Music = 47,
    Sym = 50,
    Sym2,
    Num = 54,
    FontUp = 55,
    FontUp2,
    FontLo,
    FontLo2,
    FontSy,
    Ico = 60,
}

const CLOCK_TILES: [(u8, u8); 24] = [
    (6, 30),
    (5, 31),
    (5, 31),
    (7, 30),
    (7, 31),
    (7, 31),
    (9, 30),
    (9, 31),
    (9, 31),
    (9, 28),
    (9, 29),
    (9, 29),
    (5, 30),
    (10, 29),
    (10, 29),
    (10, 28),
    (10, 31),
    (10, 31),
    (10, 30),
    (8, 31),
    (8, 31),
    (8, 30),
    (6, 31),
    (6, 31),
];

/// Small alias for formatting Debug types
fn p(t: impl std::fmt::Debug) -> String {
    format!("{:?}", t)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Btn {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
    L1,
    R1,
    X,
    Y,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameMode {
    Normal,
    Prompt,
}

#[derive(Debug)]
pub struct Game {
    pub mode: GameMode,
    cursor: usize,
    status: String,
}

const NORMAL_MODE_STATUS: &str = "<Esc> to use prompt";

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mode = GameMode::Normal;
        let cursor = 0;
        let status = String::new();

        Ok(Self {
            mode,
            cursor,
            status,
        })
    }

    pub fn set_status(&mut self, text: String) {
        self.status = text;
    }

    pub fn key_down(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        keymods: event::KeyMods,
    ) {
        match self.mode {
            GameMode::Normal => {
                match keycode {
                    event::KeyCode::Up => self._key_down(ctx, Btn::Up),
                    event::KeyCode::Down => self._key_down(ctx, Btn::Down),
                    event::KeyCode::Left => self._key_down(ctx, Btn::Left),
                    event::KeyCode::Right => self._key_down(ctx, Btn::Right),
                    event::KeyCode::Z => self._key_down(ctx, Btn::A),
                    event::KeyCode::X => self._key_down(ctx, Btn::B),
                    event::KeyCode::A => self._key_down(ctx, Btn::L1),
                    event::KeyCode::S => self._key_down(ctx, Btn::R1),
                    event::KeyCode::C => self._key_down(ctx, Btn::Start),
                    event::KeyCode::V => self._key_down(ctx, Btn::Select),
                    _ => return,
                };
            }
            GameMode::Prompt => match keycode {
                event::KeyCode::Home => {
                    self.cursor = 0;
                }
                event::KeyCode::End => {
                    self.cursor = self.status.chars().count();
                }
                event::KeyCode::Left => {
                    if self.cursor > 0 {
                        self.cursor = self.cursor - 1;
                    }
                }
                event::KeyCode::Right => {
                    if self.cursor < self.status.chars().count() {
                        self.cursor = self.cursor + 1;
                    }
                }
                _ => return,
            },
        }
        println!("cursor {:?}", self.cursor);
    }

    pub fn _key_down(&mut self, _ctx: &mut Context, btn: Btn) {
        println!("{:?}", btn);
    }

    pub fn get_prompt(&self) -> (usize, &str) {
        // (self.cursor_pos, self.prompt_content)
        (self.cursor, &self.status)
    }

    pub fn update_prompt(&mut self, c: char) {
        let byte_cursor = self
            .status
            .chars()
            .take(self.cursor)
            .collect::<String>()
            .len();

        if !c.is_ascii_control() {
            self.status.insert(byte_cursor, c);
            self.cursor += 1;
            return;
        } else if c == 0x7F as char && byte_cursor < self.status.len() {
            // Del
            self.status.remove(byte_cursor);
            if self.cursor > 0 && byte_cursor >= self.status.len() {
                // we deleted the last char, let's move cursor back as well
                self.cursor -= 1;
            }
        } else if c == 0x08 as char && byte_cursor > 0 {
            // Backspace
            self.cursor -= 1;
            let byte_cursor = self
                .status
                .chars()
                .take(self.cursor)
                .collect::<String>()
                .len();
            self.status.remove(byte_cursor);
        }
    }

    pub fn run_prompt(&mut self) {
        p(&self.status);
        self.status.clear();
        self.cursor = 0;
    }

    pub fn get_status(&self) -> &str {
        match self.mode {
            GameMode::Normal => NORMAL_MODE_STATUS,
            GameMode::Prompt => &self.status,
        }
    }
}
