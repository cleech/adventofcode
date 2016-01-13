const DATA: &'static str = include_str!("../data/input_24.txt");

pub fn main() -> Vec<String> {
    let v = DATA.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let weight = v.iter().sum::<u64>() / 3;
    let s1 = min_qe(&v, weight).to_string();

    let weight = v.iter().sum::<u64>() / 4;
    let s2 = min_qe(&v, weight).to_string();
    vec![s1, s2]
}

fn min_qe(data: &[u64], weight: u64) -> u64 {
    let ps = PowerSet::with_prune_condition(&data, move |ss| ss.iter().sum::<u64>() > weight);
    let subsets = ps.filter(|ss| ss.iter().sum::<u64>() == weight).collect::<Vec<_>>();
    let size = subsets.iter().map(|ss| ss.len()).min().unwrap();
    subsets.iter()
           .filter(|ss| ss.len() == size)
           .map(|ss| ss.iter().product::<u64>())
           .min()
           .unwrap()
}

struct PowerSet<'a, T>
    where T: 'a + Copy
{
    stack: Vec<(Vec<T>, &'a [T])>,
    stop: Box<Fn(&[T]) -> bool>,
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
        where F: 'static + Fn(&[T]) -> bool
    {
        PowerSet { stop: box stop, ..PowerSet::new(data) }
    }
}

impl<'a, T> Iterator for PowerSet<'a, T>
    where T: 'a + Copy,
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
