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
    let ps = PowerSet::with_prune_condition(&data, |ss| ss.iter().sum::<u64>() > weight);
    let subsets = ps.filter(|ss| ss.iter().sum::<u64>() == weight).collect::<Vec<_>>();
    let size = subsets.iter().map(|ss| ss.len()).min().unwrap();
    subsets.iter()
           .filter(|ss| ss.len() == size)
           .map(|ss| ss.iter().product::<u64>())
           .min()
           .unwrap()
}

struct PowerSet<'a, T, F>
    where T: 'a + Copy,
          F: Fn(&[T]) -> bool
{
    stack: Vec<(Vec<T>, &'a [T])>,
    stop: F,
}

impl<'a, T, F> PowerSet<'a, T, F>
    where T: 'a + Copy,
          F: Fn(&[T]) -> bool
{
    // not sure why this doesn't compile
    // expected type parameter, found fn item
    //
    //  fn new(data: &'a [T]) -> PowerSet<'a, T, F> {
    //      PowerSet::with_prune_condition(data, |_| false)
    //  }

    fn with_prune_condition(data: &'a [T], stop: F) -> PowerSet<'a, T, F> {
        let mut stack = Vec::new();
        if let Some((selected, remaining)) = data.split_last() {
            stack.push((vec![*selected], remaining));
            stack.push((vec![], remaining));
        } else {
            stack.push((vec![], data))
        }
        PowerSet {
            stack: stack,
            stop: stop,
        }
    }
}

impl<'a, T, F> Iterator for PowerSet<'a, T, F>
    where T: 'a + Copy,
          F: Fn(&[T]) -> bool
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
