extern crate serde_json;
use self::serde_json::Value;

pub fn main(input: &str) -> Vec<String> {
    let s1 = read_json(input);
    let s2 = part_2(input);
    vec![s1.to_string(), s2.to_string()]
}

fn read_json(input: &str) -> i64 {
    let data: Value = serde_json::from_str(input).unwrap();
    total_numbers(&data)
}

fn total_numbers(data: &Value) -> i64 {
    match data {
        &Value::Null => 0,
        &Value::Bool(_) => 0,
        &Value::I64(n) => n,
        &Value::U64(n) => n as i64,
        &Value::F64(_) => panic!("god I hope not"),
        &Value::String(_) => 0,
        &Value::Array(ref vec) => vec.iter().map(|v| total_numbers(v)).sum(),
        &Value::Object(ref bt) => bt.iter().map(|(_, v)| total_numbers(v)).sum(),
    }
}

fn part_2(input: &str) -> i64 {
    let data: Value = serde_json::from_str(input).unwrap();
    _part_2(&data)
}

fn _part_2(data: &Value) -> i64 {
    match data {
        &Value::Null => 0,
        &Value::Bool(_) => 0,
        &Value::I64(n) => n,
        &Value::U64(n) => n as i64,
        &Value::F64(_) => panic!("god I hope not"),
        &Value::String(_) => 0,
        &Value::Array(ref vec) => vec.iter().map(|v| _part_2(v)).sum(),
        &Value::Object(ref bt) => {
            if bt.iter().any(|(_, v)| v.as_string().map(|s| s == "red").unwrap_or(false)) {
                println!("skipping {:?}", data);
                0
            } else {
                println!("no red in? {:?}", data);
                bt.iter().map(|(_, v)| _part_2(v)).sum()
            }
        }
    }
}
