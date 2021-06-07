/* Map */

use std::fmt::Debug;
use std::collections::HashMap;

use osm_xml as osm;
use fnv::FnvHashMap;
use serde::{Serialize, Deserialize};

use crate::queries::Builder as QueryBuilder;

#[derive(Clone, Debug, Serialize, Deserialize)]
/// OpenStreet Map object
pub struct Node {
    // inner: osm::Node,
    pub id: osm::Id,
    pub lat: osm::Coordinate,
    pub lon: osm::Coordinate,
    pub tags: HashMap<String, String>,
}

impl From<&osm::Node> for Node {
    fn from(node: &osm::Node) -> Self {
        // let node_o = node.clone();
        let tagdict = HashMap::new();
        Node {
            // inner: node.clone(),
            id: node.id,
            lat: node.lat,
            lon: node.lon,
            tags: node.tags.iter()
            .fold(tagdict, |mut d, t| {
                d.insert(t.key.clone(), t.val.clone());
                d
            }),
        }
    }
}

// Taken from https://github.com/orva/osm-xml/blob/6e0d7f6d932f353ecb5d32a54a129240cbca7e99/src/polygon.rs

struct Rule {
    key: &'static str,
    polygon: RuleType,
    values: [&'static str; 6],
}

impl Rule {
    fn has_matching_value(&self, tval: &str) -> bool {
        match self.polygon {
            RuleType::All => true,
            RuleType::Whitelist => self.values.iter().any(|val| *val != "" && *val == tval),
            RuleType::Blacklist => !self.values.iter().any(|val| *val == tval),
        }
    }
}

enum RuleType {
    All,
    Blacklist,
    Whitelist,
}

static RULES: [Rule; 26] =
    [Rule {
         key: "building",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "highway",
         polygon: RuleType::Whitelist,
         values: ["services", "rest_area", "escape", "elevator", "", ""],
     },
     Rule {
         key: "natural",
         polygon: RuleType::Blacklist,
         values: ["coastline", "cliff", "ridge", "arete", "tree_row", ""],
     },
     Rule {
         key: "landuse",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "waterway",
         polygon: RuleType::Whitelist,
         values: ["riverbank", "dock", "boatyard", "dam", "", ""],
     },
     Rule {
         key: "amenity",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "leisure",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "barrier",
         polygon: RuleType::Whitelist,
         values: ["city_wall", "ditch", "hedge", "retaining_wall", "wall", "spikes"],
     },
     Rule {
         key: "railway",
         polygon: RuleType::Whitelist,
         values: ["station", "turntable", "roundhouse", "platform", "", ""],
     },
     Rule {
         key: "area",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "boundary",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "man_made",
         polygon: RuleType::Blacklist,
         values: ["cutline", "embankment", "pipeline", "", "", ""],
     },
     Rule {
         key: "power",
         polygon: RuleType::Whitelist,
         values: ["plant", "substation", "generator", "transformer", "", ""],
     },
     Rule {
         key: "place",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "shop",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "aeroway",
         polygon: RuleType::Blacklist,
         values: ["taxiway", "", "", "", "", ""],
     },
     Rule {
         key: "tourism",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "historic",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "public_transport",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "office",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "building:part",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "military",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "ruins",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "area:highway",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "craft",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     },
     Rule {
         key: "golf",
         polygon: RuleType::All,
         values: ["", "", "", "", "", ""],
     }];


#[derive(Clone, Debug, Serialize, Deserialize)]
/// OpenStreet Way object
pub struct Way {
    // inner: osm::Way,
    pub id: osm::Id,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>,
}

impl Way {
    pub fn nodes(&self) -> Vec<i64> {
        self.nodes.iter()
        .map(|n| *n)
        .collect::<Vec<i64>>()
    }

