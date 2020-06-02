use ggez;
use ggez::conf;
use ggez::event;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use graphics::Rect;
use rr8::{
    ui::{prompt::Prompt, topbar::TopBar, Scale, Ui},
    Game, GameMode, TILE_SIZE,
};

const WIN_W: f32 = 20. * TILE_SIZE as f32;
const WIN_H: f32 = 20. * TILE_SIZE as f32;

const WIN_SCALE: f32 = Scale::DEFAULT + Scale::DELTA * 2.;

enum MainMode {
    Ready,
    Switch,
}

struct MainState {
    game: Game,
    mode: MainMode,
    scale: f32,
    dt: u32,
}

impl MainState {
    fn new(ctx: &mut Context, scale: f32) -> GameResult<MainState> {
        let filter_mode = graphics::FilterMode::Nearest;
        let mode = MainMode::Ready;

        let win = Rect::new(0., 0., WIN_W, WIN_H);
        let ui = Ui::new(ctx, filter_mode, win, scale)?;
        let game = Game::new(ctx, ui)?;

        let s = MainState {
            game,
            mode,
            scale,
            dt: 0,
        };

        Ok(s)
    }

    fn switch_mode(&mut self, mode: GameMode) {
        self.game.mode = mode;
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, 60) {
            self.dt += 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, (0, 0, 0).into());

        let scale = self.game.ui.get_scale();
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
            graphics::set_screen_coordinates(ctx, Rect::new(0., 0., w, h))?;
            println!("Update Scale to {}", scale);
        }

        // UI has its own delta time for animations and stuff
        self.game.ui.dt = self.dt;

        self.game.ui.draw_all(ctx, &self.game)?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.game.ui.set_mouse_coords((x, y));
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let logo = keymods.contains(event::KeyMods::LOGO);

        println!("{:?}", (keymods, keycode));

        // Mode-independent keys
        let mut is_done = true;
        match keycode {
            event::KeyCode::F1 => self.switch_mode(GameMode::Normal),
            event::KeyCode::F2 => self.switch_mode(GameMode::Prompt),
            //event::KeyCode::Escape => ggez::event::quit(ctx),
            _ => {
                is_done = false;
            }
        };

        if is_done {
            return;
        }

        match self.game.mode {
            GameMode::Normal => {
                match keycode {
                    event::KeyCode::Key0 => self.game.ui.set_scale(Scale::Default),
                    event::KeyCode::Add => {
                        self.game
                            .ui
                            .set_scale(if logo { Scale::Max } else { Scale::Up })
                    }
                    event::KeyCode::Subtract => {
                        self.game
                            .ui
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

    fn text_input_event(&mut self, _ctx: &mut Context, c: char) {
        if let GameMode::Prompt = self.game.mode {
            self.game.update_prompt(c);
        }
    }
}

pub fn main() -> GameResult {
    let scale = WIN_SCALE;
    let mut cb = ContextBuilder::new("rr8", "rr8")
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

    let prompt = Prompt::default();
    let topbar = TopBar::default();
    state.game.ui.add_system(prompt);
    state.game.ui.add_system(topbar);

    event::run(ctx, event_loop, state)
}
