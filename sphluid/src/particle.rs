/// Represents a particle mooving
struct Particle<N: Num> {
    /// N-dimensional position
    x: [f32; N],
    /// N-dimensional space-phase
    p: [f32; N],
    /// Radius of the particle
    r: f32,
}
