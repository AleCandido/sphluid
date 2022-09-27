use ndarray::{arr1, Array1};
use num::Num;

/// Represents a particle
/// It is generic over the physical space dimension `N` and the "field" `F` (usually `f32` is a
/// good choice)
#[derive(Clone)]
pub struct Particle<F: Num + Copy> {
    /// N-dimensional position
    pub(crate) x: Array1<F>,
    /// N-dimensional space-phase
    pub(crate) p: Array1<F>,
    /// Radius of the particle
    pub(crate) r: F,
}

impl<F: Num + Copy> Particle<F> {
    ///
    pub fn new(x: &[F], p: &[F], r: F) -> Self {
        Self {
            x: arr1(x),
            p: arr1(p),
            r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let x1 = [1., 2., 3.];
        let p1 = [4., 5., 6.];
        let r1 = 10.;
        let i1 = Particle::new(&x1, &p1, r1);

        assert_eq!(i1.x.as_slice().unwrap(), &x1);
        assert_eq!(i1.p.as_slice().unwrap(), &p1);
        assert_eq!(i1.r, r1);
    }
}
