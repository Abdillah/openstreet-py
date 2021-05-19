import os
import sys

# We need native module, so build the project first.
dirname = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.abspath(dirname + '/../build/lib'))

from openstreet import Map

def example_get(map):
    print(map.nodes().by_id(8623725793))

    ctr = 5
    for node in map.nodes().where_tag_in("highway", [ "primary" ]).get():
        if ctr == 0:
            break;

        print("Node id:", node.id)
        print("- ", node.tags)

        ctr = ctr - 1

def example_iterator(map):
    ctr = 5
    for node in map.nodes().where_tag_in("highway", [ "primary" ]):
        if ctr == 0:
            break;

        print("Node id:", node.id)

        ctr = ctr - 1

if __init__ == "__main__":
    map = Map("./resources/madina.osm")
    example_get(map)
    example_iterator(map)
