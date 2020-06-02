use crate::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct TopBar;

impl System for TopBar {
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> GameResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, game: &Game) -> GameResult {
        let default_color = Pal::Gray.dark();
        let ((bg_row, bg_column), (fg_row, fg_column), bg_color, fg_color) = match game.mode {
            GameMode::Normal => ((1, 16), (5, 16), Pal::Green.darker(), Pal::Green),
            GameMode::Prompt => ((1, 16), (1, 25), Pal::Orange.darker(), Pal::Orange),
        };
        game.ui.draw(
            ctx,
            &game.ui.tile_alt(bg_row, bg_column, fg_color, false)?,
            1.,
            0.,
        )?;
        game.ui.draw(
            ctx,
            &game.ui.tile_alt(fg_row, fg_column, bg_color, false)?,
            1.,
            0.,
        )?;
        game.ui
            .draw_text(ctx, &p(&game.mode).to_uppercase(), 2.5, 0., default_color)?;

        game.ui.draw_text(
            ctx,
            &format!("x{}", game.ui.scale),
            18.,
            0.,
            Pal::Gray.darker(),
        )?;

        Ok(())
    }
}
