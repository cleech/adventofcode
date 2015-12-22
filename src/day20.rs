const DATA: &'static str = include_str!("../data/input_20.txt");

pub fn main() -> Vec<String> {
    let target = DATA.trim().parse::<u64>().unwrap() / 10;
    vec![part1(target).to_string()]
}

fn part1(target: u64) -> u64 {
    for n in 1u64.. {
        if factors(n).iter().sum::<u64>() >= target {
            return n;
        }
    }
    0
}

fn factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    // 1..sqrt(n)
    for x in (1..).take_while(|x| x * x <= n) {
        let (d, r) = ((n / x), (n % x));
        if r == 0 {
            v.push(x);
            if d != x {
                v.push(d);
            }
        }
    }
    v.sort();
    v
}
