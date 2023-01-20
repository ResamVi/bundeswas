/// A Downloader implements the strategy how to download a plenarprotokoll in a specific format.  
trait Downloader {
    fn download(&self, plenarprotokoll: dip::PlenarprotokollText);
}

/// Depending on the format download as .json or .txt
impl Downloader for Format {
    fn download(&self, plenarprotokoll: dip::PlenarprotokollText) {
        // TODO: If we were to support more than those formats RIP. Need to extract them to
        // separate functions.

        // Prepare path where to download.
        let path = match self {
            Format::Json => format!("plenarprotokolle/{}.json", plenarprotokoll.datum),
            Format::Txt => format!("plenarprotokolle/{}.txt", plenarprotokoll.datum),
            _ => format!("plenarprotokolle/{}.txt", plenarprotokoll.datum),
        };

        let content = match self {
            Format::Json => format!("{{\"text\": \"{}\"}}", plenarprotokoll.text.unwrap()), // Wrap in brackets that can be parsed by Label Studio.
            Format::Txt => plenarprotokoll.text.unwrap(), // Raw text dump.
        };

        // Override the files if they already exist.
        let result = fs::write(path, content);
        if let Err(e) = result {
            panic!("could not write: {}", e);
        }

        println!("Downloaded Plenarprotokoll '{}'", plenarprotokoll.id);
    }
}
