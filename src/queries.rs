/* query.rs */
use std::fmt::Debug;

use osm_xml as osm;

use crate::map;
use crate::map::TaggableElement;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterQuery {
    // Non-lazy filter already O(1)
    // ById(i64),
    ByTag(String, Vec<String>),
    HasTag(String),
    // Node only:
    // Way only:
    IsPolygon,
    HasNodes(Vec<osm::Id>),
}

trait Filter<T> where T: TaggableElement {
    fn filter(&self, item: T) -> bool;
}

impl Filter<map::Node> for FilterQuery {
    fn filter(&self, item: map::Node) -> bool {
        match self {
            Self::ByTag(k, values) => {
                if let Some(val) = item.get_tag_value(k) {
                    if values.contains(&val.to_owned()) {
                        return true;
                    }
                }
                return false;
            },
            _ => panic!("You're using exclusive filter on wrong type")
        }
    }
}

impl Filter<map::Way> for FilterQuery {
    fn filter(&self, item: map::Way) -> bool {
        match self {
            Self::ByTag(k, values) => {
                if let Some(val) = item.get_tag_value(k) {
                    if values.contains(&val.to_owned()) {
                        return true;
                    }
                }
                return false;
            },
            Self::IsPolygon => item.is_polygon(),
            Self::HasNodes(node_ids) => {
                // TODO: We will create faster index later
                for element_id in &item.nodes() {
                    if node_ids.contains(&element_id) {
                        return true;
                    }
                }
                return false;
            },
            _ => panic!("You're using exclusive filter on wrong type")
        }
    }
}

pub trait QueryBuilder<T> {
    fn append_filter(&mut self, f: FilterQuery);

    fn by_tag_eq(mut self, key: &str, value: &str) -> Self where Self: Sized {
        self.append_filter(FilterQuery::ByTag(key.to_string(), vec![ value.to_string() ]));
        self
    }

    fn by_tag_in(mut self, key: &str, values: Vec<&str>) -> Self where Self: Sized {
        let values = values.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        self.append_filter(FilterQuery::ByTag(key.to_string(), values));
        self
    }

    fn has_tag(self, _key: &str) -> Self where Self: Sized {
        unimplemented!()
    }

    fn get(&self) -> Vec<T>;

    fn by_id(&self, id: i64) -> T;
}

// #[derive(Clone)]
pub struct BuilderIter<T> where T: Clone {
    // iter: dyn std::iter::IntoIterator<Item = (i64, T), IntoIter = std::collections::hash_map::IntoIter<i64, T>>,
    into_iter: std::collections::hash_map::IntoIter<i64, T>,
    // iter: std::collections::hash_map::Iter<'a, osm::Id, T>,
    conditions: Vec<FilterQuery>,
}

impl<'a, T: Clone> BuilderIter<T> {
    fn new(into_iter: std::collections::hash_map::IntoIter<i64, T>, conditions: Vec<FilterQuery>) -> Self {
        BuilderIter { into_iter, conditions }
    }
}

impl<'a, T: TaggableElement + Debug + Clone> Iterator for BuilderIter<T>
where
    FilterQuery: Filter<T>
{
    type Item = (osm::Id, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((k, v)) = self.into_iter.next() {
            if self.conditions.iter().all(|c| c.filter(v.clone())) {
                return Some((k, v));
            }
        }
        return None;
    }
}

#[derive(Clone)]
pub struct Builder<T: Clone> {
    storage: std::sync::Arc<fnv::FnvHashMap<osm::Id, T>>,
    // iter: Option<std::collections::hash_map::Iter<'static, osm::Id, T>>,
    conditions: Vec<FilterQuery>,
}

impl<T: Clone> Builder<T> {
    pub fn new(s: fnv::FnvHashMap<osm::Id, T>) -> Builder<T> {
        Builder {
            storage: std::sync::Arc::new(s),
            conditions: vec![],
        }
    }

    pub fn iter(&self) -> BuilderIter<T> {
        let storage = (*self.storage).clone();
        let conditions = self.conditions.clone();
        BuilderIter::new(storage.into_iter(), conditions)
    }

