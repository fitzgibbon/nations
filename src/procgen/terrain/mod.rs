use crate::procgen;
use crate::procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise;
use crate::procgen::noise::Noise;
use crate::procgen::procseed::ProcSeed;
use std::f64::consts::PI;

//use cursive::theme::Color;

use rand;

pub trait Terrain {
    fn get_height(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64;
    fn get_precipitation(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64;
    fn get_temperature(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64;
    fn get_map_texture(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64;
    fn get_biome(&self, seed: &ProcSeed, point: &Vec<f64>) -> MapBiome;
    //fn render_cursive(&self, seed: &ProcSeed, point: &Vec<f64>) -> (char, Color, Color);
}

pub struct TiledWorldTerrain {
    heightmap: SkewedTiledOctavedSimplexNoise,
    moisturemap: SkewedTiledOctavedSimplexNoise,
    temperatureperturbancemap: SkewedTiledOctavedSimplexNoise,
    maptexturemap: SkewedTiledOctavedSimplexNoise,
}

impl TiledWorldTerrain {
    pub fn new() -> TiledWorldTerrain {
        let tile_distance = 1.0;
        TiledWorldTerrain {
            // TODO: for some reason all these noise maps seem to glitch when zooming in, tending towards straight hard edges at 45 degree angles. Must be a float or rounding error. Investigate in noise functions.
            heightmap: procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise::new(
                2,
                20,
                0.5,
                tile_distance,
                0.5,
            ),
            moisturemap: procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise::new(
                2,
                10,
                0.5,
                tile_distance,
                0.5,
            ),
            temperatureperturbancemap:
                procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise::new(
                    2,
                    5,
                    0.5,
                    tile_distance,
                    1.0,
                ),
            maptexturemap: procgen::noise::simplex_noise::SkewedTiledOctavedSimplexNoise::new(
                2,
                5,
                0.5,
                tile_distance,
                0.5,
            ),
        }
    }
}

pub enum MapBiome {
    Empty,
    Water,
    Ice,
    Tundra,
    BorealForest,
    Shrubland,
    TemperateGrassland,
    TemperateRainforest,
    TemperateSeasonalForest,
    TropicalRainforest,
    TropicalSeasonalForest,
    Savannah,
    Desert,
    Mountain,
}

impl Terrain for TiledWorldTerrain {
    fn get_height(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        self.heightmap.get_noise(&seed.derive(&"heightmap"), point) / 2.0 + 0.5
    }

    fn get_precipitation(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        self.moisturemap
            .get_noise(&seed.derive(&"moisturemap"), point)
            / 2.0
            + 0.5
    }

    fn get_temperature(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        let base = ((point.get(1).unwrap() + 0.25) * 2.0 * PI).sin();
        let base_weight = 1.5;
        let perturbance = self
            .temperatureperturbancemap
            .get_noise(&seed.derive(&"temperatureperturbancemap"), point);
        let perturbance_weight = 1.0;
        ((base * base_weight + perturbance * perturbance_weight)
            / (base_weight + perturbance_weight))
            / 2.0
            + 0.5
    }

    fn get_map_texture(&self, seed: &ProcSeed, point: &Vec<f64>) -> f64 {
        self.maptexturemap
            .get_noise(&seed.derive(&"maptexturemap"), point)
            / 2.0
            + 0.5
    }

    fn get_biome(&self, seed: &ProcSeed, point: &Vec<f64>) -> MapBiome {
        // fudged implementation of https://upload.wikimedia.org/wikipedia/commons/6/68/Climate_influence_on_terrestrial_biome.svg
        let water_level = 0.55;
        let mountain_level = 0.7;

        let freezing = 0.225;
        let cold = 0.3;
        let temperate = 0.6;

        let arid = 0.45;
        let moist = 0.5;
        let wet = 0.6;

        let height = self.get_height(seed, point);
        let precipitation = self.get_precipitation(seed, point);
        let temperature = self.get_temperature(seed, point);

        if height < water_level {
            if temperature < freezing {
                MapBiome::Ice
            } else {
                MapBiome::Water
            }
        } else if height < mountain_level {
            if temperature < freezing {
                MapBiome::Tundra
            } else if temperature < cold {
                if precipitation < arid {
                    MapBiome::TemperateGrassland
                } else if precipitation < moist {
                    MapBiome::Shrubland
                } else {
                    MapBiome::BorealForest
                }
            } else if temperature < temperate {
                if precipitation < arid {
                    MapBiome::TemperateGrassland
                } else if precipitation < moist {
                    MapBiome::Shrubland
                } else if precipitation < wet {
                    MapBiome::TemperateSeasonalForest
                } else {
                    MapBiome::TemperateRainforest
                }
            } else {
                if precipitation < arid {
                    MapBiome::Desert
                } else if precipitation < moist {
                    MapBiome::Savannah
                } else if precipitation < wet {
                    MapBiome::TropicalSeasonalForest
                } else {
                    MapBiome::TropicalRainforest
                }
            }
        } else {
            MapBiome::Mountain
        }
    }
}

fn color_brightness(color: &Vec<f64>, brightness: f64) -> Vec<f64> {
    color_lerp(&vec![0.0, 0.0, 0.0], color, brightness)
}

fn color_lerp(color_a: &Vec<f64>, color_b: &Vec<f64>, lerp: f64) -> Vec<f64> {
    (0..3)
        .map(|i| color_a[i] + (color_b[i] - color_a[i]) * lerp)
        .collect()
}
