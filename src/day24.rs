const DATA: &'static str = include_str!("../data/input_24.txt");

use std::cmp;
use std::{u64, usize};
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
    let best_size: Cell<usize> = Cell::new(usize::MAX);
    let best_qe: Cell<u64> = Cell::new(u64::MAX);
    let stop = |ss: &[u64]| {
        (ss.iter().sum::<u64>() > weight) ||
        (ss.iter().product::<u64>() > best_qe.get()) ||
        (ss.len() > best_size.get())
    };
    let ps = PowerSet::with_prune_condition(&data, stop);
    let subsets = ps.filter(|ss| ss.iter().sum::<u64>() == weight)
                    .inspect(|ss| {
                        best_size.set(cmp::min(best_size.get(), ss.len()));
                        best_qe.set(cmp::min(best_qe.get(), ss.iter().product()));
                    })
                    .collect::<Vec<_>>();
    subsets.iter()
           .filter(|ss| ss.len() == best_size.get())
           .map(|ss| ss.iter().product::<u64>())
           .min()
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
