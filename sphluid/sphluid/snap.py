import dataclasses
import enum
import os
from typing import Optional

import numpy.typing as npt
import xarray as xr

from . import sphluid


class Descrs(enum.Enum):
    pos = "Positions"
    mom = "Momenta"
    rad = "Radii"


class Dims(enum.Enum):
    time = "time"
    part = "particle"
    axis = "axis"


def values(members: list) -> list[str]:
    return [x.value for x in members]


@dataclasses.dataclass
class Snapshot:
    positions: npt.NDArray
    momenta: npt.NDArray
    radii: npt.NDArray
    time: Optional[int] = None
    "For annotation purpose."

    @classmethod
    def from_dict(cls, dictionary: dict[str, npt.NDArray]):
        """Import content from dictionary.

        Current implementation is trivial, but allows for further checks in
        future.

        """
        return cls(**dictionary)

    def to_xarray(self) -> xr.Dataset:
        "Convert to a :cls:`xr.Dataset`."
        return xr.Dataset(
            dict(
                positions=(
                    values([Dims.part, Dims.axis]),
                    self.positions,
                    dict(description=Descrs.pos.value),
                ),
                momenta=(
                    values([Dims.part, Dims.axis]),
                    self.momenta,
                    dict(description=Descrs.mom.value),
                ),
                radii=(
                    values([Dims.part]),
                    self.radii,
                    dict(description=Descrs.rad.value),
                ),
            )
        )

    def begin(self, path: os.PathLike):
        "Dump a new universe, with this as initial snapshot."
        self.to_xarray().expand_dims(Dims.time.value, 0).to_netcdf(path)

    def to_uni(self) -> sphluid.Universe:
        "Originate universe from snapshot."
        return sphluid.Universe(self.positions, self.momenta, self.radii)

    def from_uni(self, uni: sphluid.Universe):
        "Originate universe from snapshot."
        return Snapshot(uni.positions, uni.momenta, uni.radii)
