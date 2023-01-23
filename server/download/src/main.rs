use dip::{self, PlenarprotokollText, Plenarprotokoll};

use std::fs::{File, self};
use std::env;
use std::io::ErrorKind;

use clap::{ Parser, ArgGroup, Subcommand };
use spinners::{ Spinner, Spinners };

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

mod matcher;
mod formats;

/// A Command-Line Interface to the DIP (Dokumentations- und Informationssystem für
/// Parlamentsmaterialien) to download Plenarprotokolle.
/// 
// Examples:
//      download --format json --typ plenarprotokoll 5449
//      download --format txt --typ sitzung 8
//      download --count 10 --format txt --typ sitzung 8
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Typ {
    Plenarprotokoll,
    Sitzung,
    Datum,
}

/// Subcommands
/// 
/// Examples:
///     download list
///     download list 10 (TODO)
#[derive(Subcommand)]
enum Commands {
    /// Download one or multiple Plenarprotokoll given an identifier.
    Plenarprotokoll {
        /// What Plenarprotokoll to find.
        input: Option<String>,

        /// Downloads the selected Plenarprotokoll and all n after it.
        #[arg(short, long)]
        count: Option<usize>,

        /// Via what parameter the Plenarprotokoll should be identified.
        #[clap(value_enum)]
        #[arg(short, long, default_value_t=Typ::Plenarprotokoll)]
        typ: Typ,

        /// Output File format
        #[clap(value_enum)]
        #[arg(short, long, default_value_t=formats::Format::Stdout)]
        format: formats::Format,
    },

    /// List what Plenarprotokolle are currently available.
    List {
        /// lists test values
        #[arg(default_value_t=10)]
        count: usize,
    },
}


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

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Plenarprotokoll{ input, count, typ, format }) => {
            // Use latest plenarprotokoll if no ID was specified
            let default = &dip::new().plenarprotokolle().take(1).next().unwrap().id;
            let input = match input.as_ref() {
                Some(input) => input,
                None => default,
            };

            match typ {
                Typ::Plenarprotokoll => download_by_id(count.unwrap_or(1), &matcher::Id(input.to_string()), format),
                Typ::Sitzung => todo!(),
                Typ::Datum => todo!(),
            };
        },
        Some(Commands::List { count }) => {
            let bundestag = dip::new();

            let mut table = Table::new();
            table.add_row(row!["ID", "DOKUMENTNUMMER", "URL", "DATUM", "HAS TEXT"]);
            bundestag
                .plenarprotokolle()
                .take(*count)
                .for_each(|p: Plenarprotokoll| {
                    table.add_row(row![p.id, p.dokumentnummer, p.fundstelle.pdf_url, p.datum, p.vorgangsbezug_anzahl > 0]);
                });

            println!("{}", table);
        }
        None => {}
    }

}

// Fills a folder "plenarprotokolle" with .json files of Plenarprotokolle ready to be labelled in Label Studio.
fn download_by_id(desired_count: usize, rule: &dyn matcher::Matcher, format: &dyn formats::Downloader) {
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

