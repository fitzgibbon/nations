use crate::procgen::procseed::ProcSeed;
use crate::procgen::terrain::{Terrain, TiledWorldTerrain};

//use cursive::theme::Color;
use quicksilver::graphics::Color;

pub struct GameWorldOffset {
    x: i64,
    y: i64,
    z: i64,
}

impl GameWorldOffset {
    pub fn new(x: i64, y: i64, z: i64) -> GameWorldOffset {
        GameWorldOffset { x: x, y: y, z: z }
    }
}

pub trait GameWorld {
    fn get_origin(&self) -> GameWorldOffset;
    fn render_qs(&self, seed: &ProcSeed, offset: &GameWorldOffset) -> (Color, Color);
}

pub struct TiledGameWorld {
    width: usize,
    height: usize,
    terrain: TiledWorldTerrain,
}

impl TiledGameWorld {
    pub fn new(width: usize, height: usize) -> TiledGameWorld {
        TiledGameWorld {
            width: width,
            height: height,
            terrain: TiledWorldTerrain::new(),
        }
    }
}

impl GameWorld for TiledGameWorld {
    fn get_origin(&self) -> GameWorldOffset {
        GameWorldOffset { x: 0, y: 0, z: 0 }
    }

    fn render_qs(&self, seed: &ProcSeed, offset: &GameWorldOffset) -> (Color, Color) {
        let tv = vec![
            offset.x as f64 / self.width as f64,
            offset.y as f64 / self.height as f64,
        ];
        self.terrain.render_qs(seed, &tv)
    }
}
