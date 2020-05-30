use std::collections::HashMap;

use ggez;
use ggez::error::GameResult;
use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{event, Context};
use graphics::{Color, Drawable, FilterMode};
use palette::Pal;

pub mod palette;

const TILESET_PATH: &'static str = "/roguelike-tiles.png";
const TILESET_ALT_PATH: &'static str = "/unreleased-mcnoodlor.png";
const TILE_SIZE: u16 = 16;

const FONT_PATH: &'static str = "/roguelike-font-16.png";
const FONT_MAP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÀÀÀÀÇÈÉÈÈÒÒÒÒÙÙÙÙabcdefghijklmnopqrstuvwxyzààààçè#%&@$.,!?:;'\"()[]*/\\+-<=>0123456789 ";
const FONT_WIDTH: u16 = 8;
const FONT_HEIGHT: u16 = 16;

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

#[derive(Debug, Hash)]
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
        t: impl Into<u16>,
        column: u16,
        c: impl Into<Color>,
        tile_width: u16,
    ) -> GameResult<SpriteBatch> {
        let mut batch = self.batch();

        let rect = self.rect_with_tile_width(t.into(), column, tile_width);
        batch.add(DrawParam::default().color(c.into()).src(rect));

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

pub struct Ui {
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
                // top-left, top-right, bottom-left, bottom-right, horiz, vert
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
            font,
            map,
            map2,
            scale,
        })
    }

    pub fn bg(&self, ctx: &mut Context) -> GameResult {
        let mesh = self.mesh(ctx, 36, 18, Pal::DarkBlue.darker())?;
        self.draw(ctx, &mesh, 0., 1.)?;

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

    pub fn tile(&self, t: TileId, column: u16, c: impl Into<Color>) -> GameResult<SpriteBatch> {
        self.map.tile(t, column, c, TILE_SIZE)
    }

    pub fn tile8(&self, t: TileId, column: u16, c: impl Into<Color>) -> GameResult<SpriteBatch> {
        self.map.tile(t, column, c, TILE_SIZE / 2)
    }

    pub fn tile_alt(&self, row: u16, column: u16, c: impl Into<Color>) -> GameResult<SpriteBatch> {
        self.map2.tile(row, column, c, TILE_SIZE)
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

#[derive(Debug, Default)]
pub struct Game {
    pub status: String,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Self::default())
    }

    pub fn set_status(&mut self, text: String) {
        self.status = text;
    }

    pub fn key_down(&mut self, ctx: &mut Context, keycode: event::KeyCode) {
        let result = match keycode {
            event::KeyCode::Z => self._key_down(ctx, Btn::A),
            event::KeyCode::X => self._key_down(ctx, Btn::B),
            event::KeyCode::A => self._key_down(ctx, Btn::L1),
            event::KeyCode::S => self._key_down(ctx, Btn::R1),
            event::KeyCode::C => self._key_down(ctx, Btn::Start),
            event::KeyCode::V => self._key_down(ctx, Btn::Select),
            _ => return,
        };

        if let Ok(k) = result {
            self.set_status(format!("Pressed {}", k));
        }
    }

    pub fn _key_down(&mut self, _ctx: &mut Context, btn: Btn) -> GameResult<String> {
        Ok(format!("{:?}", btn))
    }
}
