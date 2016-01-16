extern crate serde_json;
use self::serde_json::Value;

const DATA: &'static str = include_str!("../data/input_12.txt");

pub fn main() -> Vec<String> {
    let s1 = json_sum(DATA).unwrap();
    let s2 = json_sum_no_red(DATA).unwrap();
    vec![s1.to_string(), s2.to_string()]
}

fn json_sum(s: &str) -> Result<i64, String> {
    serde_json::from_str(s)
        .map_err(|e| e.to_string())
        .map(|ref v| json_to_num(v))
}

fn json_to_num(data: &Value) -> i64 {
    match *data {
        Value::I64(n) => n,
        Value::U64(n) => n as i64,
        Value::Array(ref vec) => vec.iter().map(json_to_num).sum(),
        Value::Object(ref bt) => bt.values().map(json_to_num).sum(),
        _ => 0,
    }
}

fn json_sum_no_red(input: &str) -> Result<i64, String> {
    serde_json::from_str(input)
        .map_err(|e| e.to_string())
        .map(|ref v| json_to_num_no_red(v))
}

fn json_to_num_no_red(data: &Value) -> i64 {
    match *data {
        Value::Array(ref vec) => vec.iter().map(json_to_num_no_red).sum(),
        Value::Object(ref bt) => {
            if bt.values().any(|v| v.as_string().map_or(false, |s| s == "red")) {
                0
            } else {
                bt.values().map(json_to_num_no_red).sum()
            }
        }
        ref jv => json_to_num(jv),
    }
}

#[cfg(test)]
mod test {
    use super::{json_sum, json_sum_no_red};

    #[test]
    fn examples_part_1() {
        assert_eq!(json_sum(r#"[1,2,3]"#), Ok(6));
        assert_eq!(json_sum(r#"{"a":2,"b":4}"#), Ok(6));
        assert_eq!(json_sum(r#"[[[3]]]"#), Ok(3));
        assert_eq!(json_sum(r#"{"a":{"b":4},"c":-1}"#), Ok(3));
        assert_eq!(json_sum(r#"{"a":[-1,1]}"#), Ok(0));
        assert_eq!(json_sum(r#"[-1,{"a":1}]"#), Ok(0));
        assert_eq!(json_sum(r#"[]"#), Ok(0));
        assert_eq!(json_sum(r#"{}"#), Ok(0));
    }

    #[test]
    fn examples_part_2() {
        assert_eq!(json_sum_no_red(r#"[1,2,3]"#), Ok(6));
        assert_eq!(json_sum_no_red(r#"[1,{"c":"red","b":2},3]"#), Ok(4));
        assert_eq!(json_sum_no_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), Ok(0));
        assert_eq!(json_sum_no_red(r#"[1,"red",5]"#), Ok(6));
    }
}
