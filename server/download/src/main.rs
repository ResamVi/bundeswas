use dip;

use std::fs::{File, self};
use std::io::Write;
use std::env;
use std::io::ErrorKind;

fn main() {
    download_inhaltsverzeichnis(10);
}

// Fills a folder with .txt files of Inhaltsverzeichnise.
fn download_inhaltsverzeichnis(count: usize) {
    let bundestag = dip::new();

    let result = fs::create_dir("inhaltsverzeichnisse");
    if let Err(e) = result {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("could not create file: {}", e),
        }
    }


    for val in bundestag.plenarprotokoll_text().take(count) {
        if val.text.is_none() {
            println!("Skipped because no text was found.");
            continue;
        }

        let name = format!("inhaltsverzeichnisse/{}.txt", val.datum);
        let text = val.text.unwrap();

        let until = text.find("Beginn:").unwrap();

        // The Inhaltsverzeichnis roughly reaches until this string.
        let content = &text[..until];

        // Overrides the files if they already exist
        let result = fs::write(name, content);
        if let Err(e) = result {
            panic!("could not write: {}", e);
        }
    }

}
