use std::fs;

use chrono::{self, NaiveDate, Datelike, Weekday};
use serde::{Serialize, Deserialize};


#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Format {
    Json,
    Txt,
    Stdout,
}

/// Implement the Downloader trait to specifiy the strategy how to download a plenarprotokoll in a specific format.  
pub trait Downloader {
    fn download(&self, plenarprotokoll: dip::PlenarprotokollText);
}

/// Depending on the format download as .json or .txt
impl Downloader for Format {
    fn download(&self, plenarprotokoll: dip::PlenarprotokollText) {
        // TODO: Allow specifying path.

        // Prepare path where to download.
        let path = match self {
            Format::Json => download_json(&plenarprotokoll),
            Format::Txt => download_txt(&plenarprotokoll),
            Format::Stdout => download_stdout(&plenarprotokoll),
            _ => panic!("format not found"),
        };

        println!("Downloaded Plenarprotokoll '{}'", plenarprotokoll.id);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    text: String,
}

/// Strategy for JSON. Ready to be used in Label Studio.
///
/// Sample format for Label Studio must look like this:
/// [
///    {
///       "id":1,
///       "data":{
///          "my_text":"Opossums like to be aloft in trees."
///       }
///    },
///    {
///       "id":2,
///       "data":{
///          "my_text":"Opossums are opportunistic."
///       }
///    },
///    {
///       "id":3,
///       "data":{
///          "my_text":"Opossums like to forage for food."
///       }
///    }
/// ]
/// See: https://labelstud.io/guide/tasks.html#Example-JSON-format
fn download_json(plenarprotokoll: &dip::PlenarprotokollText) {
    let path = format!("plenarprotokolle/{}.json", plenarprotokoll.datum);

    // Convert to JSON ready for Label Studio.
    let content: Vec<Data> = Vec::new();

    // Split the Plenarprotokoll into pages.
    let splitter = get_splitter(&plenarprotokoll);
    let pages: Vec<Data> = plenarprotokoll.text.as_ref().unwrap().split(&splitter).map(|s: &str|{
        Data { text: s.to_string() }
    }).collect();

    // Serialize to JSON
    let serialized = serde_json::to_string(&pages).unwrap();

    let result = fs::write(path, serialized);
    if let Err(e) = result {
        panic!("could not write: {}", e);
    }
}

/// Strategy for Text files. Just a raw dump.
fn download_txt(plenarprotokoll: &dip::PlenarprotokollText) {
    let path = format!("plenarprotokolle/{}.txt", plenarprotokoll.datum);
    let content = plenarprotokoll.text.as_ref().unwrap(); // Raw text dump.
    let result = fs::write(path, content);
    if let Err(e) = result {
        panic!("could not write: {}", e);
    }
}

/// Strategy for Stdout. Prints to console.
fn download_stdout(plenarprotokoll: &dip::PlenarprotokollText) {
    println!("{}", plenarprotokoll.text.as_ref().unwrap());
}

/// A Plenarprotokoll page is seperated by a string that reads like this:
///      Deutscher Bundestag – 20. Wahlperiode – 68. Sitzung. Berlin, Dienstag, den 22. November 2022
///
/// We can split the Plenarprotokoll via this string but need to generate it from what we know.
/// 
/// Note: "I am on page 27 in the PDF but I have to index to plenarprotokoll[25] or plenarprotokoll[24]. Why?"
///
/// Sometimes the Inhaltsverzeichnis is padded with an extra empty page (https://dserver.bundestag.de/btp/20/20079.pdf) adding +1 to the pdf.
/// Also for some reason between page 1 and 2 in the pdf, the splitter text (Deutscher Bundestag - ...) is missing which fucks over the logic when splitting the full Protokoll into pages.
/// Finally the index to the vector is 0-indexed.
fn get_splitter(plenarprotokoll: &dip::PlenarprotokollText) -> String {
    // "20/7" => 20th Wahlperiode and Sitzung Nr. 7
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

/// Convert from English to German.
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

/// Translate month from Number to written German.
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
