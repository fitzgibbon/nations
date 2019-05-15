use nalgebra::Vector3;
use std::cmp::Ordering;

/// We use cube coordinates as described at https://www.redblobgames.com/grids/hexagons/ and https://www.redblobgames.com/grids/hexagons/implementation.html
trait Coord {
    type Scalar;
    fn cmp_len(&self, length: Self::Scalar) -> Ordering;
}

type Hex = Vector3<isize>;

impl Coord for Hex {
    type Scalar = isize;
    fn cmp_len(&self, length: Self::Scalar) -> Ordering {
        let asq = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        let bsq = length * length;
        if asq < bsq {
            Ordering::Less
        } else if asq == bsq {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
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
        let a = Hex::new(2, 3, -5);
        let b = Hex::new(-3, 1, 2);
        println!("a -> b: {}", b - a);
        println!(
            "length compare to 8: {:?}, to 9: {:?}",
            (b - a).cmp_len(8),
            (b - a).cmp_len(9)
        );
        assert_eq!(b - a, Hex::new(-5, -2, 7));
        assert_eq!((b - a).cmp_len(8), Ordering::Greater);
        assert_eq!((b - a).cmp_len(9), Ordering::Less);
    }
}
