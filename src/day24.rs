const DATA: &'static str = include_str!("../data/input_24.txt");

use std::cmp;
use std::{usize, u64};
use std::cell::Cell;

pub fn main() -> Vec<String> {
    if let Ok(mut v) = DATA.lines()
                           .map(|l| l.parse::<u64>())
                           .collect::<Result<Vec<u64>, _>>() {
        v.sort_by(|a, b| b.cmp(a));

        let weight = v.iter().sum::<u64>() / 3;
        let s1 = min_qe(&v, weight).unwrap().to_string();

        let weight = v.iter().sum::<u64>() / 4;
        let s2 = min_qe(&v, weight).unwrap().to_string();

        vec![s1, s2]
    } else {
        vec![]
    }
}

fn min_qe(data: &[u64], weight: u64) -> Option<u64> {
    let best = Cell::new((usize::MAX, u64::MAX));
    let stop = |ss: &[u64]| {
        (ss.iter().sum::<u64>() > weight) || ((ss.len(), ss.iter().product()) > best.get())
    };
    let ps = PowerSet::with_prune_condition(&data, stop);
    return ps.filter(|ss| ss.iter().sum::<u64>() == weight)
             .map(|ss| (ss.len(), ss.iter().product::<u64>()))
             .inspect(|&k| best.set(cmp::min(best.get(), k)))
             .min()
             .map(|k| k.1);
}

#[cfg(test)]
mod test {
    use super::min_qe;

    #[test]
    fn example_1() {
        let data = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(min_qe(&data, data.iter().sum::<u64>() / 3), Some(99));
    }

    #[test]
    fn example_2() {
        let data = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(min_qe(&data, data.iter().sum::<u64>() / 4), Some(44));
    }
}

struct PowerSet<'a, T>
    where T: 'a + Copy
{
    stack: Vec<(Vec<T>, &'a [T])>,
    stop: Box<Fn(&[T]) -> bool + 'a>,
}

impl<'a, T> PowerSet<'a, T> where T: 'a + Copy
{
    fn new(data: &'a [T]) -> PowerSet<'a, T> {
        let mut stack = Vec::new();
        if let Some((selected, remaining)) = data.split_last() {
            stack.push((vec![*selected], remaining));
            stack.push((vec![], remaining));
        } else {
            stack.push((vec![], data))
        }
        PowerSet {
            stack: stack,
            stop: box |_| false,
        }
    }

    fn with_prune_condition<F>(data: &'a [T], stop: F) -> PowerSet<'a, T>
        where F: 'a + Fn(&[T]) -> bool
    {
        PowerSet { stop: box stop, ..PowerSet::new(data) }
    }
}

impl<'a, T> Iterator for PowerSet<'a, T> where T: 'a + Copy
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if let Some((selected, remaining)) = self.stack.pop() {
            if let Some((sel, rem)) = remaining.split_last() {
                if !(self.stop)(&selected) {
                    let mut r = selected.clone();
                    r.push(*sel);
                    self.stack.push((r, rem));
                    self.stack.push((selected, rem));
                }
                self.next()
            } else {
                Some(selected)
            }
        } else {
            None
        }
    }
}
