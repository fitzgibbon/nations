use nalgebra::Vector3;

/// We use cube coordinates as described at https://www.redblobgames.com/grids/hexagons/ and https://www.redblobgames.com/grids/hexagons/implementation.html
trait HexCoord {
    fn new(q: isize, r: isize, s: isize) -> Self;
}

impl HexCoord for Vector3<isize> {
    fn new(q: isize, r: isize, s: isize) -> Self {
        assert_eq!(q + r + s, 0);
        Vector3::new(q, r, s)
    }
}

trait HexTiling {
    type HexCoord;

    fn new() -> Self;
    fn origin() -> Self::HexCoord;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn make_valid_hex_coordinate() {
        let _pos: Vector3<isize> = HexCoord::new(2, 3, -5);
    }
}
