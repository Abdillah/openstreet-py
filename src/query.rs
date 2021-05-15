use fnv;
use osm_xml as osm;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
enum FilterQuery {
    // Non-lazy filter already O(1)
    // ById(i64),
    ByTag(String, Vec<String>),
    HasTag(String),
    // Node only:
    // Way only:
    IsPolygon,
}

trait Filter<T> {
    fn filter(&self, item: T) -> bool;
}

impl Filter<&osm::Node> for FilterQuery {
    fn filter(&self, item: &osm::Node) -> bool {
        match self {
            Self::ByTag(k, values) => {
                for osm::Tag {key, val, ..} in &item.tags {
                    if !values.contains(&val) {
                        return false;
                    }
                }
                return true;
            },
            _ => panic!("You're using exclusive filter on wrong type")
        }
    }
}

impl Filter<&osm::Way> for FilterQuery {
    fn filter(&self, item: &osm::Way) -> bool {
        match self {
            Self::ByTag(k, values) => {
                for osm::Tag {key, val, ..} in &item.tags {
                    if !values.contains(&val) {
                        return false;
                    }
                }
                return true;
            },
            Self::IsPolygon => item.is_polygon(),
            _ => panic!("You're using exclusive filter on wrong type")
        }
    }
}


#[derive(Clone)]
pub(crate) struct Builder<T> {
    storage: std::sync::Arc<fnv::FnvHashMap<osm::Id, T>>,
    conditions: Vec<FilterQuery>
}

pub trait BuilderGet<T> {
    fn get(&self) -> Vec<&T>;
}

impl<T> Builder<T> {
    pub fn new(s: fnv::FnvHashMap<osm::Id, T>) -> Builder<T> {
        Builder {
            storage: std::sync::Arc::new(s),
            conditions: vec![],
        }
    }

    fn filters(&self) -> Vec<FilterQuery> {
        self.conditions.clone()
    }

    pub fn by_tag_eq(mut self, key: &str, value: &str) -> Self {
        self.conditions.push(FilterQuery::ByTag(key.to_string(), vec![ value.to_string() ]));
        self
    }

    pub fn by_tag_in(mut self, key: &str, values: Vec<&str>) -> Self {
        let values = values.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        self.conditions.push(FilterQuery::ByTag(key.to_string(), values));
        self
    }

    pub fn has_tag(mut self, key: &str) -> Self {
        unimplemented!()
    }

    pub fn by_id(&self, id: i64) -> &T {
        self.storage.get(&id)
        .expect(format!("No data with id {} found", id).as_str())
    }
}

impl Builder<osm::Way> {
    pub fn is_poly(mut self) -> Self {
        self.conditions.push(FilterQuery::IsPolygon);
        self
    }
}

trait BuilerGet<T> {
    fn get(&self) -> Vec<&T>;
}

impl BuilderGet<osm::Way> for Builder<osm::Way> {
    fn get(&self) -> Vec<&osm::Way> {
        let mut r: Vec<&osm::Way> = vec![];
        for (k, v) in self.storage.iter() {
            for c in &self.conditions {
                if c.filter(v) {
                    r.push(v)
                }
            }
        }
        r
    }
}

impl BuilderGet<osm::Node> for Builder<osm::Node> {
    fn get(&self) -> Vec<&osm::Node> {
        let mut r: Vec<&osm::Node> = vec![];
        for (k, v) in self.storage.iter() {
            for c in &self.conditions {
                if c.filter(v) {
                    r.push(v)
                }
            }
        }
        r
    }
}

mod test {
    use std::fs::File;

    use crate::query::Builder;
    use crate::query::FilterQuery;

    #[test]
    fn test_by_multiple_tag() {
        let f = File::open("resources/madina.osm").unwrap();
        let doc = osm::OSM::parse(f).unwrap();

        let mut qstreets = Builder::new(doc.ways)
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
    }
}
