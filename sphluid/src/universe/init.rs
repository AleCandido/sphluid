use super::Universe;
use crate::particle::Particle;

use num::Float;
use rand::Rng;

use std::convert::TryInto;

pub fn random<F: Float, const N: usize>(n: usize) -> Universe<F, N>
where
    rand::distributions::Standard: rand::distributions::Distribution<F>,
    F: std::fmt::Debug,
{
    let mut rng = rand::thread_rng();
    let particles = vec![
        Particle::new(
            &(0..N)
                .map(|_| rng.gen())
                .collect::<Vec<F>>()
                .try_into()
                .unwrap(),
            &(0..N)
                .map(|_| rng.gen())
                .collect::<Vec<F>>()
                .try_into()
                .unwrap(),
            rng.gen()
        );
        n
    ];
    return Universe { particles, time: 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let n = 1e5 as usize;
        let universe = random::<f32, 3>(n);

        assert_eq!(universe.particles.len(), n);
    }
}
