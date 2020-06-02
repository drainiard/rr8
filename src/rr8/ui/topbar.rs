use crate::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct TopBar;

impl System for TopBar {
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> GameResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, game: &Game) -> GameResult {
        let ui = &game.ui;

        let default_color = Pal::DarkBlue;
        let ((bg_row, bg_column), (fg_row, fg_column), bg_color, fg_color) = match game.mode {
            GameMode::Normal => ((1, 16), (5, 16), default_color.darker(), default_color),
            GameMode::Prompt => ((1, 16), (1, 25), default_color.darker(), default_color),
        };
        ui.draw(
            ctx,
            &ui.tile_alt(bg_row, bg_column, fg_color, false)?,
            1.,
            0.,
        )?;
        ui.draw(
            ctx,
            &ui.tile_alt(fg_row, fg_column, bg_color, false)?,
            1.,
            0.,
        )?;
        ui.draw_text(ctx, &p(&game.mode).to_uppercase(), 2.5, 0., default_color)?;

        ui.draw_text(ctx, &format!("x{}", ui.scale), 18., 0., default_color)?;

        Ok(())
    }
}
