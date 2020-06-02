use std::collections::HashMap;

use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::graphics::{Color, FilterMode};
use ggez::nalgebra::Point2;

use crate::*;

#[derive(Debug)]
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
