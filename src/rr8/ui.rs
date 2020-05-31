pub mod palette;

use std::collections::HashMap;

use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::graphics::{Color, Drawable, FilterMode};
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;

use crate::*;
use palette::Pal;

pub struct Font {
    image: graphics::Image,
    font_map: HashMap<char, f32>,
    filter_mode: FilterMode,
}

impl Font {
    pub fn new(ctx: &mut Context, filter_mode: FilterMode) -> GameResult<Self> {
        let image = Image::new(ctx, FONT_PATH)?;
        let font_map = FONT_MAP
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as f32))
            .collect();

        Ok(Self {
            image,
            font_map,
            filter_mode,
        })
    }

    pub fn text_batch(
        &self,
        text: &str,
        color: impl Into<Color> + Copy,
    ) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());
        batch.set_filter(self.filter_mode);

        let mut column = 0;
        let mut line = 0;
        for c in text.chars() {
            if c == '\n' {
                column = 0;
                line += 1;
                continue;
            }
            batch.add(
                self.build_draw_param_from_char(&c)
                    .color(color.into())
                    .dest(Point2::new(
                        (column * FONT_WIDTH) as f32,
                        (line * FONT_HEIGHT) as f32,
                    )),
            );
            column += 1;
        }

        Ok(batch)
    }

    fn build_draw_param_from_char(&self, c: &char) -> DrawParam {
        let char_id = self.font_map.get(c).unwrap_or(&1023.);
        let w = self.image.width() as f32;
        let h = self.image.height() as f32;
        DrawParam::default().src(Rect::new(
            char_id * FONT_WIDTH as f32 / w,
            (*char_id as u32 * FONT_WIDTH as u32 / w as u32) as f32 / (h / FONT_HEIGHT as f32),
            FONT_WIDTH as f32 / w,
            FONT_HEIGHT as f32 / h,
        ))
    }
}

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

impl Into<u16> for TileId {
    fn into(self) -> u16 {
        self as u16
    }
}

pub struct TileLayout {
    path: &'static str,
    /// top-left, top-right, bottom-left, bottom-right, horiz, vert
    borders: Vec<[(u16, u16); 6]>,
}

pub struct TileMap {
    image: graphics::Image,
    layout: TileLayout,
    filter_mode: FilterMode,
}

impl TileMap {
    pub fn new(ctx: &mut Context, layout: TileLayout, filter_mode: FilterMode) -> GameResult<Self> {
        let image = Image::new(ctx, layout.path)?;
        Ok(Self {
            image,
            layout,
            filter_mode,
        })
    }

    pub fn batch(&self) -> SpriteBatch {
        let mut batch = SpriteBatch::new(self.image.clone());
        batch.set_filter(self.filter_mode);

        batch
    }

    pub fn tile(
        &self,
        t: impl Into<u16> + Copy + std::fmt::Debug,
        column: u16,
        c: impl Into<Color> + Copy,
        tile_width: u16,
        flip: bool,
    ) -> GameResult<SpriteBatch> {
        let mut batch = self.batch();

        let rect = self.rect_with_tile_width(t.into(), column, tile_width);

        let mut param = DrawParam::default().color(c.into()).src(rect);
        if flip {
            param = param
                .rotation(std::f32::consts::PI)
                .offset(Point2::new(0.9, 0.9));
        }
        batch.add(param);

        Ok(batch)
    }

    pub fn fill(
        &self,
        row: impl Into<u16>,
        column: u16,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
        tile_width: u16,
    ) -> GameResult<SpriteBatch> {
        let mut batch = self.batch();

        let rect = self.rect_with_tile_width(row.into(), column, tile_width);
        for x in 0..w {
            for y in 0..h {
                batch.add(
                    DrawParam::default()
                        .color(c.into())
                        .src(rect)
                        .dest(Point2::new(
                            x as f32 * tile_width as f32,
                            y as f32 * TILE_SIZE as f32,
                        )),
                );
            }
        }

        Ok(batch)
    }

    pub fn textbox(
        &self,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
        variant: usize,
    ) -> GameResult<SpriteBatch> {
        let mut batch = self.batch();

        let w_px = (w as f32 - 1.) * TILE_SIZE as f32;
        let h_px = (h as f32 - 1.) * TILE_SIZE as f32;

        let border = self.layout.borders[variant];

        let corner_positions = vec![(0., 0.), (w_px, 0.), (0., h_px), (w_px, h_px)];

        for (o, (x, y)) in (0..4).zip(corner_positions) {
            let (row, column) = border[o];
            let rect = self.rect(row, column);
            batch.add(
                DrawParam::default()
                    .color(c.into())
                    .src(rect)
                    .dest(Point2::new(x, y)),
            );
        }
        // horizontal edges
        for y in [0., h_px].iter() {
            for dw in 1..w - 1 {
                let (row, column) = border[4];
                let rect = self.rect(row, column);
                batch.add(
                    DrawParam::default()
                        .color(c.into())
                        .src(rect)
                        .dest(Point2::new((dw as u16 * TILE_SIZE) as f32, *y)),
                );
            }
        }

        // vertical edges
        for x in [0., w_px].iter() {
            for dh in 1..h - 1 {
                let (row, column) = border[5];
                let rect = self.rect(row, column);
                batch.add(
                    DrawParam::default()
                        .color(c.into())
                        .src(rect)
                        .dest(Point2::new(*x, (dh as u16 * TILE_SIZE) as f32)),
                );
            }
        }

        Ok(batch)
    }

