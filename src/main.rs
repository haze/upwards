#[macro_use]
extern crate getset;
extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;


#[derive(Clone, Debug, Getters, Default)]
struct Stat<T> where T: Clone + std::fmt::Debug {
    #[get] max: T,
    #[get] min: T,
    #[get] current: T
}

#[derive(Clone, Debug, Default)]
struct BaseStat {
    health: Stat<i32>,
    speed: Stat<f32>,
    shield: Option<Stat<i32>>,
}

#[derive(Clone, Debug, Getters)]
struct Player {
    #[get] stats: BaseStat,
    #[get] name: String,
}

struct GameState {
    player: Player,
}

impl GameState {
    fn new(_ctx: &mut Context, player_name: String)
    -> GameState {
        GameState {
            player: Player {
                name: player_name,
                stats: BaseStat::default()
            }
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         graphics::DrawMode::Fill,
                         graphics::Point2::new(5.0, 380.0),
                         100.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    
    let cb = ggez::ContextBuilder::new("helloworld", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup{
            title: "upWards".to_owned(),
            icon: "".to_owned(),
            allow_highdpi: true,
            resizable: false,
            .. Default::default()
        });
    let ctx = &mut cb.build().expect("Failed to create game context");

    let state = &mut GameState::new(ctx, "Haze".into());
    event::run(ctx, state).unwrap();
}