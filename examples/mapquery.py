import os
import sys

# We need native module, so build the project first.
dirname = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.abspath(dirname + '/../build/lib'))

from openstreet import Map

map = Map("./resources/madina.osm")
print(map.nodes().by_id(8623725793))
print(map.nodes().by_tag_in("highway", [ "primary" ]).get())
