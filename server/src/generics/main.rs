use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Example<T> {
    code: i32,
    success: bool,
    payload: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    features: Vec<String>,
}

fn main() {
    let value = r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ]
        }
    }"#;

    let ex: Example<Payload> = serde_json::from_str(value).unwrap();
    println!("{:?}", ex);
}
