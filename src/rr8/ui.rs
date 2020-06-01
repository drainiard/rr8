pub mod font;
pub mod palette;
pub mod tile;

use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, Rect};
use ggez::graphics::{Color, Drawable, FilterMode};
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;

use crate::*;
use font::Font;
use palette::Pal;
use tile::{TileLayout, TileMap};

#[derive(Debug, Eq, PartialEq)]
pub enum Scale {
    Default,
    Up,
    Down,
    Min,
    Max,
}

impl Scale {
    pub const DELTA: f32 = 0.5;
    pub const DEFAULT: f32 = Self::DELTA * 2.;
    pub const MIN: f32 = Self::DELTA;
    pub const MAX: f32 = Self::DELTA * 7.;
}

pub trait System {
    fn update(&mut self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult;
    fn draw(&self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult;
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct TopBar;

impl System for TopBar {
    fn update(&mut self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        let default_color = Pal::Gray.dark();
        let (bg, fg, ico, bg_color, fg_color) = match game.mode {
            GameMode::Normal => (1, 5, 16, Pal::Black, default_color),
            GameMode::Prompt => (1, 5, 16, Pal::Black, Pal::Green.into()),
        };
        ui.draw(ctx, &ui.tile_alt(bg, ico, fg_color, false)?, 1., 0.)?;
        ui.draw(ctx, &ui.tile_alt(fg, ico, bg_color, false)?, 1., 0.)?;
        ui.draw_text(ctx, &p(&game.mode).to_uppercase(), 2.5, 0., default_color)?;

        ui.draw_text(ctx, &format!("x{}", ui.scale), 18., 0., Pal::Gray.darker())?;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Mouse {
    coords: (f32, f32),
}

impl Mouse {
    pub fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords;
    }
}

impl System for Mouse {
    fn update(&mut self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        if let GameMode::Normal = game.mode {
            let (x, y) = self.coords;

            let mouse = ui.tile_alt(9, 16, Pal::Orange, false)?;
            let mouse_inner = ui.tile_alt(9, 19, Pal::Red, false)?;
            let mouse_shadow = ui.tile_alt(9, 19, Pal::Black, false)?;
            ui.draw_free(ctx, &mouse_shadow, x + 1., y + 2., ui.scale)?;
            ui.draw_free(ctx, &mouse_inner, x, y, ui.scale)?;
            ui.draw_free(ctx, &mouse, x, y, ui.scale)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Prompt;

impl System for Prompt {
    fn update(&mut self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        todo!()
    }
    fn draw(&self, ctx: &mut Context, ui: &Ui, game: &Game) -> GameResult {
        let (prompt_color, prompt_text) = match &game.mode {
            GameMode::Normal => (Pal::Gray.dark(), game.get_status()),
            GameMode::Prompt => {
                let column = if ui.dt & 0b100000 > 0 { 15 } else { 20 };
                let beam = ui.tile8(TileId::Ico, column, Pal::Red, false)?;

                let (cursor_pos, prompt) = game.get_prompt();

                // this also works nice because drawing the beam before the
                // prompt makes the char underneath it visible
                ui.draw(ctx, &beam, 2. + cursor_pos as f32 / 2., 19.)?;

                (Pal::LightGray.into(), prompt)
            }
        };
        ui.draw_text(ctx, "#", 1., 19., Pal::Gray.dark())?;
        ui.draw_text(ctx, prompt_text, 2., 19., prompt_color)?;

        Ok(())
    }
}

pub struct Ui {
    pub dt: u32,
    font: Font,
    map: TileMap,
    map2: TileMap,
    mouse: Mouse,
    scale: f32,
    systems: Vec<Box<dyn System>>,
}

impl Ui {
    pub fn new(ctx: &mut Context, filter_mode: FilterMode, scale: f32) -> GameResult<Self> {
        let font = Font::new(ctx, filter_mode)?;
        let layout = TileLayout::new(
            TILESET_PATH,
            vec![[
                (TileId::Building3 as u16, 3),
                (TileId::Building3 as u16, 4),
                (TileId::Building3 as u16, 5),
                (TileId::Building3 as u16, 6),
                (TileId::Building3 as u16, 0),
                (TileId::Building3 as u16, 1),
            ]],
        );
        let layout2 = TileLayout::new(
            TILESET_ALT_PATH,
            vec![
                [(16, 16), (16, 19), (19, 16), (19, 19), (16, 17), (17, 16)],
                [(16, 20), (16, 23), (19, 20), (19, 23), (16, 21), (17, 20)],
                [(16, 24), (16, 27), (19, 24), (19, 27), (16, 25), (17, 24)],
                [(12, 16), (12, 19), (15, 16), (15, 19), (12, 17), (13, 16)],
                [(12, 20), (12, 23), (15, 20), (15, 23), (12, 21), (13, 20)],
                [(12, 24), (12, 27), (15, 24), (15, 27), (12, 25), (13, 24)],
            ],
        );
        let map = TileMap::new(ctx, layout, filter_mode)?;
        let map2 = TileMap::new(ctx, layout2, filter_mode)?;

        let mouse = Mouse::default();

        let systems: Vec<Box<dyn System>> = Vec::new();

        Ok(Self {
            dt: 0,
            font,
            map,
            map2,
            mouse,
            scale,
            systems,
        })
    }

    pub fn add_system<S: 'static + System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn draw_all(&self, ctx: &mut Context, game: &Game) -> GameResult {
        self.bg(ctx)?;

        for system in self.systems.iter() {
            system.draw(ctx, self, game)?;
        }

        // draw mouse last so it's above everything else
        self.mouse.draw(ctx, self, game)?;

        Ok(())
    }

    pub fn bg(&self, ctx: &mut Context) -> GameResult {
        let mesh = self.mesh(ctx, 20, 20, Pal::Off)?;
        self.draw(ctx, &mesh, 0., 0.)?;

        let mesh = self.mesh(ctx, 19, 18, Pal::Gray)?;
        self.draw(ctx, &mesh, 0.5, 1.)?;

        let fill = self.fill_alt(0, 5, 1, 18, Pal::Gray.dark())?;
        self.draw(ctx, &fill, -0.5, 1.)?;
        self.draw(ctx, &fill, 19.5, 1.)?;

        Ok(())
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        drawable: &impl graphics::Drawable,
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        self.draw_free(
            ctx,
            drawable,
            x * TILE_SIZE as f32 * self.scale,
            y * TILE_SIZE as f32 * self.scale,
            self.scale,
        )
    }

    pub fn draw_free(
        &self,
        ctx: &mut ggez::Context,
        drawable: &impl graphics::Drawable,
        x: f32,
        y: f32,
        scale: f32,
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            drawable,
            graphics::DrawParam::default()
                .dest(Point2::new(x, y))
                .scale(Vector2::new(scale, scale)),
        )
    }

    pub fn draw_text(
        &self,
        ctx: &mut ggez::Context,
        text: &str,
        x: f32,
        y: f32,
        color: impl Into<graphics::Color>,
    ) -> ggez::GameResult {
        self.draw(ctx, &self.text_batch(text, color.into())?, x, y)
    }

    pub fn mesh(
        &self,
        ctx: &mut Context,
        w: u8,
        h: u8,
        c: impl Into<Color>,
    ) -> GameResult<impl Drawable> {
        let mode = graphics::DrawMode::fill();
        let bounds = Rect::new_i32(
            0,
            0,
            w as i32 * TILE_SIZE as i32,
            h as i32 * TILE_SIZE as i32,
        );
        let color = c.into();

        graphics::Mesh::new_rectangle(ctx, mode, bounds, color)
    }

    pub fn textbox(
        &self,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
        variant: usize,
    ) -> GameResult<SpriteBatch> {
        self.map2.textbox(w, h, c, variant)
    }

    pub fn tile(
        &self,
        t: TileId,
        column: u16,
        c: impl Into<Color> + Copy,
        flip: bool,
    ) -> GameResult<SpriteBatch> {
        self.map.tile(t, column, c, TILE_SIZE, flip)
    }

    pub fn tile8(
        &self,
        t: TileId,
        column: u16,
        c: impl Into<Color> + Copy,
        flip: bool,
    ) -> GameResult<SpriteBatch> {
        self.map.tile(t, column, c, TILE_SIZE / 2, flip)
    }

    pub fn tile_alt(
        &self,
        row: u16,
        column: u16,
        c: impl Into<Color> + Copy,
        flip: bool,
    ) -> GameResult<SpriteBatch> {
        self.map2.tile(row, column, c, TILE_SIZE, flip)
    }

    pub fn fill8(
        &self,
        t: TileId,
        column: u16,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
    ) -> GameResult<SpriteBatch> {
        self.map.fill(t, column, w, h, c, TILE_SIZE / 2)
    }

    pub fn fill_alt(
        &self,
        row: u16,
        column: u16,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
    ) -> GameResult<SpriteBatch> {
        self.map2.fill(row, column, w, h, c, TILE_SIZE)
    }

    pub fn text_batch(
        &self,
        text: &str,
        color: impl Into<Color> + Copy,
    ) -> GameResult<SpriteBatch> {
        self.font.text_batch(text, color)
    }

    pub fn set_mouse_coords(&mut self, coords: (f32, f32)) {
        self.mouse.set_coords(coords);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Scale) {
        match scale {
            Scale::Up => {
                if self.scale < Scale::MAX {
                    self.scale += Scale::DELTA
                }
            }
            Scale::Down => {
                if self.scale > Scale::MIN {
                    self.scale -= Scale::DELTA
                }
            }
            Scale::Min => {
                self.scale = Scale::MIN;
            }
            Scale::Max => {
                self.scale = Scale::MAX;
            }
            Scale::Default => {
                self.scale = Scale::DEFAULT;
            }
        }
    }
}
