use dip;

use std::fs::File;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut skip: usize = args[1].parse().unwrap();

    let bundestag = dip::new();

    for x in bundestag.plenarprotokoll_text() {
        let text = match x.text {
            Some(result) => result,
            None => continue,
        };

        if skip > 0 {
            skip -= 1;
            continue;
        }

        println!("{}", text);
        break
    }
}
