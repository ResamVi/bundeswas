use dip;

use std::fs::{File, self};
use std::io::Write;
use std::env;
use std::io::ErrorKind;
use chrono::{self, NaiveDate, Datelike, Weekday};

fn main() {
    // download_plenarprotokolle(10);
    let bundestag = dip::new();

    let protokoll = bundestag.plenarprotokoll_text().skip(4).next().unwrap();
    let split = get_splitter(&protokoll);

    let text = protokoll.text.unwrap();

    let pages: Vec<&str> = text.split(&split).collect();
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

    for val in bundestag.plenarprotokoll_text().take(count) {
        println!("{}", val.wahlperiode);
    }
}

// Fills a folder "plenarproktolle" with .json files of Plenarprotokolle ready to be labelled in
// Label Studio.
fn download_plenarprotokolle(count: usize) {
    let bundestag = dip::new();

    let result = fs::create_dir("plenarprotokolle");
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

        let name = format!("plenarprotokolle/{}.json", val.datum);
        let text = val.text.unwrap();

        let text = format!("{{\"text\": \"{}\"}}", text);

        // Overrides the files if they already exist
        let result = fs::write(name, text);
        if let Err(e) = result {
            panic!("could not write: {}", e);
        }
    }
}

// Fills a folder with .txt files of Inhaltsverzeichnise (the first pages of a Plenarprotokoll).
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
