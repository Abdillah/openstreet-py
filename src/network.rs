/* network.rs */
use serde::{Serialize, Deserialize};

use crate::queries::QueryBuilder;
use crate::map;
use crate::map::{Way, Node};
use crate::structure::NodeMap;

/// Graph for  OpenStreet's streets
#[derive(Serialize, Deserialize)]
pub struct StreetNetwork {
    pub inner: fast_paths::InputGraph,
    pub node_idx: NodeMap<i64>,
    // pub intersection_nodes: Vec<Node>,
    pub nodeways_idx: std::collections::HashMap<i64, Vec<Way>>,
}

impl StreetNetwork {
    pub fn new(map: &map::Map, street_types: Vec<&str>) -> Self {
        // println!("Creating StreetNetwork!");
        // println!("- Constraints: {:?}", street_types);

        let mut graph = fast_paths::InputGraph::new();
        let mut qstreets = map.ways();
        if street_types.len() > 0 {
            qstreets = qstreets.by_tag_in("highway", street_types);
        }

        let mut node_idx: NodeMap<i64> = NodeMap::new();
        let mut node_ways_idx: std::collections::HashMap<i64, Vec<Way>> = std::collections::HashMap::new();

        let mut qnodes = map.nodes();

        let mut c = 0;
        for (_, way) in qstreets.iter() {
            let size = way.nodes.len();
            for i in 0..(size-1) {
                c += 2;
                let a = node_idx.get_or_insert(way.nodes[i]);
                let b = node_idx.get_or_insert(way.nodes[i+1]);

                let node_a = qnodes.by_id(way.nodes[i]);
                let node_b = qnodes.by_id(way.nodes[i+1]);
                let w = ((node_a.lat - node_b.lat).powi(2) + (node_a.lon - node_b.lon as f64).powi(2)).sqrt() * 111_120.0;

                graph.add_edge_bidir(a, b, w as usize);
                // println!("Add edge {}/{} <-({})-> {}/{}", a, way.nodes[i], w, b, way.nodes[i+1])
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
        // println!("There are {} edges added, {} num nodes", c, graph.get_num_nodes());

        // println!("Return StreetNetwork");
        Self {
            inner: graph,
            node_idx,
            nodeways_idx: node_ways_idx,
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
        let a = self.node_idx.get_or_insert(a);
        let b = self.node_idx.get_or_insert(b);
        match fast_paths::calc_path(&fast_graph, a, b) {
            Some(p) => self.node_idx.translate(&p),
            None => vec![],
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().as_bytes().to_vec()
    }

    pub fn deserialize(state: Vec<u8>) -> Self {
        serde_json::from_str(std::str::from_utf8(&state).unwrap()).unwrap()
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
