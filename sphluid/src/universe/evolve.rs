use super::Universe;

use num::Float;
use rayon::prelude::*;

impl<F: Float + Send, const N: usize> Universe<F, N> {
    pub fn trivial(&mut self) {
        self.time += 1;
    }

    pub fn vacuum(&mut self) {
        self.particles
            .par_iter_mut()
            .for_each(|p| p.x = &p.x + &p.p);
        self.time += 1;
    }
}
