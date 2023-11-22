use serde_json::Value;

pub fn part1(input: &str) -> i64 {
    let object: Value = serde_json::from_str(input).unwrap();
    sum_numbers(&object)
}

pub fn part2(input: &str) -> i64 {
    let object: Value = serde_json::from_str(input).unwrap();
    sum_numbers_no_red(&object)
}

fn sum_numbers(object: &Value) -> i64 {
    match object {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(iterable) => iterable.iter().map(sum_numbers).sum(),
        Value::Object(iterable) => iterable.iter().map(|(_, v)| sum_numbers(v)).sum(),
        _ => 0,
    }
}

fn sum_numbers_no_red(object: &Value) -> i64 {
    match object {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(iterable) => iterable.iter().map(sum_numbers_no_red).sum(),
        Value::Object(iterable) => {
            let mut sum = 0;
            for (_, v) in iterable {
                if matches!(v, Value::String(s) if s == "red") {
                    return 0;
                }
                sum += sum_numbers_no_red(v);
            }
            sum
        }
        _ => 0,
    }
}
