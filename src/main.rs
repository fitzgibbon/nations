#[macro_use]
extern crate derive_more;
mod game;
mod geometry;
mod procgen;
use game::world::GameWorld;
use procgen::noise::Noise;

use quicksilver::{
    geom::{Rectangle, Vector},     // We'll need to import Rectangle now
    graphics::{Background, Color}, // Also Background and Color
    lifecycle::{run, State, Window},
    Result,
};

struct Screen {
    seed: procgen::procseed::ProcSeed,
    world: game::world::TiledGameWorld,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            seed: procgen::procseed::ProcSeed::new(&0u32, 0.0),
            world: game::world::TiledGameWorld::new(800, (800.0 * 0.75) as usize),
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.seed.skew += 0.005;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Clear the contents of the window to a white background
        window.clear(Color::WHITE)?;

        geometry::HexManhattanIterator::new(20)
            .map(|x| geometry::HexShape::with_radius_on_grid(x, Vector::new(400, 400), 10.0))
            .enumerate()
            .for_each(|(_i, x)| {
                window.draw(
                    &x,
                    Background::Col(
                        self.world
                            .render_qs(
                                &self.seed,
                                &game::world::GameWorldOffset::new(
                                    (x.pos.x - 400.0) as i64,
                                    (x.pos.y - 400.0) as i64,
                                    0,
                                ),
                            )
                            .1,
                    ),
                )
            });

        Ok(())
    }
}

fn main() {
    run::<Screen>("Hello World", Vector::new(800, 800), Default::default());
}
