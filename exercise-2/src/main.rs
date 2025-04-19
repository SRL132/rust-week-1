use std::collections::HashMap;

fn main() {
    let mut values: HashMap<String, u64> = HashMap::new();

    values.insert(String::from("test"), 12345);
    println!("\"test\" associated value is {}", values.get(&String::from("test")).unwrap());
}