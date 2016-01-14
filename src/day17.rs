const DATA: &'static str = include_str!("../data/input_17.txt");
const TARGET: u32 = 150;

pub fn main() -> Vec<String> {
    let mut data: Vec<u32> = DATA.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    // search with pruning works best if the data is sorted
    data.sort();
    let ps = PowerSet::with_prune_condition(&data, |ss| ss.iter().sum::<u32>() > TARGET);
    let v = ps.filter(|ss| ss.iter().sum::<u32>() == TARGET).collect::<Vec<_>>();
    vec![v.len().to_string(),
         {
             let min = v.iter().map(|ss| ss.len()).min().unwrap();
             v.iter().filter(|ss| ss.len() == min).count().to_string()
         }]
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
