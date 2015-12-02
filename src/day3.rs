pub fn main(input: &str) -> Vec<String> {
    let s1 = three_one(&input);
    let s2 = three_two(&input);
    vec![s1.to_string(), s2.to_string()]
}

fn three_one(input: &str) -> i32 { -1 }

#[test]
fn test_three_one() {
    assert_eq!(three_one(""), -1);
}

fn three_two(input: &str) -> i32 { -1 }

#[test]
fn test_three_two() {
    assert_eq!(three_two(""), -1);
}
