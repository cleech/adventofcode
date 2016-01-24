extern crate serde_json;
use self::serde_json::Value;

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = json_sum(DATA).unwrap();
    let s2 = json_sum_no_red(DATA).unwrap();
    vec![s1.to_string(), s2.to_string()]
}

fn json_sum(s: &str) -> Result<i64, String> {
    serde_json::from_str(s)
        .map_err(|e| e.to_string())
        .map(|ref v| json_to_num(v, &|_| true))
}

fn json_sum_no_red(s: &str) -> Result<i64, String> {
    serde_json::from_str(s)
        .map_err(|e| e.to_string())
        .map(|ref v| json_to_num(v, &|v| v.as_string().map_or(true, |s| s != "red")))
}

fn json_to_num<F>(data: &Value, filter: &F) -> i64
    where F: Fn(&Value) -> bool
{
    match *data {
        Value::I64(n) => n,
        Value::U64(n) => n as i64,
        Value::Array(ref vec) => vec.iter().map(|v| json_to_num(v, filter)).sum(),
        Value::Object(ref bt) => {
            if bt.values().all(|v| filter(v)) {
                bt.values().map(|v| json_to_num(v, filter)).sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::{json_sum, json_sum_no_red};

    #[test]
    fn examples_1() {
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
    fn examples_2() {
        assert_eq!(json_sum_no_red(r#"[1,2,3]"#), Ok(6));
        assert_eq!(json_sum_no_red(r#"[1,{"c":"red","b":2},3]"#), Ok(4));
        assert_eq!(json_sum_no_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), Ok(0));
        assert_eq!(json_sum_no_red(r#"[1,"red",5]"#), Ok(6));
    }
}
