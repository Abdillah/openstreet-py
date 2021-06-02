/* network.rs */
use std::convert::TryInto;

use osm_xml as osm;
use rand;
use bidir_map::{BidirMap, ByFirst, BySecond};

use crate::queries::QueryBuilder;
use crate::map;
use crate::map::{Way, Node};
use crate::structure::NodeMap;

/// Graph for  OpenStreet's streets
pub struct StreetNetwork {
    pub inner: fast_paths::InputGraph,
    pub node_idx: NodeMap<i64>,
    // pub intersection_nodes: Vec<Node>,
    pub indexed_ways: std::collections::HashMap<osm::Id, Vec<Way>>,
}

impl StreetNetwork {
    pub fn new(map: &map::Map, street_types: Vec<&str>) -> Self {
        println!("Creating StreetNetwork!");
        println!("- Constraints: {:?}", street_types);

        let mut graph = fast_paths::InputGraph::new();
        let qstreets = map.ways()
        .by_tag_in("highway", vec![
            "primary"      , "secondary"      , "tertiary",
            "primary_link" , "secondary_link" , "tertiary_link",
            "residential"  , "service"
        ]);

        let mut node_idx: NodeMap<i64> = NodeMap::new();
        let mut node_ways_idx: std::collections::HashMap<osm::Id, Vec<Way>> = std::collections::HashMap::new();

        let mut c = 0;
        for (i64, way) in qstreets.iter() {
            let size = way.nodes.len();
            for i in 0..(size-1) {
                let mut a: usize = node_idx.get_or_insert(way.nodes[i]);
                let mut b: usize = node_idx.get_or_insert(way.nodes[i+1]);
                let w = rand::random::<u8>();

                graph.add_edge_bidir(a, b, w as usize);
                println!("Add edge {}/{} <-({})-> {}/{}", a, way.nodes[i], w, b, way.nodes[i+1])
            }

            for node_id in &way.nodes {
                if !node_ways_idx.contains_key(&node_id) {
                    let ways = Vec::new();
                    node_ways_idx.insert(*node_id, ways);
                }
                if let Some(ways) = node_ways_idx.get_mut(&node_id) {
                    ways.push(way.clone().into())
                }
            }
            // println!("Way {}", way.id);
        }
        node_idx.guarantee_node_ordering(&mut graph);
        graph.freeze();
        println!("There are {} edges added, {} num nodes", c, graph.get_num_nodes());

        println!("Return StreetNetwork");
        Self {
            inner: graph,
            node_idx,
            indexed_ways: node_ways_idx,
        }
    }

    pub fn intersections() -> Vec<Node> {
        unimplemented!()

    }

    pub fn shortest_path(&mut self, a: i64, b: i64) -> Vec<i64> {
        // prepare the graph for fast shortest path calculations. note that you have to do this again if you want to change the
        // graph topology or any of the edge weights
        let fast_graph = fast_paths::prepare(&self.inner);

        // calculate the shortest path between nodes with ID 8 and 6
        match fast_paths::calc_path(&fast_graph, a.try_into().unwrap(), b.try_into().unwrap()) {
            Some(p) => self.node_idx.translate(&p),
            None => vec![],
        }
    }

    pub fn serialize(&self, path: &std::path::Path) {
        let encoded: Vec<u8> = bincode::serialize(&self.inner).unwrap();
        std::fs::write(path, encoded).unwrap();
    }
}

#[test]
fn test_fastpath() {
    println!("Creating Map!");
    let map = crate::map::Map::new("resources/madina.osm".into());

    println!("Into StreetNetwork!");
    let mut gra = StreetNetwork::new(&map, vec![
        "primary"      , "secondary"      , "tertiary",
        "primary_link" , "secondary_link" , "tertiary_link",
        "residential"  , "service"
    ]);
    let sp = gra.shortest_path(1, 5);
    println!("Shortest path: {:?}", sp);

    let sp = gra.shortest_path(1, 12);
    println!("Shortest path: {:?}", sp);
}
