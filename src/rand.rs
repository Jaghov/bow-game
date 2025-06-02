use rand::{
    Rng, SeedableRng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
};
use rand_chacha::ChaCha8Rng;
use std::sync::{LazyLock, Mutex};

// Static RNG wrapped in Mutex for thread safety
static RNG: LazyLock<Mutex<ChaCha8Rng>> = LazyLock::new(|| {
    let seed = [32u8; 32];
    Mutex::new(ChaCha8Rng::from_seed(seed))
});

// Function to access RNG
pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    let mut rng = RNG.lock().unwrap();
    rng.random()
}

pub fn random_bool(p: f64) -> bool {
    let mut rng = RNG.lock().unwrap();
    rng.random_bool(p)
}

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = RNG.lock().unwrap();
    rng.random_range(range)
}
