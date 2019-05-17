pub mod simplex_noise;
use crate::procgen::procseed::ProcSeed;

pub trait Noise {
    fn get_noise(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64;
}

pub trait NoiseDerivative {
    fn get_noise_derivative(&self, seed: &ProcSeed, point: &Vec<f64>) -> Vec<f64>;
}
