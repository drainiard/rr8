use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;

use rr8::{
    ui::{self, Scale, Ui},
    Game, GameMode,
};

const WIN_W: f32 = 320.;
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

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        ggez::input::mouse::set_cursor_hidden(ctx, true);
        self.ui.set_mouse_coords((x, y));
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let logo = keymods.contains(event::KeyMods::LOGO);

        println!("{:?}", (keymods, keycode));

        match self.game.mode {
            GameMode::Normal => {
                match keycode {
                    event::KeyCode::Escape => {
                        self.mode = MainMode::Switch;
                        self.game.mode = GameMode::Prompt;
                    }
                    event::KeyCode::Add => {
                        self.ui.set_scale(if logo { Scale::Max } else { Scale::Up })
                    }
                    event::KeyCode::Subtract => {
                        self.ui
                            .set_scale(if logo { Scale::Min } else { Scale::Down })
                    }
                    _ => self.game.key_down(ctx, keycode, keymods),
                };
            }
            GameMode::Prompt => {
                match keycode {
                    event::KeyCode::Return => self.game.run_prompt(),
                    event::KeyCode::Escape => self.game.mode = GameMode::Normal,
                    _ => self.game.key_down(ctx, keycode, keymods),
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

    let prompt = ui::Prompt::default();
    let top_bar = ui::TopBar::default();
    state.ui.add_system(prompt);
    state.ui.add_system(top_bar);

    event::run(ctx, event_loop, state)
}
