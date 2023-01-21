use dip::{self, PlenarprotokollText};

use std::fs::{File, self};
use std::env;
use std::io::ErrorKind;

use chrono::{self, NaiveDate, Datelike, Weekday};
use clap::{ Parser, ArgGroup };
use spinners::{Spinner, Spinners};

mod download;

#[derive(Parser)]
struct Args {
    /// What Plenarprotokoll to find.
    input: String,

    /// Downloads the selected Plenarprotokoll and all n after it.
    #[arg(short, long)]
    count: Option<usize>,

    ///
    #[clap(value_enum)]
    #[arg(short, long)]
    typ: Typ,

    /// Output File format
    #[clap(value_enum)]
    #[arg(short, long)]
    format: download::Format,
}

/// 
#[derive(clap::ValueEnum, Clone, Debug)]
enum Typ {
    Plenarprotokoll,
    Sitzung,
    Datum,
}


// download -f json -t plenarprotokoll 5449
// download -f txt -t sitzung 8
fn main() {
    let args = Args::parse();

    match args.typ {
        Typ::Plenarprotokoll => download_by_id(args.count.unwrap_or(1), &Id(args.input), &args.format),
        Typ::Sitzung => todo!(),
        Typ::Datum => todo!(),
    };
}

// Deutscher Bundestag – 20. Wahlperiode – 68. Sitzung. Berlin
fn download_seiten(count: usize) {
    let bundestag = dip::new();

    let result = fs::create_dir("plenarprotokolle");
    if let Err(e) = result {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("could not create file: {}", e),
        }
    }

    for val in bundestag.plenarprotokoll_texte().take(count) {
        println!("{}", val.wahlperiode);
    }
}

trait Matcher {
    fn matches(&self, plenarprotokoll: &dip::PlenarprotokollText) -> bool;
}

/// An input parameter that should be interpreted as an Id of a Plenarprotokoll.
struct Id(String);

impl Matcher for Id {
    fn matches(&self, plenarprotokoll: &dip::PlenarprotokollText) -> bool {
        plenarprotokoll.id == self.0
    }
}



// Fills a folder "plenarprotokolle" with .json files of Plenarprotokolle ready to be labelled in Label Studio.
fn download_by_id(desired_count: usize, rule: &dyn Matcher, format: &dyn download::Downloader) {
    let bundestag = dip::new();

    // Create folder if it does not exist.
    let result = fs::create_dir("plenarprotokolle");
    if let Err(e) = result {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("could not create directory: {}", e),
        }
    }

    // Start a spinner when looking.
    let mut sp = Spinner::new(Spinners::Dots10, "Searching through all Plenarprotokolle.".into());

    // Start looking for a match.
    let mut skipped = 0;
    for plenarprotokoll in bundestag.plenarprotokoll_texte() {
        // Skip until we find a first occurence.
        if !rule.matches(&plenarprotokoll) {
            skipped += 1;
            continue
        }
        break;
    }
    sp.stop_and_persist("✔", "Found a match.".into());


    // Download match + some more if specified.
    let mut download_count = 0;
    for plenarprotokoll in bundestag.plenarprotokoll_texte().skip(skipped).take(desired_count) {
        // Stop downloading when the requested amount of Plenarprotokolle is reached.
        if download_count == desired_count {
            break
        }

        // Sometimes a Plenarprotokoll is listed but no text has been published yet.
        if plenarprotokoll.text.is_none() {
            println!("\nSkipped Plenarprotokoll '{}' because no text has been published yet.", plenarprotokoll.id);
            continue;
        }

        format.download(plenarprotokoll);
        download_count += 1;
    }
}

// Fills a folder with .txt files of Inhaltsverzeichnise (the first pages of a Plenarprotokoll).
// fn download_inhaltsverzeichnis(count: usize) {
//     let bundestag = dip::new();

//     let result = fs::create_dir("inhaltsverzeichnisse");
//     if let Err(e) = result {
//         match e.kind() {
//             std::io::ErrorKind::AlreadyExists => (),
//             _ => panic!("could not create file: {}", e),
//         }
//     }


//     for val in bundestag.plenarprotokoll_texte().take(count) {
//         if val.text.is_none() {
//             println!("Skipped because no text was found.");
//             continue;
//         }

//         let name = format!("inhaltsverzeichnisse/{}.txt", val.datum);
//         let text = val.text.unwrap();

//         let until = text.find("Beginn:").unwrap();

//         // The Inhaltsverzeichnis roughly reaches until this string.
//         let content = &text[..until];

//         // Overrides the files if they already exist
//         let result = fs::write(name, content);
//         if let Err(e) = result {
//             panic!("could not write: {}", e);
//         }
//     }

// }

// A Plenarprotokoll page is seperated by a string that reads like this:
//      Deutscher Bundestag – 20. Wahlperiode – 68. Sitzung. Berlin, Dienstag, den 22. November 2022
//
// We can split the Plenarprotokoll via this string but need to generate it from what we know.
fn get_splitter(plenarprotokoll: &dip::PlenarprotokollText) -> String {
    let (wahlperiode, sitzung) = plenarprotokoll.dokumentnummer.split_once("/").unwrap();

    let date = NaiveDate::parse_from_str(&plenarprotokoll.datum, "%Y-%m-%d").unwrap();

    format!("Deutscher Bundestag – {}. Wahlperiode – {}. Sitzung. Berlin, {}, den {}. {} {}",
        wahlperiode, 
        sitzung, 
        translate_day(date.weekday()), 
        date.day(), 
        translate_month(date.month()),
        date.year()
    )
}

// Convert from english to german.
fn translate_day(day: Weekday) -> &'static str {
    match day {
        Weekday::Mon => "Montag",
        Weekday::Tue => "Dienstag",
        Weekday::Wed => "Mittwoch",
        Weekday::Thu => "Donnerstag",
        Weekday::Fri => "Freitag",
        Weekday::Sat => "Samstag",
        Weekday::Sun => "Sonntag"
    }
}

fn translate_month(month: u32) -> &'static str {
    match month {
        1 => "Januar",
        2 => "Februar",
        3 => "März",
        4 => "April",
        5 => "Mai",
        6 => "Juni",
        7 => "Juli",
        8 => "August",
        9 => "September",
        10 => "Oktober",
        11 => "November",
        12 => "Dezember",
        _ => panic!("could not translate month"),
    }
}
