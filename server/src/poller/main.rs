use dip;

use core::fmt;
use core::iter;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

fn main() {
    let bundestag = dip::BundestagsAPI::new();

    for i in bundestag.aktivitaeten().take(2) {
        println!("{}", i);
    };
}
