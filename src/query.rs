#[derive(Clone, PartialEq)]
pub(crate) enum FilterQuery {
    // Non-lazy filter already O(1)
    // ById(i64),
    ByTag(String, Vec<String>),
    HasTag(String),
    // Node only:
    // Way only:
    IsPolygon,
    HasNodes(Vec<osm::Id>),
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
            Self::HasNodes(node_ids) => {
                // TODO: We will create faster index later
                for element in &item.nodes {
                    if let osm::UnresolvedReference::Node(id) = element {
                        if node_ids.contains(&id) {
                            return true;
                        }
                    }
                }
                return false;
            },
            _ => panic!("You're using exclusive filter on wrong type")
        }
    }
}

pub(crate) trait QueryBuilder<T> {
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

    fn has_tag(mut self, key: &str) -> Self where Self: Sized {
        unimplemented!()
    }

    fn get(&self) -> Vec<&T>;

    fn by_id(&self, id: i64) -> &T;
}


#[derive(Clone)]
pub(crate) struct Builder<T> {
    storage: std::sync::Arc<fnv::FnvHashMap<osm::Id, T>>,
    conditions: Vec<FilterQuery>,
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
}

impl<T> Iterator for Builder<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.storage.iter();
        unimplemented!()
    }
}

impl Builder<osm::Way> {
    pub fn contain_nodes(mut self, node_ids: Vec<i64>) -> Self {
        self.conditions.push(FilterQuery::HasNodes(node_ids));
        self
    }

    pub fn is_poly(mut self) -> Self {
        self.conditions.push(FilterQuery::IsPolygon);
        self
    }
}


impl QueryBuilder<osm::Way> for Builder<osm::Way> {
    fn append_filter(&mut self, f: FilterQuery) {
        self.conditions.push(f);
    }

    fn by_id(&self, id: i64) -> &osm::Way {
        self.storage.get(&id)
        .expect(format!("No data with id {} found", id).as_str())
    }

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

impl QueryBuilder<osm::Node> for Builder<osm::Node> {
    fn append_filter(&mut self, f: FilterQuery) {
        self.conditions.push(f);
    }

    fn by_id(&self, id: i64) -> &osm::Node {
        self.storage.get(&id)
        .expect(format!("No data with id {} found", id).as_str())
    }

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
