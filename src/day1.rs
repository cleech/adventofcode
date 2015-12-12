pub fn main(input: &str) -> Vec<String> {
    let s1 = floor_from_instructions(&input);
    let s2 = fist_instruction_to_reach(&input, -1);
    vec![s1.to_string(), s2.to_string()]
}

fn instruction_map(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => panic!("invalid input"),
    }
}

fn floor_from_instructions(input: &str) -> i32 {
    input.chars().map(instruction_map).sum()
}

#[test]
fn test_floor_from_inst() {
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

fn fist_instruction_to_reach(input: &str, target_floor: i32) -> usize {
    input.chars()
         .map(instruction_map)
         .scan(0, |floor, direction| {
             *floor += direction;
             Some(*floor)
         })
         .position(|floor| floor == target_floor)
         .unwrap() + 1
}

#[test]
fn test_first_inst_to_reach() {
    assert_eq!(fist_instruction_to_reach(")", -1), 1);
    assert_eq!(fist_instruction_to_reach("()())", -1), 5);
}
