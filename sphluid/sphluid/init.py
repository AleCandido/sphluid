import numpy as np

from . import snap


def random(dim: int) -> snap.Snapshot:
    nparticles = 1000

    positions = np.random.rand(nparticles, dim)
    momenta = np.random.rand(nparticles, dim)
    radii = np.random.rand(nparticles)

    return snap.Snapshot(positions, momenta, radii)
