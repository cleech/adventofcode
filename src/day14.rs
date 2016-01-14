use std::collections::HashMap;
use std::iter::Iterator;
use std::cmp;
use std::str::FromStr;

const DATA: &'static str = include_str!("../data/input_14.txt");

pub fn main() -> Vec<String> {
    let s1 = race(DATA, 2503);
    let s2 = race_2(DATA, 2503);
    vec![s1.to_string(), s2.to_string()]
}

struct Reindeer {
    name: String,
    speed: usize,
    stamina: usize,
    rest: usize,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(input: &str) -> Result<Reindeer, ()> {
        if let (Some(name), Some(speed), Some(stamina), Some(rest)) = scan_fmt!(input,
                                                                                "{} can fly {d} \
                                                                                 km/s for {d} \
                                                                                 seconds, but \
                                                                                 then must rest \
                                                                                 for {d} seconds\
                                                                                 .",
                                                                                String,
                                                                                usize,
                                                                                usize,
                                                                                usize) {
            Ok(Reindeer {
                name: name,
                speed: speed,
                stamina: stamina,
                rest: rest,
            })
        } else {
            Err(())
        }
    }
}

impl Reindeer {
    fn speed_at_time(&self, second: usize) -> usize {
        // a reindeer flies in a cycle of full speed and rest
        // figure out where in the reindeers individual cycle it is
        // to determine out how fast it travels in any particular second
        let cycle = self.stamina + self.rest;
        if (second % cycle) < self.stamina {
            self.speed
        } else {
            0
        }
    }

    fn distance_at_time(&self, second: usize) -> usize {
        (0..second).map(|t| self.speed_at_time(t)).sum()
    }
}

fn race(input: &str, seconds: usize) -> usize {
    input.lines()
         .filter_map(|line| Reindeer::from_str(line).ok())
         .map(|r| r.distance_at_time(seconds))
         .max()
         .unwrap_or(0)
}

fn race_2(input: &str, seconds: usize) -> usize {
    let reindeer = input.lines()
                        .filter_map(|line| Reindeer::from_str(line).ok())
                        .collect::<Vec<_>>();
    let mut scorecard: HashMap<&str, (usize, usize)> = HashMap::new();
    for r in &reindeer {
        scorecard.insert(&r.name[..], (0usize, 0));
    }

    let mut furthest = 0;
    for t in 0..seconds {
        for r in &reindeer {
            if let Some(&mut (_, ref mut distance)) = scorecard.get_mut(&r.name[..]) {
                // *distance = r.distance_at_time(t+1);
                *distance += r.speed_at_time(t);
                furthest = cmp::max(furthest, *distance);
            }
        }
        for r in &reindeer {
            if let Some(&mut (ref mut score, ref distance)) = scorecard.get_mut(&r.name[..]) {
                if *distance == furthest {
                    *score += 1;
                }
            }
        }
    }
    if let Some(winner) = scorecard.iter()
                                   .max_by_key(|&(_, v)| v.0) {
        (winner.1).0
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::{race, race_2};

    #[test]
    fn test_day14() {
        let input = ["Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                     "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."]
                        .join("\n");
        assert_eq!(race(&input, 1000), 1120);
        assert_eq!(race_2(&input, 1000), 689);
    }
}
