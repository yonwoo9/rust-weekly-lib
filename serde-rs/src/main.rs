use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
    address: String,
}

fn main() {
    // 创建一个 Person 实例
    let person = Person {
        name: String::from("test"),
        age: 18,
        email: String::from("test@example.com"),
        address: String::from("123 Main Street"),
    };

    // 序列化 Person 实例为 JSON 字符串
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // 反序列化 JSON 字符串为 Person 实例
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
