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
    // ===============
    // Width = 16-bit
    // ===============
    // 13 + 13 + 7
    Chars = 1,
    // 12
    Expr = 6,
    // 13
    Fauna = 9,
    // 4
    Trolls = 12,
    // 8
    Unliving = 15,
    // 8
    Creatures = 18,
    // 16 + 7 + 12
    Building = 21,
    // 12
    Devices = 26,
    // 13
    Overworld = 29,
    // 16 + 5
    Explore = 32,
    // 16 + 6
    Food = 36,
    // 16 + 11
    Outfit = 40,
    // 15
    Magick = 44,
    // 6
    Music = 47,
    // 16 + 7
    Sym = 50,
    // ===============
    // Width = 8-bit
    // ===============
    // 10
    Num = 54,
    // 32 + 11
    Font = 55,
    // Ico
    Ico = 60,
}

pub struct TileMap {
    image: graphics::Image,
}

impl TileMap {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::new(ctx, TILESET_PATH)?;
        Ok(Self { image })
    }

    pub fn tile<C: Into<Color>>(&self, t: TileId, r: u16, o: u16, c: C) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());

        let rect = self.rect(t, r, o);
        batch.add(DrawParam::default().color(c.into()).src(rect));

        Ok(batch)
    }

    pub fn textbox<C: Into<Color> + Copy>(&self, w: u8, h: u8, c: C) -> GameResult<SpriteBatch> {
        let mut batch = SpriteBatch::new(self.image.clone());
        batch.set_filter(FilterMode::Nearest);

        let w_px = (w as f32 - 1.) * TILE_SIZE as f32;
        let h_px = (h as f32 - 1.) * TILE_SIZE as f32;

        let corner_positions = vec![(0., 0.), (w_px, 0.), (0., h_px), (w_px, h_px)];
        for (o, (x, y)) in (3..8).zip(corner_positions) {
            let rect = self.rect(TileId::Building, 2, o);
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
                let rect = self.rect(TileId::Building, 2, 0);
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
                let rect = self.rect(TileId::Building, 2, 1);
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

    pub fn rect(&self, tid: TileId, row: u16, offset: u16) -> Rect {
        let x = offset * TILE_SIZE;
        let y = (tid as u16 + row) * TILE_SIZE;

        self.rect_raw(x, y, TILE_SIZE, TILE_SIZE)
    }

    fn rect_raw<N: Into<f32>>(&self, x: N, y: N, w: N, h: N) -> Rect {
        let iw = self.image.width() as f32;
        let ih = self.image.height() as f32;

        Rect::new(x.into() / iw, y.into() / ih, w.into() / iw, h.into() / ih)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
