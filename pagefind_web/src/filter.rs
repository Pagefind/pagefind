use std::borrow::Cow;

use bit_set::BitSet;

use crate::json::Value;
use crate::util::*;
use crate::SearchIndex;

#[derive(Debug)]
pub enum FilterBehaviour {
    Any,
    All,
}

fn collapse(mut maps: Vec<BitSet>, behaviour: FilterBehaviour) -> BitSet {
    let mut maps = maps.drain(..);
    let mut output = maps.next().unwrap_or_default();

    for map in maps {
        match behaviour {
            FilterBehaviour::Any => output.union_with(&map),
            FilterBehaviour::All => output.intersect_with(&map),
        }
    }
    output
}

impl SearchIndex {
    // TODO: Move this intersection, and the string formatting, into better methods.
    pub fn get_filters(&self, obj: &mut write_json::Object, intersect_pages: Option<Vec<usize>>) {
        let intersect_pages = intersect_pages.as_ref();
        for (filter, values) in &self.filters {
            let mut filter_obj = obj.object(filter);

            for (value, pages) in values {
                let len = match intersect_pages {
                    Some(intersection) => pages
                        .iter()
                        .filter(|p| intersection.contains(&(**p as usize)))
                        .count(),
                    None => pages.len(),
                };
                filter_obj.number(value, len as f64);
            }
        }
    }

    fn invert(&self, set: &mut BitSet) {
        set.symmetric_difference_with(&BitSet::<u32>::from_iter(0..self.pages.len()));
    }

