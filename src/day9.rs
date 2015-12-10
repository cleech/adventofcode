use std::cmp::{min, max};
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

extern crate permutohedron;

pub fn main(input: &str) -> Vec<String> {
    let map = RouteMap::build(&input);
    let s1 = map.find_shortest_route();
    let s2 = map.find_longest_route();
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

#[test]
fn test_unordered_pair() {
    assert_eq!(UnorderedPair(0, 1), UnorderedPair(1, 0));
}

#[derive(Eq, PartialEq, Debug)]
struct Route {
    cities: UnorderedPair<String>,
    distance: usize,
}

impl Route {
    fn from_str(route: &str) -> Route {
        let (src, dst, distance) = scan_fmt!(route, "{} to {} = {d}", String, String, usize);
        Route {
            cities: UnorderedPair(src.unwrap(), dst.unwrap()),
            distance: distance.unwrap(),
        }
    }
}

#[test]
fn test_route_eq() {
    assert_eq!(Route::from_str("Chicago to Detroit = 282"),
               Route::from_str("Detroit to Chicago = 282"));
}

struct RouteMap {
    cities: HashSet<String>,
    routes: HashMap<UnorderedPair<String>, usize>,
}

impl RouteMap {
    fn build(input: &str) -> RouteMap {
        let mut map = HashMap::new();
        let mut cities = HashSet::new();

        for line in input.lines() {
            let r = Route::from_str(line);
            cities.insert(r.cities.0.clone());
            cities.insert(r.cities.1.clone());
            map.insert(r.cities, r.distance);
        }
        RouteMap {
            cities: cities,
            routes: map,
        }
    }

    fn lookup(&self, a: &str, b: &str) -> Option<&usize> {
        self.routes.get(&UnorderedPair(a.to_owned(), b.to_owned()))
    }

    fn get_total_route_distance(&self, stops: &[&String]) -> usize {
        stops.windows(2)
             .map(|w| self.lookup(&w[0], &w[1]).unwrap())
             .sum()
    }

    fn find_shortest_route(&self) -> usize {
        let mut data = self.cities.iter().collect::<Vec<_>>();
        let heap = permutohedron::Heap::new(&mut data);
        heap.map(|p| self.get_total_route_distance(&p[..])).min().unwrap()
    }

    fn find_longest_route(&self) -> usize {
        let mut data = self.cities.iter().collect::<Vec<_>>();
        let heap = permutohedron::Heap::new(&mut data);
        heap.map(|p| self.get_total_route_distance(&p[..])).max().unwrap()
    }
}

#[test]
fn test_day9() {
    let routes = ["London to Dublin = 464", "London to Belfast = 518", "Dublin to Belfast = 141"]
                     .join("\n");
    let map = RouteMap::build(&routes);
    let d = map.find_shortest_route();
    assert_eq!(d, 605);
}
