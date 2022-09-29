from sphluid import init
from sphluid.sphluid import Universe

snap = init.random(3)
#  snap.create_universe("ciao.nc")
__import__("pdb").set_trace()

uni = Universe.from_time("ciao.nc", 0)