    pub fn rect(&self, row: u16, column: u16) -> Rect {
        self.rect_with_tile_width(row, column, TILE_SIZE)
    }

    pub fn rect_with_tile_width(&self, row: u16, column: u16, tile_width: u16) -> Rect {
        let x = column * tile_width;
        let y = row * TILE_SIZE;

        self.rect_raw(x, y, tile_width, TILE_SIZE)
    }

    fn rect_raw<N: Into<f32>>(&self, x: N, y: N, w: N, h: N) -> Rect {
        let iw = self.image.width() as f32;
        let ih = self.image.height() as f32;

        Rect::new(x.into() / iw, y.into() / ih, w.into() / iw, h.into() / ih)
    }
}

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

pub struct Ui {
    pub dt: u32,
    font: Font,
    map: TileMap,
    map2: TileMap,
    scale: f32,
}

impl Ui {
    pub fn new(ctx: &mut Context, filter_mode: FilterMode, scale: f32) -> GameResult<Self> {
        let font = Font::new(ctx, filter_mode)?;
        let layout = TileLayout {
            path: TILESET_PATH,
            borders: vec![[
                (TileId::Building3 as u16, 3),
                (TileId::Building3 as u16, 4),
                (TileId::Building3 as u16, 5),
                (TileId::Building3 as u16, 6),
                (TileId::Building3 as u16, 0),
                (TileId::Building3 as u16, 1),
            ]],
        };
        let layout2 = TileLayout {
            path: TILESET_ALT_PATH,
            borders: vec![
                [(16, 16), (16, 19), (19, 16), (19, 19), (16, 17), (17, 16)],
                [(16, 20), (16, 23), (19, 20), (19, 23), (16, 21), (17, 20)],
                [(16, 24), (16, 27), (19, 24), (19, 27), (16, 25), (17, 24)],
                [(12, 16), (12, 19), (15, 16), (15, 19), (12, 17), (13, 16)],
                [(12, 20), (12, 23), (15, 20), (15, 23), (12, 21), (13, 20)],
                [(12, 24), (12, 27), (15, 24), (15, 27), (12, 25), (13, 24)],
            ],
        };
        let map = TileMap::new(ctx, layout, filter_mode)?;
        let map2 = TileMap::new(ctx, layout2, filter_mode)?;

        Ok(Self {
            dt: 0,
            font,
            map,
            map2,
            scale,
        })
    }

    pub fn draw_all(&self, ctx: &mut Context, game: &Game) -> GameResult {
        self.bg(ctx)?;
        self.draw_prompt(ctx, game)?;
        self.draw_topbar(ctx, game)?;

        Ok(())
    }

    pub fn draw_prompt(&self, ctx: &mut Context, game: &Game) -> GameResult {
        if let GameMode::Prompt = game.mode {
            let column = if self.dt & 0b100000 > 0 { 15 } else { 20 };
            let beam = self.tile8(TileId::Ico, column, Pal::Red, false)?;

            let (cursor_pos, prompt) = game.get_prompt();

            self.draw(ctx, &beam, 10. + cursor_pos as f32 / 2., 19.)?;
            self.draw_text(ctx, ">", 9., 19., Pal::LightGray)?;
        } else {
            self.draw_text(ctx, "#", 9., 19., Pal::LightGray)?;
        }

        self.draw_text(ctx, &game.get_status(), 10., 19., Pal::LightGray)?;

        Ok(())
    }

    pub fn draw_topbar(&self, ctx: &mut Context, game: &Game) -> GameResult {
        let (bg, fg, ico, bg_color, fg_color) = match game.mode {
            GameMode::Normal => (1, 5, 16, Pal::Black, Pal::Green),
            GameMode::Prompt => (1, 6, 26, Pal::Orange, Pal::Black),
        };
        self.draw(ctx, &self.tile_alt(bg, ico, fg_color, false)?, 9., 0.)?;
        self.draw(ctx, &self.tile_alt(fg, ico, bg_color, false)?, 9., 0.)?;
        self.draw_text(ctx, &p(&game.mode).to_uppercase(), 10.5, 0., Pal::LightGray)?;

        Ok(())
    }

    pub fn bg(&self, ctx: &mut Context) -> GameResult {
        let mesh = self.mesh(ctx, 19, 20, Pal::Black)?;
        self.draw(ctx, &mesh, 8.5, 0.)?;

        let mesh = self.mesh(ctx, 18, 18, Pal::DarkBlue.dark())?;
        self.draw(ctx, &mesh, 9., 1.)?;

        let fill = self.fill_alt(0, 5, 1, 18, Pal::DarkBlue.dark())?;
        self.draw(ctx, &fill, 8.5, 1.0)?;
        self.draw(ctx, &fill, 26.5, 1.0)?;

        Ok(())
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        drawable: &impl graphics::Drawable,
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            drawable,
            graphics::DrawParam::default()
                .dest(Point2::new(
                    x * TILE_SIZE as f32 * self.scale,
                    y * TILE_SIZE as f32 * self.scale,
                ))
                .scale(Vector2::new(self.scale, self.scale)),
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
