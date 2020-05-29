use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use rr8::{palette::Pal, Font, TileId, Ui};

const WIN_W: f32 = 576.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = 1.25;

struct MainState {
    font: Font,
    ui: Ui,
    scale: f32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let font = Font::new(ctx)?;
        let ui = Ui::new(ctx)?;

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
        pal: Pal,
    ) -> ggez::GameResult {
        self._draw(ctx, &self.font.text_batch(text, pal)?, x, y)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Pal::Black.darker());

        let fill = self.ui.fill(ctx, 18, 18, Pal::DarkBlue.darker())?;
        self._draw(ctx, &fill, 0., 1.)?;

        let fill = self.ui.fill(ctx, 18, 18, Pal::DarkBlue.dark())?;
        self._draw(ctx, &fill, 18., 1.)?;

        let mut cursor = 0.;

        let health = self.ui.tile(TileId::Sym, 5, Pal::Red)?;
        self._draw(ctx, &health, cursor / 2., 19.)?;
        cursor += 1.5; // account for full-width tile

        for d in [9, 9].iter() {
            cursor += 1.;
            let digit = self.ui.tile8(TileId::Num, *d, Pal::White)?;
            self._draw(ctx, &digit, cursor / 2., 19.)?;
        }
        cursor += 3.;

        let mana = self.ui.tile(TileId::Food2, 4, Pal::Purple)?;
        self._draw(ctx, &mana, cursor / 2., 19.)?;
        cursor += 1.5; // account for full-width tile

        for d in [2, 3].iter() {
            cursor += 1.;
            let digit = self.ui.tile8(TileId::Num, *d, Pal::White)?;
            self._draw(ctx, &digit, cursor / 2., 19.)?;
        }
        cursor += 3.;

        let loc = self.ui.tile(TileId::Overworld, 10, Pal::Brown)?;
        self._draw(ctx, &loc, cursor / 2., 19.)?;
        cursor += 1.5; // account for full-width tile

        cursor += 1.;
        let text = "Arcadia";
        let location = self.font.text_batch(text, Pal::White)?;
        self._draw(ctx, &location, cursor / 2., 19.)?;
        cursor += text.chars().count() as f32 * 2.;

        cursor = 0.;
        let medal = self.ui.tile8(TileId::Ico, 6, Pal::Yellow)?;
        self._draw(ctx, &medal, cursor / 2., 0.)?;
        cursor += 0.5;

        for d in [5, 3, 0].iter() {
            cursor += 1.;
            let digit = self.ui.tile8(TileId::Num, *d, Pal::White)?;
            self._draw(ctx, &digit, cursor / 2., 0.)?;
        }
        cursor += 5.;

        let crystal = self.ui.tile(TileId::Explore, 15, Pal::Green)?;
        self._draw(ctx, &crystal, cursor / 2., 0.)?;
        cursor += 1.5; // account for full-width tile

        for d in [3, 4].iter() {
            cursor += 1.;
            let digit = self.ui.tile8(TileId::Num, *d, Pal::White)?;
            self._draw(ctx, &digit, cursor / 2., 0.)?;
        }

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
