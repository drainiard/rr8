pub mod font;
pub mod mouse;
pub mod palette;
pub mod prompt;
pub mod tile;
pub mod topbar;

use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, Rect};
use ggez::graphics::{Color, Drawable, FilterMode};
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;

use crate::*;
use font::Font;
use mouse::Mouse;
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

#[derive(Debug)]
pub struct Ui {
    pub dt: u32,
    font: Font,
    map: TileMap,
    map2: TileMap,
    mouse: Mouse,
    scale: f32,
    default_scale: f32,
    systems: Vec<Box<dyn System>>,
    win: Rect,
}

impl Ui {
    pub fn new(
        ctx: &mut Context,
        filter_mode: FilterMode,
        win: Rect,
        scale: f32,
    ) -> GameResult<Self> {
        let font = Font::new(ctx, filter_mode)?;
        let layout = TileLayout::new(TILESET_PATH);
        let layout2 = TileLayout::new(TILESET_ALT_PATH);
        let map = TileMap::new(ctx, layout, filter_mode)?;
        let map2 = TileMap::new(ctx, layout2, filter_mode)?;

        let mouse = Mouse::default();

        let systems: Vec<Box<dyn System>> = Vec::new();

        Self::validate_rect(win)?;

        Ok(Self {
            dt: 0,
            font,
            map,
            map2,
            mouse,
            scale,
            default_scale: scale,
            systems,
            win,
        })
    }

    fn validate_rect(rect: Rect) -> GameResult {
        let unit = TILE_SIZE as f32;

        if rect.x % unit > 0. || rect.y % unit > 0. || rect.w % unit > 0. || rect.h % unit > 0. {
            Err(ggez::GameError::WindowError(format!(
                "{:?} Not a multiple of {}",
                rect, TILE_SIZE
            )))
        } else {
            Ok(())
        }
    }

    pub fn add_system<S: 'static + System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn draw_all(&self, ctx: &mut Context, game: &Game) -> GameResult {
        // window
        let mesh = self.mesh(ctx, self.win.w as u8, self.win.w as u8, Pal::Off)?;
        self.draw(ctx, &mesh, 0., 0.)?;

        // body
        let mesh = self.mesh(ctx, 20, 18, Pal::DarkBlue.dark())?;
        self.draw(ctx, &mesh, 0., 1.)?;

        for system in self.systems.iter() {
            system.draw(ctx, game)?;
        }

        self.draw_textbox(ctx, 1., 2., 18., 1., Pal::DarkBlue)?;
        self.draw_textbox(ctx, 1., 4., 18., 14., Pal::DarkBlue)?;

        self.draw_text(ctx, "Rust Retro 8-bit", 6., 2., Pal::Blue)?;

        // draw mouse last so it's above everything else
        self.mouse.draw(ctx, game)?;

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

    pub fn draw_textbox(
        &self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: impl Into<Color> + Copy,
    ) -> GameResult {
        let fx = x * TILE_SIZE as f32 * self.scale;
        let fy = y * TILE_SIZE as f32 * self.scale;
        let fw = w * TILE_SIZE as f32;
        let fh = h * TILE_SIZE as f32;

        let bg = self.mesh_free(ctx, fw, fh, color)?;

        self.draw_free(ctx, &bg, fx, fy, self.scale)
    }

    pub fn mesh(
        &self,
        ctx: &mut Context,
        w: u8,
        h: u8,
        c: impl Into<Color>,
    ) -> GameResult<impl Drawable> {
        self.mesh_free(
            ctx,
            w as f32 * TILE_SIZE as f32,
            h as f32 * TILE_SIZE as f32,
            c,
        )
    }

    pub fn mesh_free(
        &self,
        ctx: &mut Context,
        w: f32,
        h: f32,
        c: impl Into<Color>,
    ) -> GameResult<impl Drawable> {
        let mode = graphics::DrawMode::fill();
        let bounds = Rect::new(0., 0., w, h);
        let color = c.into();

        graphics::Mesh::new_rectangle(ctx, mode, bounds, color)
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
                self.scale = self.default_scale;
            }
        }
    }
}
