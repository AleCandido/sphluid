import sys
import argparse

import numpy as np
import matplotlib.pyplot as plt
import xarray as xr

parser = argparse.ArgumentParser()
parser.add_argument("history", help="netcdf history data file")
args = parser.parse_args()

ds = xr.load_dataset(args.history)

plt.figure().add_subplot(projection='3d')
for v in ds.x[:,:,::50].T:
    plt.plot(*v)
plt.show()
