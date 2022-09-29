import numpy as np
from xarray.core import variable

from . import snap


def random(dim: int) -> snap.Snapshot:
    nparticles = 1000

    variables = dict(
        positions=np.random.rand(nparticles, dim),
        momenta=np.random.rand(nparticles, dim),
        radii=np.random.rand(nparticles),
    )

    return snap.Snapshot.from_dict(variables)
