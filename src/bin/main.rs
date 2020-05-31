use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;

use rr8::{
    ui::{Scale, Ui},
    Game, GameMode,
};

const WIN_W: f32 = 576.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = Scale::DEFAULT + Scale::DELTA * 2.;

enum MainMode {
    Ready,
    Switch,
}

struct MainState {
    game: Game,
    mode: MainMode,
    scale: f32,
    ui: Ui,
    dt: u32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context, scale: f32) -> ggez::GameResult<MainState> {
        let filter_mode = graphics::FilterMode::Nearest;
        let game = Game::new(ctx)?;
        let mode = MainMode::Ready;
        let ui = Ui::new(ctx, filter_mode, scale)?;

        let s = MainState {
            game,
            mode,
            scale,
            ui,
            dt: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, 60) {
            self.dt += 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, (0, 0, 0).into());

        let scale = self.ui.get_scale();
        if self.scale != scale {
            self.scale = scale;
            let (w, h) = (WIN_W * scale, WIN_H * scale);
            graphics::set_mode(
                ctx,
                conf::WindowMode::default()
                    .dimensions(w, h)
                    .min_dimensions(w, h)
                    .max_dimensions(w, h)
                    .resizable(true),
            )?;
            graphics::set_screen_coordinates(ctx, graphics::Rect::new(0., 0., w, h))?;
            println!("Update Scale to {}", scale);
        }

        // UI has its own delta time for animations and stuff
        self.ui.dt = self.dt;

        self.ui.draw_all(ctx, &self.game)?;

        graphics::present(ctx)?;

        std::thread::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let alt = keymods.contains(event::KeyMods::ALT);
        let ctrl = keymods.contains(event::KeyMods::CTRL);
        let logo = keymods.contains(event::KeyMods::LOGO);
        let shift = keymods.contains(event::KeyMods::SHIFT);

        match self.game.mode {
            GameMode::Normal => {
                match keycode {
                    event::KeyCode::Backslash => {
                        self.mode = MainMode::Switch;
                        self.game.mode = GameMode::Prompt;
                    }
                    event::KeyCode::Key0 => {
                        if ctrl {
                            self.ui.set_scale(Scale::Default)
                        }
                    }
                    event::KeyCode::Add => {
                        self.ui.set_scale(if logo { Scale::Max } else { Scale::Up })
                    }
                    event::KeyCode::Subtract => {
                        self.ui
                            .set_scale(if logo { Scale::Min } else { Scale::Down })
                    }
                    event::KeyCode::Escape => ggez::event::quit(ctx),
                    k => self.game.key_down(ctx, keycode, keymods),
                    // k => println!("Pressed {:?}", k),
                };
            }
            GameMode::Prompt => {
                match keycode {
                    event::KeyCode::Return => self.game.run_prompt(),
                    event::KeyCode::Escape => self.game.mode = GameMode::Normal,
                    k => self.game.key_down(ctx, keycode, keymods),
                    // k => println!("Pressed {:?}", k),
                };
            }
        }
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, c: char) {
        if let MainMode::Switch = self.mode {
            self.mode = MainMode::Ready;
            return;
        }

        if let GameMode::Prompt = self.game.mode {
            self.game.update_prompt(c);
        }
    }
}

pub fn main() -> ggez::GameResult {
    let scale = WIN_SCALE;
    let mut cb = ggez::ContextBuilder::new("rr8", "rr8")
        .window_setup(
            conf::WindowSetup::default()
                .title("Retro Rust 8-bit IDE")
                .vsync(true),
        )
        .window_mode(conf::WindowMode::default().dimensions(WIN_W * scale, WIN_H * scale));

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx, scale)?;
    event::run(ctx, event_loop, state)
}
