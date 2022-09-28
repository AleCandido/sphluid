import dataclasses
from typing import Optional

import numpy.typing as npt
import xarray as xr


@dataclasses.dataclass
class Snapshot:
    positions: Optional[list[npt.NDArray]] = None
    momenta: Optional[list[npt.NDArray]] = None
    radii: Optional[list[npt.NDArray]] = None

    descriptions = dict(
        pos="Particles positions", mom="Particles momenta", rad="Particles radii"
    )

    def to_xarray(self) -> xr.Dataset:
        part = "particles"
        ax = "axis"

        return xr.Dataset(
            dict(
                positions=(
                    [part, ax],
                    self.positions,
                    dict(description=self.descriptions["pos"]),
                ),
                momenta=(
                    [part, ax],
                    self.momenta,
                    dict(description=self.descriptions["mom"]),
                ),
                radii=(
                    [part],
                    self.radii,
                    dict(description=self.descriptions["rad"]),
                ),
            )
        )
