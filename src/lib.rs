extern crate osm_xml as osm;
extern crate fnv;

use std::fs::File;

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

mod query;

#[pyclass]
struct Node {
    inner: osm::Node
}

#[pyclass]
struct Way {
    inner: osm::Way
}

#[pyclass]
struct WayQueryBuilder {
    _qb: query::Builder<osm::Way>,
}

impl WayQueryBuilder {
    fn new(ways: &fnv::FnvHashMap<i64, osm::Way>) -> WayQueryBuilder {
        WayQueryBuilder { _qb: query::Builder::new(ways.clone()) }
    }
}


#[pymethods]
impl WayQueryBuilder {
    pub fn by_id(&self, id: osm::Id) -> Way {
        Way { inner: self._qb.by_id(id).clone() }
    }

    pub fn by_tag_in(&self, key: &str, values: Vec<&str>) -> WayQueryBuilder {
        WayQueryBuilder { _qb: self._qb.clone().by_tag_in(key, values) }
    }

    pub fn get(&self) -> Vec<Way> {
        self._qb.get().iter_mut()
        .map(|w| Way { inner: w.clone() })
        .collect::<Vec<Way>>()
    }
}

#[pyclass]
struct NodeQueryBuilder {
    _qb: query::Builder<osm::Node>,
}

impl NodeQueryBuilder {
    fn new(nodes: &fnv::FnvHashMap<i64, osm::Node>) -> NodeQueryBuilder {
        NodeQueryBuilder { _qb: query::Builder::new(nodes.clone()) }
    }
}

use query::BuilderGet;

#[pymethods]
impl NodeQueryBuilder {
    pub fn by_id(&self, id: osm::Id) -> Node {
        Node { inner: self._qb.by_id(id).clone() }
    }

    pub fn by_tag_in(&self, key: &str, values: Vec<&str>) -> NodeQueryBuilder {
        NodeQueryBuilder { _qb: self._qb.clone().by_tag_in(key, values) }
    }

    pub fn get(&self) -> Vec<Node> {
        self._qb.get().iter_mut()
        .map(|n| Node { inner: n.clone() })
        .collect::<Vec<Node>>()
    }
}


#[pyclass]
struct Map {
    inner: osm::OSM,
}

#[pymethods]
impl Map {
    #[new]
    pub fn new(path: String) -> Map {
        let f = File::open(path).unwrap();
        let doc = osm::OSM::parse(f).unwrap();
        Map { inner: doc }
    }

    pub fn ways(&self) -> WayQueryBuilder {
        WayQueryBuilder::new(&self.inner.ways)
    }

    pub fn nodes(&self) -> NodeQueryBuilder {
        NodeQueryBuilder::new(&self.inner.nodes)
    }
}

#[pymodule]
fn openstreet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "OpenStreet map with advanced graph functionality built in.")?;
    m.add_class::<Map>()?;
    m.add_class::<Node>()?;
    m.add_class::<Way>()?;
    Ok(())
}

#[test]
fn test_osm() {
    let f = File::open("resources/madina.osm").unwrap();
    let doc = osm::OSM::parse(f).unwrap();

    let mut streets: Vec<&osm::Way> = vec!();
    for way in doc.ways.values() {
        if let Some(highway_type) = way.tags.iter().find(|t| t.key == "highway") {
            if highway_type.val == "primary" || highway_type.val == "secondary" || highway_type.val == "tertiary"
            || highway_type.val == "primary_link" || highway_type.val == "secondary_link" || highway_type.val == "tertiary_link"
            || highway_type.val == "residential" || highway_type.val == "service" {
                streets.push(way)
            }
        }
    }


    // find nodes that occur in more than one street
    use std::collections::HashMap;

    let mut intersections: HashMap<i64, Vec<&osm::Way>> = HashMap::new();
    for way in &streets {
        for node in &way.nodes {
            if let osm::UnresolvedReference::Node(node_id) = node {
                if !intersections.contains_key(&node_id) {
                    intersections.insert(*node_id, vec!());
                }
                let ways = intersections.get_mut(&node_id).unwrap();
                ways.push(way);
            }
        }
    }

    println!("{:?}", intersections);

    // return intersections;


    // println!("Node count {}", doc.nodes.len());
    // println!("Way count {}", doc.ways.len());
    // println!("Polygon count {}", poly_count);
    // println!("Relation count {}", doc.relations.len());
    // println!("Tag count {}", tag_count(&doc));

    // println!("Way reference count: {}, invalid references: {}",  way_info.0, way_info.1);
    // println!("Relation reference count: {}, resolved: {}, unresolved: {}", rel_info.0, rel_info.1, rel_info.2);
}
