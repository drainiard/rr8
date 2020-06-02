use crate::*;
use ggez::graphics::Color;

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
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> GameResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, game: &Game) -> GameResult {
        let ui = &game.ui;

        // get coords in tile units (instead of float pixels), adjusted for the top and side shifting
        let pixel_to_tile_coord = |coord: f32, offset: f32| -> f32 {
            ((coord / ui.scale) / TILE_SIZE as f32 - offset) as u32 as f32
        };

        if let GameMode::Normal = game.mode {
            let (x, y) = self.coords;

            let (ox, oy) = (0., 1.);
            // tiles origin is offset wrt mouse coords
            let dx = pixel_to_tile_coord(x, ox);
            let dy = pixel_to_tile_coord(y, oy);

            if 0. <= dx && dx < 20. && 0. <= dy && dy < 18. {
                // viewport is Rect::new(1.,1.,18.,18.);
                let (tx, ty) = (0, 1);
                let mut hover_color: Color = Pal::Red.into();
                hover_color.a = 0.5;

                let tile = ui.tile_alt(tx, ty, hover_color, false)?;
                ui.draw(ctx, &tile, dx + ox, dy + oy)?;

                ui.draw_text(ctx, &p((dx, dy, x, y)), 1., 19., Pal::DarkBlue)?;
            }
        }

        Ok(())
    }
}
