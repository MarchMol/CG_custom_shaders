use fastnoise_lite::{CellularDistanceFunction, DomainWarpType, FastNoiseLite, FractalType, NoiseType};


pub fn get_sun_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_frequency(Some(0.022));
    noise.set_cellular_distance_function(Some(CellularDistanceFunction::EuclideanSq));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise
}

pub fn get_mercury_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_frequency(Some(0.01));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_weighted_strength(Some(1.6));
    noise
}

pub fn get_saturn_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::ValueCubic));
    noise.set_frequency(Some(0.01));
    noise
}
pub fn get_jupiter_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::ValueCubic));
    noise.set_frequency(Some(0.01));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise
}
pub fn get_neptune_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::ValueCubic));
    noise.set_frequency(Some(0.01));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise
}


pub fn get_venus_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_frequency(Some(0.01));
    noise.set_domain_warp_type(Some(DomainWarpType::OpenSimplex2));
    noise.set_domain_warp_amp(Some(150.0));
    noise
}
  pub fn get_earth_noise() -> FastNoiseLite{
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_frequency(Some(0.01));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_weighted_strength(Some(3.0));
    noise.set_domain_warp_type(Some(DomainWarpType::OpenSimplex2));
    noise.set_domain_warp_amp(Some(400.0));
    noise
  }