    fn parse_filter_value(
        &self,
        filter_key: &str,
        filter: &Value,
        behaviour: FilterBehaviour,
    ) -> Option<BitSet> {
        debug!({
            format! {"Processing value object {filter:?} with {behaviour:?}" }
        });

        let filter_map = self.filters.get(filter_key);
        if filter_map.is_none() {
            debug!({
                format! {"No map for {filter_key}"}
            });
        }

        let mut maps = Vec::new();
        let build_set = |val: &str| {
            debug!({
                format! {"Adding the filter {filter_key}: {val:?}" }
            });
            if let Some(Some(filter_pages)) = filter_map.map(|m| m.get(val)) {
                let mut set = BitSet::new();
                for page in filter_pages {
                    set.insert(*page as usize);
                }
                set
            } else {
                debug!({
                    format! {"Nothing found. . . ." }
                });
                // Filter does not exist, push in a set of 0 pages to force no results
                BitSet::new()
            }
        };

        match filter {
            Value::Str(s) => maps.push(build_set(s)),
            Value::Arr(arr) => {
                for value in arr {
                    match value {
                        Value::Str(s) => {
                            maps.push(build_set(s));
                        }
                        Value::Obj(_) => {
                            if let Some(inner_set) =
                                self.parse_filter_value(filter_key, value, FilterBehaviour::All)
                            {
                                maps.push(inner_set);
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Value::Obj(obj) => {
                for (k, value) in obj {
                    if let Some(Some(mut map)) = match (&**k, value) {
                        ("any" | "none", Value::Obj(_) | Value::Arr(_) | Value::Str(_)) => {
                            Some(self.parse_filter_value(filter_key, value, FilterBehaviour::Any))
                        }
                        ("all" | "not", Value::Obj(_) | Value::Arr(_) | Value::Str(_)) => {
                            Some(self.parse_filter_value(filter_key, value, FilterBehaviour::All))
                        }
                        _ => {
                            debug!({
                                format! {"Unsupported filter key {k} value {:?}", value}
                            });
                            None
                        }
                    } {
                        if matches!(&**k, "none" | "not") {
                            self.invert(&mut map);
                        }
                        maps.push(map);
                    }
                }
            }
            _ => {
                debug!({
                    format! {"Unsupported filter value {:?}", filter}
                });
                return None;
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    fn parse_filter_arr(&self, filter: &[Value], behaviour: FilterBehaviour) -> Option<BitSet> {
        debug!({
            format! {"Processing outer array {filter:?} with {behaviour:?}" }
        });

        let mut maps = Vec::new();

        for value in filter {
            let Value::Obj(value) = value else {
                debug!({
                    format! {"Skipping {value:?} as it is not an object" }
                });
                continue;
            };
            if let Some(map) = self.parse_filter_obj(value, FilterBehaviour::All) {
                maps.push(map)
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    fn parse_filter_obj(
        &self,
        filter: &[(Cow<str>, Value)],
        behaviour: FilterBehaviour,
    ) -> Option<BitSet> {
        debug!({
            format! {"Processing outer object {filter:?} with {behaviour:?}" }
        });

        let mut maps = Vec::new();

        for (k, value) in filter {
            let map = match (&**k, value) {
                ("any" | "none", Value::Obj(o)) => self.parse_filter_obj(o, FilterBehaviour::Any),
                ("all" | "not", Value::Obj(o)) => self.parse_filter_obj(o, FilterBehaviour::All),
                ("any" | "none", Value::Arr(a)) => self.parse_filter_arr(a, FilterBehaviour::Any),
                ("all" | "not", Value::Arr(a)) => self.parse_filter_arr(a, FilterBehaviour::All),
                (k, _) => self.parse_filter_value(k, value, FilterBehaviour::All),
            };
            if let Some(mut map) = map {
                if matches!(&**k, "none" | "not") {
                    self.invert(&mut map);
                }
                maps.push(map);
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    pub fn filter(&self, filter: &str) -> Option<BitSet> {
        debug!({
            format! {"Filtering with {:?}", filter}
        });

        if self.filter_chunks(filter).is_none() {
            debug!({ "No filter names anywhere in this object. Pretending it doesn't exist." });
            return None;
        }

        let Some(all_filters) = Value::parse(filter) else {
            debug!({ "Malformed object passed to Pagefind filters" });
            return None;
        };
        let Some(obj) = all_filters.as_obj() else {
            debug!({ "Filters was passed a non-object" });
            return None;
        };

        self.parse_filter_obj(obj, FilterBehaviour::All)
    }

    pub fn dig_filter(&self, filter: &Value) -> Option<Vec<String>> {
        debug!({
            format! {"Digging into {:?} to look for filters", filter}
        });

        let mut has_filters = false;
        let mut indexes = Vec::new();

        match filter {
            Value::Obj(obj) => {
                for (k, value) in obj {
                    match (&**k, value) {
                        ("any" | "all" | "not" | "none", Value::Obj(_) | Value::Arr(_)) => {
                            if let Some(nested_filters) = self.dig_filter(value) {
                                has_filters = true;
                                indexes.extend(nested_filters)
                            }
                        }
                        (filter, _) => {
                            has_filters = true;
                            if let Some(hash) = self.filter_chunks.get(filter) {
                                debug!({
                                    format! {"Need {:?} for {:?}", hash, filter}
                                });
                                indexes.push(hash.clone());
                            } else {
                                debug!({
                                    format! {"No hash found for {:?}", filter}
                                })
                            }
                        }
                    }
                }
            }
            Value::Arr(arr) => {
                for value in arr {
                    match value {
                        Value::Obj(_) | Value::Arr(_) => {
                            if let Some(nested_filters) = self.dig_filter(value) {
                                has_filters = true;
                                indexes.extend(nested_filters)
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        if has_filters {
            Some(indexes)
        } else {
            None
        }
    }

    pub fn filter_chunks(&self, filter: &str) -> Option<Vec<String>> {
        debug!({
            format! {"Finding the filter chunks needed for {:?}", filter}
        });

        let Some(all_filters) = Value::parse(filter) else {
            debug!({ "Malformed object passed to Pagefind filters" });
            return None;
        };
        if all_filters.as_obj().is_none() {
            debug!({ "Filters was passed a non-object" });
            return None;
        }

        self.dig_filter(&all_filters)
    }
}
