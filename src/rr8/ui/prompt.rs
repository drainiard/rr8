use crate::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Prompt;

impl System for Prompt {
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> GameResult {
        todo!()
    }
    fn draw(&self, ctx: &mut Context, game: &Game) -> GameResult {
        let ui = &game.ui;

        if let GameMode::Prompt = &game.mode {
            // GameMode::Normal => (Pal::Gray.dark(), game.get_status()),
            let column = if ui.dt & 0b100000 > 0 { 15 } else { 20 };
            let beam = ui.tile8(TileId::Ico, column, Pal::Red, false)?;

            let (cursor_pos, prompt) = game.get_prompt();

            // this also works nice because drawing the beam before the
            // prompt makes the char underneath it visible
            ui.draw(ctx, &beam, 2. + cursor_pos as f32 / 2., 19.)?;

            let (prompt_color, prompt_text) = (Pal::Blue, prompt);

            ui.draw_text(ctx, "#", 1., 19., Pal::Gray.dark())?;
            ui.draw_text(ctx, prompt_text, 2., 19., prompt_color)?;
        }

        Ok(())
    }
}
