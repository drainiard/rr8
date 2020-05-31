use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::graphics::{Color, FilterMode};
use ggez::nalgebra::Point2;
use ggez::Context;

use crate::*;

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

impl TileLayout {
    pub(crate) fn new(path: &'static str, borders: Vec<[(u16, u16); 6]>) -> Self {
        Self { path, borders }
    }
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
