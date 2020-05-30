use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use rr8::{palette::Pal, Font, TileId, Ui};

const WIN_W: f32 = 576.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = 2.;

struct MainState {
    font: Font,
    ui: Ui,
    scale: f32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let filter_mode = graphics::FilterMode::Linear;
        let font = Font::new(ctx, filter_mode)?;
        let ui = Ui::new(ctx, filter_mode)?;

        let s = MainState {
            font,
            ui,
            scale: WIN_SCALE,
        };

        Ok(s)
    }

    fn _draw(
        &mut self,
        ctx: &mut ggez::Context,
        drawable: &impl graphics::Drawable,
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            drawable,
            graphics::DrawParam::default()
                .dest(na::Point2::new(x * 16. * self.scale, y * 16. * self.scale))
                .scale(na::Vector2::new(self.scale, self.scale)),
        )
    }

    fn _draw_text(
        &mut self,
        ctx: &mut ggez::Context,
        text: &str,
        x: f32,
        y: f32,
        color: impl Into<graphics::Color>,
    ) -> ggez::GameResult {
        self._draw(ctx, &self.font.text_batch(text, color.into())?, x, y)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Pal::Black.darker());

        let mesh = self.ui.mesh(ctx, 18, 18, Pal::DarkBlue.darker())?;
        self._draw(ctx, &mesh, 0., 1.)?;

        let mesh = self.ui.mesh(ctx, 18, 18, Pal::DarkBlue.dark())?;
        self._draw(ctx, &mesh, 18., 1.)?;

        let fill = self
            .ui
            .fill8(TileId::Ico, 17, 1, 18, Pal::DarkBlue.darker())?;
        self._draw(ctx, &fill, 18., 1.)?;
        self._draw(ctx, &fill, 35.5, 1.)?;

        let mut cursor = 0.;
        let mut y = 19.;

        let loc = self.ui.tile(TileId::Overworld, 10, Pal::LightGray)?;
        self._draw(ctx, &loc, cursor / 2., y)?;
        cursor += 2.5; // account for full-width tile

        self._draw_text(ctx, "Arcadia", cursor / 2., y, Pal::LightGray)?;

        cursor = 0.;
        y = 0.;

        let health = self.ui.tile(TileId::Sym, 5, Pal::Red)?;
        self._draw(ctx, &health, cursor / 2., y)?;
        cursor += 2.5; // account for full-width tile

        self._draw_text(ctx, "99", cursor / 2., y, Pal::Red)?;
        cursor += 3.;

        let mana = self.ui.tile(TileId::Food2, 4, Pal::Purple)?;
        self._draw(ctx, &mana, cursor / 2., y)?;
        cursor += 2.5; // account for full-width tile

        self._draw_text(ctx, "23", cursor / 2., y, Pal::Purple)?;
        cursor += 3.;

        let medal = self.ui.tile8(TileId::Ico, 6, Pal::Yellow)?;
        self._draw(ctx, &medal, cursor / 2., y)?;
        cursor += 2.5;

        self._draw_text(ctx, "580", cursor / 2., y, Pal::Yellow)?;
        cursor += 5.;

        let crystal = self.ui.tile(TileId::Explore, 15, Pal::Green)?;
        self._draw(ctx, &crystal, cursor / 2., y)?;
        cursor += 2.5; // account for full-width tile

        self._draw_text(ctx, "34", cursor / 2., y, Pal::Green)?;

        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let mut cb = ggez::ContextBuilder::new("rr8", "rr8")
        .window_setup(
            conf::WindowSetup::default()
                .title("Retro Rust 8-bit IDE")
                .vsync(true),
        )
        .window_mode(conf::WindowMode::default().dimensions(WIN_W * WIN_SCALE, WIN_H * WIN_SCALE));

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
