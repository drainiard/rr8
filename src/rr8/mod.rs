use std::collections::HashMap;

use ggez;
use ggez::error::GameResult;
use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::nalgebra::Point2;
use ggez::Context;
use graphics::{Color, FilterMode};

pub mod palette;

const TILESET_PATH: &'static str = "/roguelike-tiles.png";
const TILESET_ALT_PATH: &'static str = "/unreleased-mcnoodlor.png";
const FONT_PATH: &'static str = "/roguelike-font-16.png";
const FONT_MAP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÀÀÀÀÇÈÉÈÈÒÒÒÒÙÙÙÙabcdefghijklmnopqrstuvwxyzààààçè#%&@$.,!?:;'\"()[]*/\\+-<=> ";
const TILE_SIZE: u16 = 16;

pub struct Font {
    image: graphics::Image,
    font_map: HashMap<char, f32>,
}

impl Font {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::new(ctx, FONT_PATH)?;
        let font_map = FONT_MAP
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as f32))
            .collect();

        Ok(Self { image, font_map })
    }

    pub fn text_batch(
        &self,
        text: &str,
        color: impl Into<Color> + Copy,
    ) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());
        batch.set_filter(FilterMode::Nearest);

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
                    .dest(Point2::new((column * 8) as f32, (line * 16) as f32)),
            );
            column += 1;
        }

        Ok(batch)
    }

    fn build_draw_param_from_char(&self, c: &char) -> DrawParam {
        let char_id = self.font_map.get(c).unwrap_or(&1023.);
        DrawParam::default().src(Rect::new(char_id * 8. / 1024., 0., 8. / 1024., 1.))
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

pub struct TileLayout {
    path: &'static str,
    borders: Vec<[(u16, u16); 6]>,
}

pub struct TileMap {
    image: graphics::Image,
    layout: TileLayout,
}

impl TileMap {
    pub fn new(ctx: &mut Context, layout: TileLayout) -> GameResult<Self> {
        let image = Image::new(ctx, layout.path)?;
        Ok(Self { image, layout })
    }

    pub fn tile(
        &self,
        t: TileId,
        column: u16,
        c: impl Into<Color>,
        tile_width: u16,
    ) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());

        let rect = self.rect_with_tile_width(t as u16, column, tile_width);
        batch.add(DrawParam::default().color(c.into()).src(rect));

        Ok(batch)
    }

    pub fn textbox(
        &self,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
        variant: usize,
    ) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());
        batch.set_filter(FilterMode::Nearest);

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
    map: TileMap,
    map2: TileMap,
}

impl Ui {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
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
        let map = TileMap::new(ctx, layout)?;
        let map2 = TileMap::new(ctx, layout2)?;

        Ok(Self { map, map2 })
    }

    pub fn fill(
        &self,
        ctx: &mut Context,
        w: u8,
        h: u8,
        c: impl Into<Color> + Copy,
    ) -> GameResult<impl graphics::Drawable> {
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
}
