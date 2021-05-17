extern crate osm_xml as osm;
extern crate fnv;

use std::fs::File;

use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

mod query;

use query::QueryBuilder;

#[pyclass]
/// OpenStreet Map object
struct Node {
    inner: osm::Node
}

#[pyclass]
/// OpenStreet Way object
struct Way {
    inner: osm::Way
}

#[pyclass]
/// OpenStreet Bounds object
struct Bounds {
    #[pyo3(get)]
    /// Min latitude
    pub minlat: f64,
    #[pyo3(get)]
    /// Min longitude
    pub minlon: f64,
    #[pyo3(get)]
    /// Max latitude
    pub maxlat: f64,
    #[pyo3(get)]
    /// Max longitude
    pub maxlon: f64,
}

#[pyclass]
/// Object that save filtering operations
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
    #[text_signature = "(self, id)"]
    /// Returns Node with given ``id``
    pub fn by_id(&self, id: osm::Id) -> Way {
        Way { inner: self._qb.by_id(id).clone() }
    }

    #[text_signature = "(self, key, values)"]
    /// Filter Way with tag of key ``key`` that contains one of ``values``
    ///
    /// See :py:class:`Map` documentation for usage example.
    ///
    /// Parameters
    /// ----------
    /// key : str
    ///     Key name of the tags.
    /// values : List[str]
    ///     Possible tag values to include.
    pub fn where_tag_in(&self, key: &str, values: Vec<&str>) -> WayQueryBuilder {
        WayQueryBuilder { _qb: self._qb.clone().by_tag_in(key, values) }
    }

    #[text_signature = "(self, key, value)"]
    /// Filter Way with tag of key ``key`` equal ``value``
    ///
    /// See :py:class:`Map` documentation for usage example.
    ///
    /// Parameters
    /// ----------
    /// key : str
    ///     Key name of the tags.
    /// value : str
    ///     A tag value to filter.
    pub fn where_tag_eq(&self, key: &str, value: &str) -> WayQueryBuilder {
        WayQueryBuilder { _qb: self._qb.clone().by_tag_eq(key, value) }
    }

    #[text_signature = "(self, key, value)"]
    /// Filter Way that contains nodes ``nodes``
    ///
    /// See :py:class:`Map` documentation for usage example.
    ///
    /// Parameters
    /// ----------
    /// nodes : List[int]
    ///     A list of node ids.
    pub fn where_contain_nodes(&self, nodes: Vec<osm::Id>) -> WayQueryBuilder {
        WayQueryBuilder { _qb: self._qb.clone().contain_nodes(nodes) }
    }

    #[text_signature = "(self)"]
    /// Returns the filtered Way list
    pub fn get(&self) -> Vec<Way> {
        self._qb.get().iter_mut()
        .map(|w| Way { inner: w.clone() })
        .collect::<Vec<Way>>()
    }
}

#[pyclass]
/// Object that save filtering operations
struct NodeQueryBuilder {
    _qb: query::Builder<osm::Node>,
}

impl NodeQueryBuilder {
    fn new(nodes: &fnv::FnvHashMap<i64, osm::Node>) -> NodeQueryBuilder {
        NodeQueryBuilder { _qb: query::Builder::new(nodes.clone()) }
    }
}

#[pymethods]
impl NodeQueryBuilder {
    /// Returns Node with given ``id``
    pub fn by_id(&self, id: osm::Id) -> Node {
        Node { inner: self._qb.by_id(id).clone() }
    }

    #[text_signature = "(self, key, values)"]
    /// Filter Node with tag of key ``key`` that contains one of ``values``
    ///
    /// Parameters
    /// ----------
    /// key : str
    ///     Key name of the tags.
    /// values : List[str]
    ///     Possible tag values to include.
    ///
    /// Returns
    /// -------
    /// self : :py:class:`.NodeQueryBuilder`
    pub fn where_tag_in(&self, key: &str, values: Vec<&str>) -> NodeQueryBuilder {
        NodeQueryBuilder { _qb: self._qb.clone().by_tag_in(key, values) }
    }

    #[text_signature = "(self, key, value)"]
    /// Filter Node with tag of key ``key`` equal ``value``
    ///
    /// See :py:class:`Map` documentation for usage example.
    ///
    /// Parameters
    /// ----------
    /// key : str
    ///     Key name of the tags.
    /// value : str
    ///     A tag value to filter.
    ///
    /// Returns
    /// -------
    /// self : :py:class:`.NodeQueryBuilder`
    pub fn where_tag_eq(&self, key: &str, value: &str) -> NodeQueryBuilder {
        NodeQueryBuilder { _qb: self._qb.clone().by_tag_eq(key, value) }
    }

    #[text_signature = "(self)"]
    /// Returns the filtered Node list
    pub fn get(&self) -> Vec<Node> {
        self._qb.get().iter_mut()
        .map(|n| Node { inner: n.clone() })
        .collect::<Vec<Node>>()
    }
}


#[pyclass]
/// Map provide parsing and storage for OSM format
///
/// Map contains three main information: nodes, ways, and bounds.
/// For ways and nodes, both must be accessed using query style
/// or fluent interface.
///
/// .. code-block:: python
///    :linenos:
///
///    map = Map("/path/to/map.osm")
///    streets = map.ways().where_tag_in("highstreet", [ "primary", "secondary" ]).get()
///
/// Tag is an element in OSM format looked like these:
/// ``<tag key="akeyhere" value="somevalue" />``. So using the ``by_tag_in`` filter
/// would means looping over all the ways in the OSM with the matching tag "highstreet"
/// and value of "primary" or "secondary".
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

    /// Return query builder to filter ways collection
    ///
    /// Refer to WayQueryBuilder methods for available filters.
    /// Call :py:func:`WayQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn ways(&self) -> WayQueryBuilder {
        WayQueryBuilder::new(&self.inner.ways)
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to NodeQueryBuilder methods for available filters.
    /// Call :py:func:`NodeQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn nodes(&self) -> NodeQueryBuilder {
        NodeQueryBuilder::new(&self.inner.nodes)
    }

    /// Return bounds of map
    pub fn bounds(&self) -> Option<Bounds> {
        if let Some(bounds) = self.inner.bounds {
            Some(Bounds {
                minlat: bounds.minlat,
                minlon: bounds.minlon,
                maxlat: bounds.maxlat,
                maxlon: bounds.maxlon,
            })
        } else {
            None
        }
    }
}

#[pymodule]
fn _binding(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__name__", "openstreet")?;
    m.add("__package__", "openstreet")?;
    m.add("__doc__", "OpenStreet map with advanced graph functionality built in.")?;
    m.add_class::<Map>()?;
    m.add_class::<Node>()?;
    m.add_class::<Way>()?;
    m.add_class::<Bounds>()?;
    m.add_class::<NodeQueryBuilder>()?;
    m.add_class::<WayQueryBuilder>()?;
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
