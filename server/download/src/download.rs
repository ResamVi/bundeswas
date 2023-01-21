use std::fs;

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

fn download_json(plenarprotokoll: &dip::PlenarprotokollText) {
    let path = format!("plenarprotokolle/{}.json", plenarprotokoll.datum);
    let content = format!("{{\"text\": \"{}\"}}", plenarprotokoll.text.as_ref().unwrap()); // Wrap in brackets that can be parsed by Label Studio.
    let result = fs::write(path, content);
    if let Err(e) = result {
        panic!("could not write: {}", e);
    }
}

fn download_txt(plenarprotokoll: &dip::PlenarprotokollText) {
    let path = format!("plenarprotokolle/{}.txt", plenarprotokoll.datum);
    let content = plenarprotokoll.text.as_ref().unwrap(); // Raw text dump.
    let result = fs::write(path, content);
    if let Err(e) = result {
        panic!("could not write: {}", e);
    }
}

fn download_stdout(plenarprotokoll: &dip::PlenarprotokollText) {
    println!("{}", plenarprotokoll.text.as_ref().unwrap());
}
