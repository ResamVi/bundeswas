use dip::{self, PlenarprotokollText};

use std::fs::{File, self};
use std::env;
use std::io::ErrorKind;

use clap::{ Parser, ArgGroup };
use spinners::{Spinner, Spinners};

mod matcher;
mod downloader;

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
    format: downloader::Format,
}

/// 
#[derive(clap::ValueEnum, Clone, Debug)]
enum Typ {
    Plenarprotokoll,
    Sitzung,
    Datum,
}


// download --format json --typ plenarprotokoll 5449
// download --format txt --typ sitzung 8
// download --count 10 --format txt --typ sitzung 8
fn main() {
    // let bundestag = dip::new();
    // let x = bundestag.plenarprotokoll_texte().skip(2).take(1).next().unwrap();
    // println!("{:?}", x.id);
    // println!("{:?}", x.text);

    // let splitter = get_splitter(&x);
    // println!("{:?}", s)

    // let text = x.text.unwrap();
    // let v: Vec<&str> = text.split(&splitter).collect();
    // println!("{:?}", v.get(25).unwrap());

    let args = Args::parse();

    match args.typ {
        Typ::Plenarprotokoll => download_by_id(args.count.unwrap_or(1), &matcher::Id(args.input), &args.format),
        Typ::Sitzung => todo!(),
        Typ::Datum => todo!(),
    };
}

// Fills a folder "plenarprotokolle" with .json files of Plenarprotokolle ready to be labelled in Label Studio.
fn download_by_id(desired_count: usize, rule: &dyn matcher::Matcher, format: &dyn downloader::Downloader) {
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