    fn filters(&self) -> Vec<FilterQuery> {
        self.conditions.clone()
    }
}

impl Builder<map::Way> {
    pub fn contain_nodes(mut self, node_ids: Vec<i64>) -> Self {
        self.conditions.push(FilterQuery::HasNodes(node_ids));
        self
    }

    pub fn is_poly(mut self) -> Self {
        self.conditions.push(FilterQuery::IsPolygon);
        self
    }
}


impl QueryBuilder<map::Way> for Builder<map::Way> {
    fn append_filter(&mut self, f: FilterQuery) {
        self.conditions.push(f);
    }

    fn by_id(&self, id: i64) -> map::Way {
        self.storage.get(&id)
        .map(|w| w.clone())
        .expect(format!("No data with id {} found", id).as_str())
    }

    fn get(&self) -> Vec<map::Way> {
        let mut r: Vec<map::Way> = vec![];
        for (_k, v) in self.storage.iter() {
            for c in &self.conditions {
                if c.filter(v.clone()) {
                    r.push(v.clone())
                }
            }
        }
        r
    }
}

impl QueryBuilder<map::Node> for Builder<map::Node> {
    fn append_filter(&mut self, f: FilterQuery) {
        self.conditions.push(f);
    }

    fn by_id(&self, id: i64) -> map::Node {
        self.storage.get(&id)
        .map(|n| n.clone())
        .expect(format!("No data with id {} found", id).as_str())
    }

    fn get(&self) -> Vec<map::Node> {
        let mut r: Vec<map::Node> = vec![];
        for (_k, v) in self.storage.iter() {
            for c in &self.conditions {
                if c.filter(v.clone()) {
                    r.push(v.clone())
                }
            }
        }
        r
    }
}


#[cfg(test)]
mod test {
    use osm_xml as osm;
    use crate::queries::Builder;
    use crate::queries::FilterQuery;
    use crate::queries::QueryBuilder;

    #[test]
    fn test_iter() {
        let f = std::fs::File::open("resources/madina.osm").unwrap();
        let doc = osm::OSM::parse(f).unwrap();

        let mut hm: fnv::FnvHashMap<i64, crate::map::Way> = fnv::FnvHashMap::default();
        for (key, value) in doc.ways.iter() {
            hm.insert(*key, value.into());
        }

        let mut qstreets: Builder<crate::map::Way> = Builder::new(hm)
        .by_tag_in("highway", vec![
            "primary"      , "secondary"      , "tertiary",
            "primary_link" , "secondary_link" , "tertiary_link",
            "residential"  , "service"
        ]);

        let mut iter = qstreets.iter();
        assert!(iter.next().is_some());
    }

    #[test]
    fn test_by_multiple_tag() {
        let f = std::fs::File::open("resources/madina.osm").unwrap();
        let doc = osm::OSM::parse(f).unwrap();

        let mut hm: fnv::FnvHashMap<i64, crate::map::Way> = fnv::FnvHashMap::default();
        for (key, value) in doc.ways.iter() {
            hm.insert(*key, value.into());
        }

        let highway_filter = vec![
            "primary"      , "secondary"      , "tertiary",
            "primary_link" , "secondary_link" , "tertiary_link",
            "residential"  , "service"
        ];

        let mut qstreets: Builder<crate::map::Way> = Builder::new(hm.clone())
        .by_tag_in("highway", vec![
            "primary"      , "secondary"      , "tertiary",
            "primary_link" , "secondary_link" , "tertiary_link",
            "residential"  , "service"
        ]);

        std::assert!(
            FilterQuery::ByTag("highway".to_string(), vec![
                "primary"      , "secondary"      , "tertiary",
                "primary_link" , "secondary_link" , "tertiary_link",
                "residential"  , "service"
            ].iter().map(|s| s.to_string()).collect::<Vec<String>>())
            == qstreets.filters().first().unwrap().clone()
        );

        qstreets.iter().for_each(|(k, v)| {
            let tagval = v.tags.get("highway").map(|s| s.clone()).unwrap_or("".to_owned());
            assert!(highway_filter.contains(&tagval.as_str()));
        });
    }
}
