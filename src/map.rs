use osm_xml as osm;
use std::collections::HashMap;
use fnv::FnvHashMap;

use crate::queries;
use crate::python;

use queries::Builder as QueryBuilder;

#[derive(Clone)]
/// OpenStreet Map object
pub(crate) struct Node {
    inner: osm::Node,
    pub id: osm::Id,
    pub lat: osm::Coordinate,
    pub lon: osm::Coordinate,
    pub tags: HashMap<String, String>,
}

impl From<&osm::Node> for Node {
    fn from(node: &osm::Node) -> Self {
        let node_o = node.clone();
        let mut tagdict = HashMap::new();
        Node {
            inner: node.clone(),
            id: node_o.id,
            lat: node_o.lat,
            lon: node_o.lon,
            tags: node_o.tags.iter()
            .fold(tagdict, |mut d, t| {
                d.insert(t.key.clone(), t.val.clone());
                d
            }),
        }
    }
}

#[derive(Clone)]
/// OpenStreet Way object
pub(crate) struct Way {
    inner: osm::Way,
    pub id: osm::Id,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>,
}

impl Way {
    pub fn nodes(&self) -> Vec<i64> {
        self.inner.nodes.iter()
        .map(|n| if let osm::UnresolvedReference::Node(id) = n { Some(id) } else { None })
        .filter(|n| n.is_some())
        .map(|n| *n.unwrap())
        .collect::<Vec<i64>>()
    }

    pub fn is_polygon(&self) -> bool {
        self.inner.is_polygon()
    }
}

impl From<&osm::Way> for Way {
    fn from(way: &osm::Way) -> Self {
        let way_o = way.clone();
        let mut tagdict = HashMap::new();

        Way {
            inner: way.clone(),
            id: way_o.id,
            tags: way_o.tags.iter()
            .fold(tagdict, |mut d, t| {
                d.insert(t.key.clone(), t.val.clone());
                d
            }),
            nodes: way_o.nodes.iter()
            .map(|n| if let osm::UnresolvedReference::Node(id) = n { Some(id) } else { None })
            .filter(|n| n.is_some())
            .map(|n| *n.unwrap())
            .collect(),
        }
    }
}

/// OpenStreet Bounds object
pub(crate) struct Bounds {
    /// Min latitude
    pub minlat: f64,
    /// Min longitude
    pub minlon: f64,
    /// Max latitude
    pub maxlat: f64,
    /// Max longitude
    pub maxlon: f64,
}

pub(crate) trait TaggableElement {
    fn get_id(&self) -> i64;

    fn get_tag_value(&self, key: &str) -> Option<&str>;
}

impl TaggableElement for Node {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_tag_value(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|v| v.as_str())
    }
}

impl TaggableElement for Way {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_tag_value(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|v| v.as_str())
    }
}


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
pub(crate) struct Map {
    inner: osm::OSM,
}

impl Map {
    pub fn new(path: String) -> Map {
        let f = std::fs::File::open(path).unwrap();
        let doc = osm::OSM::parse(f).unwrap();

        Map {
            inner: doc,
        }
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to WayQueryBuilder methods for available filters.
    /// Call :py:func:`WayQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn ways(&self) -> QueryBuilder<Way> {
        // TODO: Remove runtime overhead by clone the xml parser
        let mut ways: FnvHashMap<i64, Way> = FnvHashMap::default();
        for (id, way) in &self.inner.ways {
            ways.insert(*id, way.into());
        }
        QueryBuilder::<Way>::new(ways)
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to NodeQueryBuilder methods for available filters.
    /// Call :py:func:`NodeQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn nodes(&self) -> QueryBuilder<Node> {
        let mut nodes: FnvHashMap<i64, Node> = FnvHashMap::default();
        for (id, node) in &self.inner.nodes {
            nodes.insert(*id, node.into());
        }

        QueryBuilder::<Node>::new(nodes)
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
