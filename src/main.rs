#[macro_use]
extern crate derive_more;
mod geometry;

use quicksilver::{
    geom::{Rectangle, Vector},     // We'll need to import Rectangle now
    graphics::{Background, Color}, // Also Background and Color
    lifecycle::{run, State, Window},
    Result,
};

struct Screen;

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Clear the contents of the window to a white background
        window.clear(Color::WHITE)?;
        // Draw a red rectangle
        window.draw(
            &Rectangle::new((200, 200), (200, 200)),
            Background::Col(Color::RED),
        );
        // Draw a green hexagon
        window.draw(
            &geometry::HexShape::with_size(Vector::new(300, 300), Vector::new(200, 200)),
            Background::Col(Color::GREEN),
        );

        geometry::HexManhattanIterator::new(9)
            .map(|x| {
                geometry::HexShape::with_size_on_grid(x, Vector::new(400, 300), Vector::new(30, 30))
            })
            .enumerate()
            .for_each(|(i, x)| {
                window.draw(
                    &x,
                    Background::Col(Color::BLUE.with_alpha((i % 16) as f32 * (0.5 / 16.0) + 0.5)),
                )
            });

        Ok(())
    }
}

fn main() {
    run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
}
