use super::Universe;

use num::Float;

impl<F: Float, const N: usize> Universe<F, N> {
    pub fn trivial(&mut self) {
        self.time += 1;
    }
}
