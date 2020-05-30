use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;

use rr8::{palette::Pal, Game, TileId, Ui};

const WIN_W: f32 = 576.;
const WIN_H: f32 = 320.;

const WIN_SCALE: f32 = 2.;

struct MainState {
    game: Game,
    ui: Ui,
    dt: u8,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let filter_mode = graphics::FilterMode::Nearest;
        let game = Game::new(ctx)?;
        let scale = WIN_SCALE;
        let ui = Ui::new(ctx, filter_mode, scale)?;

        let s = MainState { game, ui, dt: 0 };

        Ok(s)
    }

    fn _draw(
        &mut self,
        ctx: &mut ggez::Context,
        drawable: &impl graphics::Drawable,
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        self.ui.draw(ctx, drawable, x, y)
    }

    fn _draw_text(
        &mut self,
        ctx: &mut ggez::Context,
        text: &str,
        x: f32,
        y: f32,
        color: impl Into<graphics::Color>,
    ) -> ggez::GameResult {
        self._draw(ctx, &self.ui.text_batch(text, color.into())?, x, y)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, Pal::Black.darker());

        self.ui.bg(ctx)?;

        let clock_tiles = &[
            (6, 30),
            (5, 31),
            (5, 31),
            (7, 30),
            (7, 31),
            (7, 31),
            (9, 30),
            (9, 31),
            (9, 31),
            (9, 28),
            (9, 29),
            (9, 29),
            (5, 30),
            (10, 29),
            (10, 29),
            (10, 28),
            (10, 31),
            (10, 31),
            (10, 30),
            (8, 31),
            (8, 31),
            (8, 30),
            (6, 31),
            (6, 31),
        ];
        let mut hour = 24 * self.dt as u16 / 0xff; // convert u16 into 24h
        let (clock_row, clock_column) = clock_tiles.get(hour as usize).unwrap_or(&(10, 20));
        let (meridiem, time_color) = if hour < 12 {
            ("PM", Pal::Pink)
        } else {
            ("AM", Pal::Peach)
        };
        // convert to AM/PM format
        hour = hour % 12;
        if hour < 1 {
            hour = 12
        }

        let clock_tile = self.ui.tile_alt(*clock_row, *clock_column, time_color)?;
        self._draw(ctx, &clock_tile, 24., 0.)?;
        self._draw_text(
            ctx,
            &format!("{:>2?}{}", hour, meridiem),
            25.,
            0.,
            time_color,
        )?;

        self._draw(ctx, &self.ui.tile_alt(21, 24, Pal::LightGray)?, 9., 19.)?;
        self._draw_text(ctx, &self.game.status.clone(), 10., 19., Pal::LightGray)?;

        let mut cursor = 18.;
        let y = 0.;

        let health = self.ui.tile(TileId::Sym, 5, Pal::Red)?;
        self._draw(ctx, &health, cursor / 2., y)?;
        cursor += 2.5;

        self._draw_text(ctx, "99", cursor / 2., y, Pal::Red)?;
        cursor += 3.;

        let mana = self.ui.tile(TileId::Food2, 4, Pal::Purple)?;
        self._draw(ctx, &mana, cursor / 2., y)?;
        cursor += 2.5;

        self._draw_text(ctx, "23", cursor / 2., y, Pal::Purple)?;
        cursor += 3.;

        let medal = self.ui.tile8(TileId::Ico, 6, Pal::Yellow)?;
        self._draw(ctx, &medal, cursor / 2., y)?;
        cursor += 2.5;

        self._draw_text(ctx, "580", cursor / 2., y, Pal::Yellow)?;
        cursor += 5.;

        let crystal = self.ui.tile(TileId::Explore, 15, Pal::Green)?;
        self._draw(ctx, &crystal, cursor / 2., y)?;
        cursor += 2.5;

        self._draw_text(ctx, "34", cursor / 2., y, Pal::Green)?;

        graphics::present(ctx)?;

        std::thread::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Escape => ggez::event::quit(ctx),
            k => self.game.key_down(ctx, k),
        };
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
