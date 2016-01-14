use std::cmp::{min, max};
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::ops::Add;
use std::str::FromStr;

extern crate itertools;
use self::itertools::Itertools;

extern crate permutohedron;

const DATA: &'static str = include_str!("../data/input_9.txt");

pub fn main() -> Vec<String> {
    let map = RouteMap::build(DATA).unwrap();
    let s1 = map.find_shortest_route().unwrap();
    let s2 = map.find_longest_route().unwrap();
    vec![s1.to_string(), s2.to_string()]
}

#[derive(Eq, Debug)]
struct UnorderedPair<T>(T, T);

impl<T: PartialEq> PartialEq for UnorderedPair<T> {
    #[inline]
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        ((self.0 == other.0) && (self.1 == other.1)) || ((self.0 == other.1) && (self.1 == other.0))
    }
}

impl<T: Hash + Ord> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (min(&self.0, &self.1), max(&self.0, &self.1)).hash(state)
    }
}

#[cfg(test)]
mod test_unordered_pairs {
    use super::UnorderedPair;

    #[test]
    fn test_unordered_pair() {
        assert_eq!(UnorderedPair(0, 1), UnorderedPair(1, 0));
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Route {
    cities: UnorderedPair<String>,
    distance: usize,
}

impl FromStr for Route {
    type Err = &'static str;

    fn from_str(route: &str) -> Result<Route, &'static str> {
        if let (Some(src), Some(dst), Some(distance)) = scan_fmt!(route,
                                                                  "{} to {} = {d}",
                                                                  String,
                                                                  String,
                                                                  usize) {
            Ok(Route {
                cities: UnorderedPair(src, dst),
                distance: distance,
            })
        } else {
            Err("parse error on route")
        }
    }
}

struct RouteMap {
    cities: HashSet<String>,
    routes: HashMap<UnorderedPair<String>, usize>,
}

impl RouteMap {
    fn build(input: &str) -> Result<RouteMap, &'static str> {
        let mut map = HashMap::new();
        let mut cities = HashSet::new();

        for line in input.lines() {
            match Route::from_str(line) {
                Ok(r) => {
                    cities.insert(r.cities.0.clone());
                    cities.insert(r.cities.1.clone());
                    map.insert(r.cities, r.distance);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(RouteMap {
            cities: cities,
            routes: map,
        })
    }

    fn lookup(&self, a: &str, b: &str) -> Option<&usize> {
        self.routes.get(&UnorderedPair(a.to_owned(), b.to_owned()))
    }

    fn get_total_route_distance(&self, stops: &[&String]) -> Option<usize> {
        stops.windows(2)
             .map(|w| self.lookup(&w[0], &w[1]))
             .fold_options(0, Add::add)
    }

    fn find_shortest_route(&self) -> Option<usize> {
        let mut data = self.cities.iter().collect::<Vec<_>>();
        let permutations = permutohedron::Heap::new(&mut data);
        permutations.filter_map(|p| self.get_total_route_distance(&p[..])).min()

    }

    fn find_longest_route(&self) -> Option<usize> {
        let mut data = self.cities.iter().collect::<Vec<_>>();
        let permutations = permutohedron::Heap::new(&mut data);
        permutations.filter_map(|p| self.get_total_route_distance(&p[..])).max()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::{Route, RouteMap};

    #[test]
    fn test_route_eq() {
        assert_eq!(Route::from_str("Chicago to Detroit = 282"),
                   Route::from_str("Detroit to Chicago = 282"));
    }

    #[test]
    fn test_day9() {
        let routes = ["London to Dublin = 464",
                      "London to Belfast = 518",
                      "Dublin to Belfast = 141"]
                         .join("\n");
        let map = RouteMap::build(&routes).unwrap();
        let d = map.find_shortest_route();
        assert_eq!(d, Some(605));
    }
}
