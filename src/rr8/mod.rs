use std::collections::HashMap;

use ggez;
use ggez::error::GameResult;
use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Image, Rect};
use ggez::nalgebra::Point2;
use ggez::Context;
use graphics::{Color, FilterMode};

const FONT_PATH: &'static str = "/roguelike-font-16.png";

const FONT_MAP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÀÀÀÀÇÈÉÈÈÒÒÒÒÙÙÙÙabcdefghijklmnopqrstuvwxyzààààçè#%&@$.,!?:;'\"()[]*/\\+-<=> ";

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
        ctx: &mut Context,
        text: &str,
        color: &Color,
    ) -> GameResult<SpriteBatch> {
        let image = self.recolor_font(ctx, color)?;
        let mut batch = SpriteBatch::new(image);
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
                    .dest(Point2::new((column * 8) as f32, (line * 16) as f32)),
            );
            column += 1;
        }

        Ok(batch)
    }

    fn recolor_font(&self, ctx: &mut Context, color: &Color) -> GameResult<Image> {
        let color_tuple = color.to_rgba();
        let color_rgba = [color_tuple.0, color_tuple.1, color_tuple.2, color_tuple.3];

        let rgba: Vec<u8> = self
            .image
            .to_rgba8(ctx)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, &b)| if b == 255 { color_rgba[i % 4] } else { b })
            .collect();

        Image::from_rgba8(ctx, self.image.width(), self.image.height(), &rgba)
    }

    fn build_draw_param_from_char(&self, c: &char) -> DrawParam {
        let char_id = self.font_map.get(c).unwrap_or(&1023.);
        DrawParam::default().src(Rect::new(char_id * 8. / 1024., 0., 8. / 1024., 1.))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
