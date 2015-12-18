const DATA: &'static str = include_str!("../data/input_17.txt");

pub fn main() -> Vec<String> {
    vec![part1().to_string(), part2().to_string()]
}

fn part1() -> usize {
    let mut data: Vec<u32> = DATA.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    data.sort();
    let ps = PowerSet::with_prune_condition(&data, |ss| ss.iter().sum::<u32>() > 150);
    let v = ps.filter(|ss| ss.iter().sum::<u32>() == 150).collect::<Vec<_>>();
    v.len()
}

fn part2() -> usize {
    let mut data: Vec<u32> = DATA.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    data.sort();
    let ps = PowerSet::with_prune_condition(&data, |ss| ss.iter().sum::<u32>() > 150);
    let v = ps.filter(|ss| ss.iter().sum::<u32>() == 150).collect::<Vec<_>>();
    let min = v.iter().map(|ss| ss.len()).min().unwrap();
    v.iter().filter(|ss| ss.len() == min).count()
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
            // stack.push((vec![], &[T::zero(); 0][..]))
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
