import os
import sys
import pickle

# We need native module, so build the project first.
dirname = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.abspath(dirname + '/../build/lib'))

from openstreet import Map
from openstreet import StreetNetwork

def example_get(map):
    print(map.nodes().by_id(8623725793))

    ctr = 5
    for way in map.ways().where_tag_in("highway", [ "primary" ]).get():
        if ctr == 0:
            break;

        print("Ways id:", way.id)
        print("- ", way.tags)

        ctr = ctr - 1

def example_iterator(map):
    ctr = 5
    for way in map.ways().where_tag_in("highway", [ "primary" ]):
        if ctr == 0:
            break;

        print("Ways id:", way.id)

        ctr = ctr - 1

def example_graph(map):
    snet = StreetNetwork(map, [ "primary" ]);

    print("Shortest path between ", 4137262376, " and ", 4137262384)
    path = snet.shortest_path(4137262376, 4137262384);
    print(".. are ", path)

    print("Pickling..")
    f = open('./streetnetwork.pickle', 'wb')
    pickle.dump(snet, f)

    print("Recovering..")
    f = open('./streetnetwork.pickle', 'rb')
    snet2 = pickle.load(f)

    print("Shortest path between ", 4137262376, " and ", 4137262384)
    path = snet2.shortest_path(4137262376, 4137262384)
    print(".. are ", path)

if __name__ == "__main__":
    map = Map("./resources/madina.osm")
    # example_get(map)
    # example_iterator(map)
    example_graph(map)
