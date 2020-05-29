use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use rr8::{palette::Pal, Font, TileMap};

const WIN_W: f32 = 288.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = 2.;

struct MainState {
    font: Font,
    map: TileMap,
    scale: f32,
    pos: (u8, u8),
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let font = Font::new(ctx)?;
        let map = TileMap::new(ctx)?;

        let s = MainState {
            font,
            map,
            scale: WIN_SCALE,
            pos: (4, 4),
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
                .dest(na::Point2::new(x * 16., (16. * y) * self.scale))
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
        graphics::clear(ctx, Pal::DarkBlue.darker());

        let text = "Hey, you!\nYou're finally awake?";
        self._draw_text(ctx, text, 2., 17., Pal::White)?;

        self._draw(ctx, &self.map.textbox(18, 4, Pal::Gray)?, 0., 16.)?;

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
