use dip;

use std::fs::File;
use std::io::Write;

fn main() {
    let bundestag = dip::new();

    for x in bundestag.plenarprotokoll_text().take(1) {
        let mut file = File::create("plenarprotokoll.txt").unwrap();
        file.write_all(x.text.as_bytes()).unwrap();
    }

}
