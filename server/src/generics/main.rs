use std::{fmt::Display, marker::PhantomData};

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

struct ExampleIterator<'a, T> {
    content: &'a str,
    phantom: PhantomData<T>,
}

impl<'a, T: Deserialize<'a> + std::fmt::Debug> ExampleIterator<'a, T> {
    fn something(self) {
        println!("{}", self.content);
        let ex: Example<T> = serde_json::from_str(self.content).unwrap();
        println!("{:?}", ex);
    }
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

    // let ex: Example<Payload> = serde_json::from_str(value).unwrap();
    // println!("{:?}", ex);

    let eit = ExampleIterator::<Payload> {
        content: value,
        phantom: PhantomData::<Payload>,
    };

    let ex = eit.something();
    println!("{:?}", ex);
}
