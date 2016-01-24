const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = floor_from_instructions(DATA);
    let s2 = first_instruction_to_reach(DATA, -1).unwrap();
    vec![s1.to_string(), s2.to_string()]
}

// translate input characters to a floor change (+/- 1)
fn instruction_map(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

// find final floor, just the sum of the directions
fn floor_from_instructions(input: &str) -> i32 {
    input.chars().map(instruction_map).sum()
}

// use scan to track intermediate values while summing
// then search for the desired floor with position
fn first_instruction_to_reach(input: &str, target_floor: i32) -> Option<usize> {
    input.chars()
         .scan(0, |floor, c| {
             *floor += instruction_map(c);
             Some(*floor)
         })
         .position(|floor| floor == target_floor)
         .map(|instruction| {
             // position is 0 indexed, but we want to count moves from 1
             instruction + 1
         })
}

#[cfg(test)]
mod test {
    use super::{floor_from_instructions, first_instruction_to_reach};

    #[test]
    fn examples_1() {
        assert_eq!(floor_from_instructions("(())"), 0);
        assert_eq!(floor_from_instructions("()()"), 0);
        assert_eq!(floor_from_instructions("((("), 3);
        assert_eq!(floor_from_instructions("(()(()("), 3);
        assert_eq!(floor_from_instructions("))((((("), 3);
        assert_eq!(floor_from_instructions("())"), -1);
        assert_eq!(floor_from_instructions("))("), -1);
        assert_eq!(floor_from_instructions(")))"), -3);
        assert_eq!(floor_from_instructions(")())())"), -3);
    }

    #[test]
    fn examples_2() {
        assert_eq!(first_instruction_to_reach(")", -1), Some(1));
        assert_eq!(first_instruction_to_reach("()())", -1), Some(5));
    }
}
