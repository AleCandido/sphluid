import pathlib

from sphluid import init, Universe

unipth = pathlib.Path("universe.nc").absolute()

snap = init.random(3)
snap.begin(unipth)

uni = Universe.from_time(unipth, 0)
__import__("pdb").set_trace()
