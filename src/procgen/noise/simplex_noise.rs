use crate::procgen::procseed::ProcSeed;
use std::cmp::Ordering;
use std::f64::consts::PI;

pub struct SimplexNoise {
    pub dim: usize,
    pub skew: f64,
    pub unskew: f64,
    pub side_len: f64,
    pub corner_to_face_sq: f64,
    pub grads: Vec<Vec<f64>>,
    pub value_scalar: f64,
}

impl SimplexNoise {
    pub fn new(dim: usize) -> SimplexNoise {
        let skew = if dim > 1 {
            ((dim as f64 + 1.0).sqrt() - 1.0) / dim as f64
        } else {
            0.0
        };
        let unskew = if dim > 1 {
            skew / (dim as f64 * skew + 1.0)
        } else {
            1.0
        };
        let side_len = (dim as f64).sqrt() / (dim as f64 * skew + 1.0);
        let a = (side_len.powi(2) - (side_len / 2.0).powi(2)).sqrt();
        let corner_to_face = if dim == 1 {
            side_len
        } else if dim == 2 {
            a
        } else {
            (a.powi(2) + (a / 2.0).powi(2)).sqrt()
        };
        let value_scalar = if dim > 1 {
            (1.0 / (dim as f64 - 1.0).sqrt())
                * (100.0 / ((dim as f64 - 1.0).powi(3) * (dim as f64 - 1.0).sqrt()) + 13.0)
        } else {
            1.0
        };

        let combinations: Vec<Vec<f64>> = (0..(2 as i32).pow((dim - 1) as u32))
            .map(|i| {
                (0..(dim - 1))
                    .map(|j| ((i >> j) & 1) as f64 * 2.0 - 1.0)
                    .collect()
            })
            .collect();

        // all possible gradients in our number of dimensions with one zero coordinate
        let grads = if dim > 1 {
            (0..(dim * combinations.len()))
                .map(|i| {
                    [
                        combinations.get(i % combinations.len()).unwrap()[..i / combinations.len()]
                            .to_vec(),
                        vec![0.0],
                        combinations.get(i % combinations.len()).unwrap()[i / combinations.len()..]
                            .to_vec(),
                    ]
                    .iter()
                    .fold(Vec::new(), |mut acc, v| {
                        acc.extend(v);
                        acc
                    })
                })
                .collect()
        } else {
            vec![vec![-1.0], vec![1.0]]
        };

        //println!("gradients for {} dimensions: {:?}", dim, grads);

        SimplexNoise {
            dim: dim,
            skew: skew,
            unskew: unskew,
            side_len: side_len,
            corner_to_face_sq: corner_to_face.powi(2),
            grads: grads,
            value_scalar: value_scalar,
        }
    }