    pub fn is_polygon(&self) -> bool {
        if self.nodes.len() > 0 && self.nodes.first() == self.nodes.last() {
            return true;
        }

        RULES.iter()
        .any(|rule| {
            let mut r = false;
            if let Some(tagval) = self.tags.get(rule.key).map(|v| v.clone()) {
                r = rule.has_matching_value(&tagval);
                // println!("Rule key {} tagval {} => {}", rule.key, tagval, r);
            }
            r
        })
    }
}

impl From<&osm::Way> for Way {
    fn from(way: &osm::Way) -> Self {
        let way_o = way.clone();
        let tagdict = HashMap::new();

        Way {
            // inner: way.clone(),
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

#[derive(Clone, Deserialize)]
/// OpenStreet Bounds object
pub struct Bounds {
    /// Min latitude
    pub minlat: f64,
    /// Min longitude
    pub minlon: f64,
    /// Max latitude
    pub maxlat: f64,
    /// Max longitude
    pub maxlon: f64,
}

pub trait TaggableElement {
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


#[derive(Clone)]
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
pub struct Map {
    // inner: osm::OSM,
    nodes: FnvHashMap<i64, Node>,
    ways: FnvHashMap<i64, Way>,
    bounds: Option<Bounds>,
}

impl Map {
    pub fn new(path: String) -> Map {
        let f = std::fs::File::open(path).unwrap();
        let doc = osm::OSM::parse(f).unwrap();

        // TODO: Remove runtime overhead by clone the xml parser
        let mut nodes: FnvHashMap<i64, Node> = FnvHashMap::default();
        for (id, node) in &doc.nodes {
            nodes.insert(*id, node.into());
        }

        let mut ways: FnvHashMap<i64, Way> = FnvHashMap::default();
        for (id, way) in &doc.ways {
            ways.insert(*id, way.into());
        }

        let bounds = doc.bounds.map(|bounds| {
            Bounds {
                minlat: bounds.minlat,
                minlon: bounds.minlon,
                maxlat: bounds.maxlat,
                maxlon: bounds.maxlon,
            }
        });

        Map {
            // inner: doc,
            ways: ways,
            nodes: nodes,
            bounds: bounds,
        }
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to WayQueryBuilder methods for available filters.
    /// Call :py:func:`WayQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn ways(&self) -> QueryBuilder<Way> {
        QueryBuilder::<Way>::new(self.ways.clone())
    }

    /// Return query builder to filter ways collection
    ///
    /// Refer to NodeQueryBuilder methods for available filters.
    /// Call :py:func:`NodeQueryBuilder.get` when done to retrieve the result.
    /// See :py:class:`Map` documentation for example.
    pub fn nodes(&self) -> QueryBuilder<Node> {
        QueryBuilder::<Node>::new(self.nodes.clone())
    }

    /// Return bounds of map
    pub fn bounds(&self) -> Option<Bounds> {
        self.bounds.clone()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    #[test]
    fn tagless_and_nonloop_is_not_polygon() {
        let way = Way {
            id: 1234567,
            tags: HashMap::new(),
            nodes: Vec::new(),
        };

        assert!(!way.is_polygon());
    }

    #[test]
    fn closed_loop_is_polygon() {
        let way = Way {
            id: 1234567,
            tags: HashMap::new(),
            nodes: vec![
                1,
                2,
                3,
                26,
                1,
            ],
        };

        assert!(way.is_polygon());
    }

    #[test]
    fn detect_ruletype_all_with_tag_val() {
        let way = Way {
            id: 1234567,
            tags: map!{
                "building".to_owned() => "this_is_not_valid".to_owned()
            },
            nodes: Vec::new(),
        };

        assert!(way.is_polygon());
    }

    #[test]
    fn detect_ruletype_all_without_tag_val() {
        let way = Way {
            id: 1234567,
            tags: map!{
                "building".to_owned() => "".to_owned()
            },
            nodes: Vec::new(),
        };

        assert!(way.is_polygon());
    }

    // #[test]
    // fn whitelist_val_included_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("highway"),
    //                        val: String::from("escape"),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(is_polygon(&way));
    // }

    // #[test]
    // fn whitelist_val_not_included_is_not_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("highway"),
    //                        val: String::from("footway"),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(!is_polygon(&way));
    // }

    // #[test]
    // fn whitelist_with_empty_val_is_not_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("highway"),
    //                        val: String::from(""),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(!is_polygon(&way));
    // }

    // #[test]
    // fn whitelist_with_matching_and_nonmatching_tags_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![
    //             Tag { key: String::from("highway"), val: String::from("footway") },
    //             Tag { key: String::from("highway"), val: String::from("escape") },
    //             ],
    //         nodes: Vec::new(),
    //     };

    //     assert!(is_polygon(&way));
    // }

    // #[test]
    // fn nonloop_and_whitelist_match_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("highway"),
    //                        val: String::from("escape"),
    //                    }],
    //         nodes: vec![
    //             UnresolvedReference::Node(1),
    //             UnresolvedReference::Node(2),
    //             UnresolvedReference::Node(3),
    //             ],
    //     };

    //     assert!(is_polygon(&way));
    // }

    // #[test]
    // fn blacklist_val_included_is_not_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("natural"),
    //                        val: String::from("cliff"),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(!is_polygon(&way));
    // }

    // #[test]
    // fn blacklist_val_not_included_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("natural"),
    //                        val: String::from("tree"),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(is_polygon(&way));
    // }

    // #[test]
    // fn blacklist_with_empty_val_is_not_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("natural"),
    //                        val: String::from(""),
    //                    }],
    //         nodes: Vec::new(),
    //     };

    //     assert!(!is_polygon(&way));
    // }

    // #[test]
    // fn blacklist_with_matching_and_nonmatching_tags_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![
    //             Tag { key: String::from("natural"), val: String::from("cliff") },
    //             Tag { key: String::from("natural"), val: String::from("tree") },
    //             ],
    //         nodes: Vec::new(),
    //     };

    //     assert!(is_polygon(&way));
    // }

    // #[test]
    // fn nonloop_and_blacklist_cleared_is_polygon() {
    //     let way = Way {
    //         id: 1234567,
    //         tags: vec![Tag {
    //                        key: String::from("natural"),
    //                        val: String::from("tree"),
    //                    }],
    //         nodes: vec![
    //             UnresolvedReference::Node(1),
    //             UnresolvedReference::Node(2),
    //             UnresolvedReference::Node(3),
    //             ],
    //     };

    //     assert!(is_polygon(&way));
    // }


    // #[test]
    // fn rules_with_no_value_are_not_polygons() {
    //     let keys = vec![
    //         "building",
    //         "highway",
    //         "natural",
    //         "landuse",
    //         "waterway",
    //         "amenity",
    //         "leisure",
    //         "barrier",
    //         "railway",
    //         "area",
    //         "boundary",
    //         "man_made",
    //         "power",
    //         "place",
    //         "shop",
    //         "aeroway",
    //         "tourism",
    //         "historic",
    //         "public_transport",
    //         "office",
    //         "building:part",
    //         "military",
    //         "ruins",
    //         "area:highway",
    //         "craft",
    //         "golf",
    //         ];

    //     let ways = keys.iter().map(|key| {
    //         return Way {
    //             id: 1234567,
    //             tags: vec![ Tag { key: String::from(*key), val: String::from("no") }, ],
    //             nodes: Vec::new(),
    //         };
    //     });

    //     for way in ways {
    //         assert!(!is_polygon(&way));
    //     }
    // }
}
