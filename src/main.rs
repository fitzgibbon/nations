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
            &Rectangle::new((50, 50), (100, 200)),
            Background::Col(Color::RED),
        );
        Ok(())
    }
}

fn main() {
    run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
}
