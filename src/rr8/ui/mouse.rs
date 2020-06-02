use crate::*;

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
        if let GameMode::Normal = game.mode {
            let (x, y) = self.coords;

            let mouse = game.ui.tile_alt(9, 16, Pal::Orange, false)?;
            let mouse_inner = game.ui.tile_alt(9, 19, Pal::Red, false)?;
            let mouse_shadow = game.ui.tile_alt(9, 19, Pal::Black, false)?;
            game.ui
                .draw_free(ctx, &mouse_shadow, x + 1., y + 2., game.ui.scale)?;
            game.ui.draw_free(ctx, &mouse_inner, x, y, game.ui.scale)?;
            game.ui.draw_free(ctx, &mouse, x, y, game.ui.scale)?;
        }

        Ok(())
    }
}
