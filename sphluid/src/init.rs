use super::particle::Particle;
use num::{cast, one, zero, Float};

pub fn random<F: Float, const N: usize>(n: usize) -> Vec<Particle<F, N>> {
    return vec![Particle::new(&[cast(0.).unwrap(); N], &[zero(); N], one()); n];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let n = 1e5 as usize;
        let particles = random::<f32, 3>(n);

        assert_eq!(particles.len(), n);
    }
}
