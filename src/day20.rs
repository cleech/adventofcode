const DATA: &'static str = include_str!("../data/input_20.txt");

pub fn main() -> Vec<String> {
    let target = DATA.trim().parse::<usize>().unwrap();
    vec![part1(target).to_string(), part2(target).to_string()]
}

fn part1(target: usize) -> usize {
    // each house[n] will get 10 * n gifts from elf[n]
    // so target/10 is a safe upper bound for the number of houses (and elves)
    let max_houses = target / 10;
    let mut houses = vec![0; max_houses];

    for elf in 1..max_houses {
        let mut house = elf;
        while house < max_houses {
            houses[house] += elf * 10;
            house += elf;
        }
    }
    houses.into_iter().position(|p| p >= target).unwrap()
}

fn part2(target: usize) -> usize {
    let max_houses = target / 11;
    let mut houses = vec![0; max_houses];

    for elf in 1..target {
        let mut house = elf;
        let mut i = 0;
        while house < max_houses && i < 50 {
            houses[house] += elf * 11;
            house += elf;
            i += 1;
        }
    }
    houses.into_iter().position(|p| p >= target).unwrap()
}
