#[macro_use]
extern crate derive_more;
mod geometry;
mod procgen;
use procgen::noise::Noise;

use quicksilver::{
    geom::{Rectangle, Vector},     // We'll need to import Rectangle now
    graphics::{Background, Color}, // Also Background and Color
    lifecycle::{run, State, Window},
    Result,
};

struct Screen {
    seed: procgen::procseed::ProcSeed,
    height_map: procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            seed: procgen::procseed::ProcSeed::new(&0u32, 0.0),
            height_map: procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise::new(
                2, 4, 0.7, 1.0, 0.5,
            ),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.seed.skew += 0.005;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Clear the contents of the window to a white background
        window.clear(Color::WHITE)?;

        geometry::HexManhattanIterator::new(15)
            .map(|x| geometry::HexShape::with_radius_on_grid(x, Vector::new(400, 300), 10.0))
            .enumerate()
            .for_each(|(i, x)| {
                window.draw(
                    &x,
                    Background::Col(Color::BLUE.with_alpha(
                        self.height_map.get_noise(
                            &self.seed,
                            &vec![(x.pos.x / 150.0) as f64, (x.pos.y / 150.0) as f64],
                        ) as f32
                            * 0.5
                            + 0.5,
                    )),
                )
            });

        Ok(())
    }
}

fn main() {
    run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
}
