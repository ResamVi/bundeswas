//! This crate implements calls to the DIP (Dokumentations- und Informationsystem für
//! Parlamentsmaterialien)
//!
//! See their documentation [here](https://dip.bundestag.de/documents/informationsblatt_zur_dip_api.pdf)
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub use model::Aktivitaet;
use model::Plenarprotokoll;
pub use model::PlenarprotokollText;
use model::Vorgang;

mod model;

// Notes on structs:
// We require 'Serialize' for 'to_string_pretty' to pretty print to console
// 'Deserialize' for HTTP response body -> rust struct mapping

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] // just for num_found -> numFound ...
struct Response<T> {
    // num_found: i32, // This field exists but isn't used 
    documents: Vec<T>,
    cursor: String,
}

/// Retrieve resources provided by the Deutscher Bundestag
/// what they coined "Dokumentations- und Informationssystem" (DIP).
///
/// We have permission for 25 parallel API calls at most.
pub struct DIP { }

pub fn new() -> DIP {
    DIP { }
}

impl DIP {
    /// Get a list of all Aktivitäten.
    /// Beschränke auf den Bundestag (kein Bundesrat).
    /// Beschränke auf die 20. Legislaturperiode (26.09.2021).
    pub fn aktivitaeten(&self) -> impl Iterator<Item = Aktivitaet> {
        PaginatedResource::new("https://search.dip.bundestag.de/api/v1/aktivitaet?f.zuordnung=BT&f.datum.start=2021-09-26")
    }

    pub fn plenarprotokolle(&self) -> impl Iterator<Item = Plenarprotokoll> {
        PaginatedResource::new("https://search.dip.bundestag.de/api/v1/plenarprotokoll?f.zuordnung=BT&f.datum.start=2021-09-26")
    }

    pub fn plenarprotokoll_texte(&self) -> impl Iterator<Item = PlenarprotokollText> {
        PaginatedResource::new("https://search.dip.bundestag.de/api/v1/plenarprotokoll-text?f.zuordnung=BT&f.datum.start=2021-09-26")
    }

    // pub fn plenarprotokoll_text(&self) -> PlenarprotokollText {
    //     let body: String = ureq::get("https://search.dip.bundestag.de/api/v1/plenarprotokoll-text/5449?f.datum.start=2021-10-26")
    //         .set("Authorization", "ApiKey GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp")
    //         .call()
    //         .expect("could not GET")
    //         .into_string()
    //         .expect("could not convert to to string");

    //     let mut response: PlenarprotokollText = serde_json::from_str(&body).unwrap();
    //     response
    // }

    pub fn vorgaenge(&self, plenarprotokoll_id: String) -> impl Iterator<Item = Vorgang> {
        PaginatedResource::new(format!("https://search.dip.bundestag.de/api/v1/vorgang?f.zuordnung=BT&f.datum.start=2021-09-26&f.plenarprotokoll={plenarprotokoll_id}").as_str())
    }
}

// Allows for consuming resources that are paginated and must 
// be retrieved in multiple GET requests ("Folgeanfragen").
//
// DIP specifies paginated results via a "cursor" 
// parameter that needs to be appended to the next request.
//
// TODO: Generic implementation
struct PaginatedResource<T> {
    // Static
    // client: Client,
    url: String,

    // Is mutated while we iterate
    cursor: String,
    entries: Vec<T>,
}

impl<T> PaginatedResource<T> {
    /// A PaginatedResource requires a http client to dispatch more GET requests
    /// when the current page has been exhausted.
    fn new(url: &str) -> PaginatedResource<T> {
        PaginatedResource {
            // client,
            url: url.to_string(),
            cursor: String::new(),
            entries: Vec::new(),
        }
    }
}

// TODO: core::iter weg?
impl<T: DeserializeOwned> core::iter::Iterator for PaginatedResource<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.entries.is_empty() {
            return self.entries.pop()
        }

        // Refill when empty
        let mut url = self.url.clone(); // TODO: needed?

        if !self.cursor.is_empty() {
            url.push_str("&cursor=");
            url.push_str(self.cursor.replace("+", "%2B").as_str());
        }

        // Temporary for debugging. 
        // So we can copy the link and paste into browser: 
        // Append apikey instead of as header
        url.push_str("&apikey=GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp");

        // println!("{}", url);

        // TODO: handle error
        let body: String = ureq::get(&url)
            // .set("Authorization", "ApiKey GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp")
            .call()
            .expect("could not GET")
            .into_string()
            .expect("could not convert to to string");

        let mut response: Response<Self::Item> = serde_json::from_str(&body).unwrap();
        response.documents.reverse();

        if self.cursor == response.cursor {
            return None
        } 

        self.cursor = response.cursor;
        self.entries = response.documents;
        self.entries.pop()
    }
}
