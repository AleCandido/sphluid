use super::Universe;

use rayon::prelude::*;

impl Universe {
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
