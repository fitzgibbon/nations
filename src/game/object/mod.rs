// cursive::theme::Color;
use quicksilver::graphics::Color;

pub trait GameObject {}

pub trait VisibleGameObject: GameObject {
    fn render_cursive(&self) -> (char, Color, Color);
}

pub trait PhysicalGameObject: GameObject {}

pub trait GroundObject: VisibleGameObject {}
