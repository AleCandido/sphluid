use super::particle::Particle;

fn random() -> Vec<Particle> {
    return vec![
        Paricle {
            x: [0., 3],
            p: [0., 3],
            R: [1.]
        };
        1e5 as usize
    ];
}