    fn get(&self, seed: &ProcSeed, point: &Vec<f64>) -> (f64, Vec<f64>) {
        // skew factor
        let mut s = 0.0;
        for x in point.iter() {
            s += *x;
        }
        s *= self.skew;

        // skew input point
        let skewed_int_point: Vec<i64> = (0..self.dim)
            .map(|i| (point.get(i).unwrap() + s).floor() as i64)
            .collect();

        // unskew factor
        let mut t = 0.0;
        for x in skewed_int_point.iter() {
            t += *x as f64;
        }
        t *= self.unskew;
        //println!("t: {}", t);

        // unskew displacement from hypercube origin
        let unskewed_displacement: Vec<f64> = (0..self.dim)
            .map(|i| point.get(i).unwrap() - *skewed_int_point.get(i).unwrap() as f64 + t)
            .collect();

        // sort axes in descending order of displacement from hypercube origin
        let mut axis_order: Vec<usize> = (0..self.dim).map(|i| i).collect();
        axis_order.sort_by(|a, b| {
            if unskewed_displacement.get(*b).unwrap() < unskewed_displacement.get(*a).unwrap() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // iterate through each vertex in the simplex, applying contribution
        let mut noise = 0.0;
        let mut der: Vec<f64> = (0..self.dim).map(|_| 0.0).collect();
        let mut unskew_total = 0.0;
        let mut vertex = skewed_int_point.clone();
        for i in 0..(self.dim + 1) {
            if i != 0 {
                *vertex.get_mut(*axis_order.get(i - 1).unwrap()).unwrap() += 1;
            }
            let u: Vec<f64> = (0..self.dim)
                .map(|i| {
                    unskewed_displacement.get(i).unwrap()
                        - (*vertex.get(i).unwrap() - *skewed_int_point.get(i).unwrap()) as f64
                        + unskew_total
                })
                .collect();
            let mut attenuation = self.corner_to_face_sq;
            for x in u.iter() {
                attenuation -= x.powi(2);
            }
            if attenuation > 0.0 {
                let vseed = seed.derive(&vertex);
                let hash = vseed.get();
                let hash_fract = (hash as u64) as f64 / u64::max_value() as f64;
                let hash_index = (hash_fract * self.grads.len() as f64).floor() as usize;
                let grad = self.grads.get(hash_index).unwrap();

                let mut dotprod = 0.0;
                for j in 0..self.dim {
                    dotprod += grad.get(j).unwrap() * u.get(j).unwrap();
                }
                noise += dotprod * attenuation.powi(4);
                der = (0..self.dim)
                    .map(|i| {
                        der.get(i).unwrap() + grad.get(i).unwrap() * attenuation.powi(4)
                            - (dotprod * 8.0 * attenuation.powi(3)) * u.get(i).unwrap()
                    })
                    .collect();
            }
            unskew_total += self.unskew;
        }
        noise *= self.value_scalar;
        der = (0..self.dim)
            .map(|i| der.get(i).unwrap() * self.value_scalar)
            .collect();
        return (noise, der);
    }
}

impl super::Noise for SimplexNoise {
    fn get_noise(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        let (noise, _) = self.get(seed, &point.clone());
        return noise;
    }
}

pub struct OctavedSimplexNoise {
    pub dim: usize,
    pub octaves: Vec<SimplexNoise>,
    pub octave_factor: f64,
}

impl OctavedSimplexNoise {
    pub fn new(dim: usize, num_octaves: usize, octave_factor: f64) -> OctavedSimplexNoise {
        let octaves = (0..num_octaves).map(|_| SimplexNoise::new(dim)).collect();

        OctavedSimplexNoise {
            dim: dim,
            octaves: octaves,
            octave_factor: octave_factor,
        }
    }
}

impl super::Noise for OctavedSimplexNoise {
    fn get_noise(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        let noise: f64 = (0..self.octaves.len())
            .map(|i| {
                let mul = self.octave_factor.powi(i as i32);
                let (oct_noise, _) = self.octaves.get(i).unwrap().get(
                    &seed.derive(&i),
                    &(0..self.dim).map(|i| point.get(i).unwrap() / mul).collect(),
                );
                let noise = oct_noise * mul;
                noise
            })
            .sum();
        //return (noise / scale, Vec::from_fn(self.dim, |i| *der.get(i) / scale));
        let scale: f64 = (0..self.octaves.len())
            .map(|i| self.octave_factor.powi(i as i32))
            .sum();
        return noise / scale;
    }
}

pub struct TiledOctavedSimplexNoise {
    pub dim: usize,
    pub tile_distance: f64,
    pub scale: f64,
    pub source: OctavedSimplexNoise,
}

impl TiledOctavedSimplexNoise {
    pub fn new(
        dim: usize,
        num_octaves: usize,
        octave_factor: f64,
        tile_distance: f64,
        scale: f64,
    ) -> TiledOctavedSimplexNoise {
        TiledOctavedSimplexNoise {
            dim: dim,
            tile_distance: tile_distance,
            scale: scale,
            source: OctavedSimplexNoise::new(dim * 2, num_octaves, octave_factor),
        }
    }
}

impl super::Noise for TiledOctavedSimplexNoise {
    fn get_noise(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        let point_source = (0..self.dim * 2)
            .map(|i| {
                self.scale
                    * if i % 2 == 0 {
                        (*point.get(i / 2).unwrap() * 2.0 * PI / self.tile_distance).sin()
                    } else {
                        (*point.get(i / 2).unwrap() * 2.0 * PI / self.tile_distance).cos()
                    }
            })
            .collect();
        return self.source.get_noise(seed, &point_source);
    }
}

pub struct SkewedTiledOctavedSimplexNoise {
    dim: usize,
    tile_distance: f64,
    scale: f64,
    source: OctavedSimplexNoise,
}

impl SkewedTiledOctavedSimplexNoise {
    pub fn new(
        dim: usize,
        num_octaves: usize,
        octave_factor: f64,
        tile_distance: f64,
        scale: f64,
    ) -> SkewedTiledOctavedSimplexNoise {
        SkewedTiledOctavedSimplexNoise {
            dim: dim,
            tile_distance: tile_distance,
            scale: scale,
            source: OctavedSimplexNoise::new(dim * 2 + 1, num_octaves, octave_factor),
        }
    }
}

impl super::Noise for SkewedTiledOctavedSimplexNoise {
    fn get_noise(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        let mut point_source: Vec<f64> = (0..self.dim * 2)
            .map(|i| {
                self.scale
                    * if i % 2 == 0 {
                        (*point.get(i / 2).unwrap() * 2.0 * PI / self.tile_distance).sin()
                    } else {
                        (*point.get(i / 2).unwrap() * 2.0 * PI / self.tile_distance).cos()
                    }
            })
            .collect();
        point_source.push(seed.get_skew());
        return self.source.get_noise(seed, &point_source);
    }
}
