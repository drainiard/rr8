use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use rr8::Font;

const WIN_W: f32 = 400.;
const WIN_H: f32 = 320.;

struct MainState {
    font: Font,
    scale: f32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let font = Font::new(ctx)?;

        let s = MainState { font, scale: 2. };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.13, 0.12, 0.12, 1.0].into());

        // let batch = self.font.text_batch("fn init() {\n spr(242, 16, 40);\n}\n");

        let batch = self.font.text_batch(
            ctx,
            "Hey, you!\nYou're finally awake?",
            &graphics::Color::from_rgb(255, 255, 0),
        )?;

        // graphics::queue_text(
        //     ctx,
        //     &batch,
        //     na::Point2::new(8., 8.),
        //     Some(graphics::Color::from_rgb(200, 160, 80)),
        // );

        graphics::draw(
            ctx,
            &batch,
            graphics::DrawParam::default()
                .dest(na::Point2::new(
                    16. * self.scale,
                    WIN_H - 16. * 3. * self.scale,
                ))
                .scale(na::Vector2::new(self.scale, self.scale)),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let mut cb = ggez::ContextBuilder::new("rr8", "rr8")
        .window_setup(conf::WindowSetup::default().title("Retro Rust 8-bit IDE"))
        .window_mode(conf::WindowMode::default().dimensions(WIN_W, WIN_H));

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
