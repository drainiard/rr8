use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;

use rr8::{ui::Ui, Game, GameMode};

const WIN_W: f32 = 576.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = 1.;

enum MainMode {
    Ready,
    Switch,
}

struct MainState {
    game: Game,
    mode: MainMode,
    ui: Ui,
    dt: u32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let filter_mode = graphics::FilterMode::Nearest;
        let game = Game::new(ctx)?;
        let mode = MainMode::Ready;
        let scale = WIN_SCALE;
        let ui = Ui::new(ctx, filter_mode, scale)?;

        let s = MainState {
            game,
            mode,
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
        match self.game.mode {
            GameMode::Normal => {
                match keycode {
                    event::KeyCode::Backslash => {
                        self.mode = MainMode::Switch;
                        self.game.mode = GameMode::Prompt;
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
