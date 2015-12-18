const DATA: &'static str = include_str!("../data/input_17.txt");

pub fn main() -> Vec<String> {
    vec![part1().to_string(), part2().to_string()]
}

fn part1() -> usize {
    let data: Vec<u32> = DATA.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let ps = PowerSet::new(&data);
    let v = ps.filter(|ss| ss.iter().sum::<u32>() == 150).collect::<Vec<_>>();
    v.len()
}

fn part2() -> usize {
    let data: Vec<u32> = DATA.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let ps = PowerSet::new(&data);
    let v = ps.filter(|ss| ss.iter().sum::<u32>() == 150).collect::<Vec<_>>();
    let min = v.iter().map(|ss| ss.len()).min().unwrap();
    v.iter().filter(|ss| ss.len() == min).count()
}

struct PowerSet<T> {
    stack: Vec<(Vec<T>, Vec<T>)>,
}

impl<T: Copy> PowerSet<T> {
    fn new(data: &[T]) -> PowerSet<T> {
        let mut stack = Vec::new();
        if let Some((selected, remaining)) = data.split_last() {
            stack.push((vec![*selected], remaining.to_vec()));
            stack.push((vec![], remaining.to_vec()));
        } else {
            stack.push((vec![], vec![]));
        }
        PowerSet { stack: stack }
    }
}

impl<T: Copy> Iterator for PowerSet<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if let Some((selected, remaining)) = self.stack.pop() {
            if let Some((sel, rem)) = remaining.split_last() {
                let mut r = selected.clone();
                r.push(*sel);
                self.stack.push((r, rem.to_vec()));
                self.stack.push((selected.clone(), rem.to_vec()));
                self.next()
            } else {
                Some(selected)
            }
        } else {
            None
        }
    }
}
