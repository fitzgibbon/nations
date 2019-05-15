use nalgebra::Vector3;
use std::cmp::{max, min, Ordering};

/// We use cube coordinates as described at https://www.redblobgames.com/grids/hexagons/ and https://www.redblobgames.com/grids/hexagons/implementation.html
trait Coord {
    type Scalar;
    fn cmp_len(&self, length: Self::Scalar) -> Ordering;
    fn manhattan(&self) -> Self::Scalar;
    fn manhattan_iter(length: Self::Scalar) -> Box<Iterator<Item = Self>>;
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

    fn manhattan(&self) -> Self::Scalar {
        (self.x.abs() + self.y.abs() + self.z.abs()) / 2
    }

    fn manhattan_iter(length: Self::Scalar) -> Box<Iterator<Item = Self>> {
        Box::new(HexManhattanIterator::new(length))
    }
}

struct HexManhattanIterator {
    x: isize,
    y: isize,
    length: isize,
}

impl HexManhattanIterator {
    fn new(length: isize) -> Self {
        HexManhattanIterator {
            x: -length,
            y: 0,
            length,
        }
    }
}

impl Iterator for HexManhattanIterator {
    type Item = Hex;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        if self.y > min(self.length, self.length - self.x) {
            self.x += 1;
            self.y = max(-self.length, -self.x - self.length)
        }
        if self.x <= self.length {
            result = Some(Hex::new(self.x, self.y, -self.x - self.y));
        }
        self.y += 1;
        result
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
        println!("manhattan a -> b: {}", (b - a).manhattan());
        assert_eq!((b - a).manhattan(), 7);
    }

    #[test]
    fn walk_manhattan() {
        println!(
            "{}",
            HexManhattanIterator::new(2)
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
