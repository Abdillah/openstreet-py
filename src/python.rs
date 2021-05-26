/**/
use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::class::iter::{PyIterProtocol, IterNextOutput};

use osm_xml as osm;

use crate::map;
use crate::queries;
use crate::queries::QueryBuilder;

#[pyclass]
#[derive(Clone)]
/// OpenStreet Map object
struct Node {
    inner: map::Node
}

#[pymethods]
impl Node {
    #[getter]
    /// A numeric Id
    pub fn id(&self) -> PyResult<i64> {
        Ok(self.inner.id)
    }

    #[getter]
    /// Latitude
    pub fn lat(&self) -> PyResult<f64> {
        Ok(self.inner.lat)
    }

    #[getter]
    /// Longitude
    pub fn lon(&self) -> PyResult<f64> {
        Ok(self.inner.lon)
    }

    #[getter]
    /// A Dictionary of tag key and value
    pub fn tags(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.inner.tags.clone())
    }
}

impl From<map::Node> for Node {
    fn from(node: map::Node) -> Self {
        Node { inner: node }
    }
}

#[pyclass]
#[derive(Clone)]
/// OpenStreet Way object
struct Way {
    inner: map::Way
}

#[pymethods]
impl Way {
    #[getter]
    /// A numeric Id
    pub fn id(&self) -> PyResult<i64> {
        Ok(self.inner.id)
    }

    #[getter]
    /// A Dictionary of tag key and value
    pub fn tags(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.inner.tags.clone())
    }

    #[getter]
    /// A List of Node IDs
    pub fn nodes(&self) -> PyResult<Vec<i64>> {
        Ok(self.inner.nodes.clone())
    }

    /// Determine whether it is a closed polygon (area) or not
    pub fn is_area(&self) -> bool {
        self.inner.is_polygon()
    }
}

impl From<map::Way> for Way {
    fn from(way: map::Way) -> Self {
        Way { inner: way }
    }
}

#[pyclass]
#[derive(Clone)]
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
struct NodeQueryIter {
    inner: queries::BuilderIter<map::Node>,
}

#[pyproto]
impl PyIterProtocol for NodeQueryIter {
    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<Node, &str> {
        if let Some(result) = slf.inner.next() {
            IterNextOutput::Yield(result.1.into())
        } else {
            IterNextOutput::Return("Exhausted")
        }
    }
}


#[pyclass]
/// Object that save filtering operations
struct NodeQueryBuilder {
    inner: queries::Builder<map::Node>,
}


#[pymethods]
impl NodeQueryBuilder {
    /// Returns Node with given ``id``
    pub fn by_id(&self, id: osm::Id) -> Node {
        Node { inner: self.inner.by_id(id).clone() }
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
        NodeQueryBuilder { inner: self.inner.clone().by_tag_in(key, values) }
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
        NodeQueryBuilder { inner: self.inner.clone().by_tag_eq(key, value) }
    }

    #[text_signature = "(self)"]
    /// Returns the filtered Node list
    pub fn get(&self) -> Vec<Node> {
        self.inner.get().iter_mut()
        .map(|n| Node { inner: n.clone() })
        .collect::<Vec<Node>>()
    }
}

impl From<queries::Builder<map::Node>> for NodeQueryBuilder {
    fn from(builder: queries::Builder<map::Node>) -> Self {
        NodeQueryBuilder { inner: builder }
    }
}

#[pyproto]
impl PyIterProtocol for NodeQueryBuilder {
    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<NodeQueryIter>> {
        let iter = NodeQueryIter {
            inner: slf.inner.iter()
        };
        Py::new(slf.py(), iter)
    }
}


#[pyclass]
/// Object that save filtering operations
struct WayQueryBuilder {
    inner: queries::Builder<map::Way>,
}


#[pymethods]
impl WayQueryBuilder {
    #[text_signature = "(self, id)"]
    /// Returns Node with given ``id``
    pub fn by_id(&self, id: osm::Id) -> Way {
        Way { inner: self.inner.by_id(id).clone() }
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
        WayQueryBuilder { inner: self.inner.clone().by_tag_in(key, values) }
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
        WayQueryBuilder { inner: self.inner.clone().by_tag_eq(key, value) }
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
        WayQueryBuilder { inner: self.inner.clone().contain_nodes(nodes) }
    }

    #[text_signature = "(self)"]
    /// Returns the filtered Way list
    pub fn get(&self) -> Vec<Way> {
        self.inner.get().iter_mut()
        .map(|w| Way { inner: w.clone() })
        .collect::<Vec<Way>>()
    }
}

impl From<queries::Builder<map::Way>> for WayQueryBuilder {
    fn from(builder: queries::Builder<map::Way>) -> Self {
        WayQueryBuilder { inner: builder }
    }
}

#[pyproto]
impl PyIterProtocol for WayQueryBuilder {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        unimplemented!()
        // slf.inner.iter()
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
    inner: map::Map,
}

#[pymethods]
impl Map {
    #[new]
    pub fn new(path: String) -> Self {
        Self {
            inner: map::Map::new(path)
        }
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to WayQueryBuilder methods for available filters.
    /// Call :py:func:`WayQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn ways(&self) -> WayQueryBuilder {
        self.inner.ways().into()
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to NodeQueryBuilder methods for available filters.
    /// Call :py:func:`NodeQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn nodes(&self) -> NodeQueryBuilder {
        self.inner.nodes().into()
    }

    /// Return Bounds object of the map
    pub fn bounds(&self) -> Option<Bounds> {
        if let Some(bounds) = self.inner.bounds() {
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
